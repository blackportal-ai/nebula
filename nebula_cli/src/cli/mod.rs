//! command line interpreting module based on [clap]
//!
//! Contains the text-command interface based on clap for the nebula command line tool.

use std::ffi::OsString;

use clap::{Parser, Subcommand, ValueEnum};

use color_eyre::{Section, eyre::Report};
use nebula_common::{
    NebulaCliState,
    api::ListResult,
    datapackage::DataPackage,
    model::{PackageStatus as ApiPackageStatus, PackageType as ApiPackageType},
    nebula_proto::PackageInfo,
};

use crate::version;

mod bridge;
use bridge::*;

mod run;
pub use run::run_legacy_cmd;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[cfg(feature = "tui")]
    #[arg(long, default_value_t = false)]
    /// use a [ratatui] based terminal user interface instead of a simple cmd-tool
    pub tui: bool,

    #[arg(short, long, default_value_t = false)]
    /// start the cmd-tool in interactive mode, that allows typing multiple commands
    pub interactive: bool,

    /// use verbose output, only in non TUI mode.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[command(subcommand)]
    /// command that is executed
    pub cmd: Option<Command>,

    /// Tick rate, i.e. number of ticks per second in tui
    #[cfg(feature = "tui")]
    #[arg(short, long, value_name = "FLOAT", default_value_t = 4.0)]
    pub tick_rate: f64,

    /// Frame rate, i.e. number of frames per second in tui
    #[cfg(feature = "tui")]
    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]
    pub frame_rate: f64,
}

/// This enum is the same as [nebula_common::api::PackageStatus] but extends it with [clap::ValueEnum]
///
/// todo: check if there is a way with less duplicated code
#[repr(u8)]
#[derive(ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PackageStatus {
    All,

    NotInstalled,

    #[default]
    Installed,

    Updatedable,
}

impl From<PackageStatus> for ApiPackageStatus {
    fn from(value: PackageStatus) -> Self {
        ApiPackageStatus::try_from(value as u8).unwrap()
    }
}

/// This enum is the same as [nebula_common::api::PackageType] but extends it with [clap::ValueEnum]
///
/// todo: check if there is a way with less duplicated code
#[repr(u8)]
#[derive(ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PackageType {
    #[default]
    Both,

    Dataset,

    Model,
}

impl From<PackageType> for ApiPackageType {
    fn from(value: PackageType) -> Self {
        ApiPackageType::try_from(value as u8).unwrap()
    }
}

#[derive(Debug, Parser)]
pub struct CmdArgs {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(
    Debug,
    Subcommand,
    Clone,
    strum::Display,
    strum::EnumIter,
    strum::EnumDiscriminants,
    strum::EnumString
)]
#[strum_discriminants(name(CommandVariants))]
pub enum Command {
    /// init a virtual environment in the given folder (not yet)
    Init(ClapInitArgs),

    /// prints status information (not yet)
    Status(ClapStatusArgs),

    /// Installs a package (not yet)
    Install(ClapInstallArgs),

    /// Updates a specific package or all packages (not yet)
    Update(ClapUpdateArgs),

    /// Uninstall a specific package or all packages (not yet)
    Uninstall(ClapUninstallArgs),

    /// Searches packages by complex criteria (not yet)
    Search(ClapSearchArgs),

    /// List packages that fit simple criteria e.g.(non)-installed,
    List(ClapListArgs),

    /// Sync the local cache with the remote registry
    Sync(ClapSyncArgs),
}

#[allow(dead_code)]
pub trait PostCommandHandler {
    fn on_init(&self) {}
    fn on_status(&self) {}
    fn on_install(&self) {}
    fn on_update(&self) {}
    fn on_uninstall(&self) {}
    fn on_search_packages(&self, _packages: Vec<PackageInfo>) {}
    fn on_list(&self, _res: ListResult) {}
    fn on_sync(&self) {}
    fn on_cli_error(&self, _rep: &Report) {}
    fn on_clap_error(&self, _rep: &Report) {}
}

pub struct LegacyPostCommandHandler;

impl LegacyPostCommandHandler {
    fn print_package_info(&self, pi: &PackageInfo) {
        println!("{}", pi.name);
        let mut line = String::with_capacity(pi.name.len());
        for _ in 1..=pi.name.len() {
            line.push('-')
        }
        println!("{}", line);
        println!("Desc: {}", pi.description);
        println!("{} bytes download, {} bytes installed", pi.download_size, pi.installed_size);
    }

    fn print_datapackage_info(&self, dp: &DataPackage) {
        let name = dp.name.as_ref().unwrap();
        let version = dp.version.as_ref().unwrap();
        let id = dp.id.as_ref().unwrap();

        println!("{}-{} | {}", name, version, id);

        if let Some(desc) = dp.description.as_ref() {
            println!("Desc: {}", desc);
        }
        //println!("{} bytes download, {} bytes installed", dp., pi.installed_size);
    }
}

impl PostCommandHandler for LegacyPostCommandHandler {
    fn on_search_packages(&self, packages: Vec<PackageInfo>) {
        if packages.is_empty() {
            println!("No package found.");
        } else {
            for pi in packages.iter() {
                self.print_package_info(pi);
            }
        }
    }

    fn on_list(&self, res: ListResult) {
        let packages = &res.packages;
        if packages.is_empty() {
            println!("No packages found.");
        } else {
            for dp in packages.iter() {
                self.print_datapackage_info(dp);
            }
        }
    }

    fn on_clap_error(&self, rep: &Report) {
        println!("{:?}", rep)
    }

    fn on_cli_error(&self, rep: &Report) {
        println!("Something went wrong:");
        println!("{:?}", rep);
    }
}

pub async fn command_interpret<I, T, C>(
    itr: I,
    state: &mut NebulaCliState,
    pch: &mut C,
) -> Result<(), Report>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
    C: PostCommandHandler,
{
    match CmdArgs::try_parse_from(itr) {
        Ok(cli) => {
            let res = match cli.cmd {
                Command::Init(init_args) => init(init_args, state).await,
                Command::Status(status_args) => status(status_args, state).await,
                Command::Install(install_args) => install_package(install_args, state).await,
                Command::Update(update_args) => update_package(update_args, state).await,
                Command::Uninstall(uninstall_args) => {
                    uninstall_package(uninstall_args, state).await
                }
                Command::Search(search_args) => search_packages(search_args, state, pch).await,
                Command::List(list_args) => list_packages(list_args, state, pch).await,

                Command::Sync(sync_args) => sync(sync_args, state).await,
            };

            if let Err(err) = res {
                let err = err.with_note(|| "Probably nothing happened");
                pch.on_cli_error(&err);
            }
        }
        Err(err) => {
            let r: Report = err.into();
            pch.on_clap_error(&r);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_package_status_conversion() {
        let all: ApiPackageStatus = PackageStatus::All.into();
        // Expect enum value corresponding to 0
        assert_eq!(all, ApiPackageStatus::All);

        // Test conversion for PackageStatus::NotInstalled
        let not_installed: ApiPackageStatus = PackageStatus::NotInstalled.into();
        // Expect enum value corresponding to 0
        assert_eq!(not_installed, ApiPackageStatus::NotInstalled);

        // Test conversion for PackageStatus::Installed (default variant)
        let installed: ApiPackageStatus = PackageStatus::Installed.into();
        // Expect enum value corresponding to 1
        assert_eq!(installed, ApiPackageStatus::Installed);

        // Test conversion for PackageStatus::Updatedable
        let updatedable: ApiPackageStatus = PackageStatus::Updatedable.into();
        // Expect enum value corresponding to 2
        assert_eq!(updatedable, ApiPackageStatus::Updatedable);
    }

    #[test]
    fn test_clap_subcommand_parsing() {
        // Testing clap parsing for command subcommands.
        // Here we test that passing "Status" leads to the Status command variant.
        // The inner arguments for Status are not checked in detail.
        let args = vec!["test", "status"];
        let cli_args = CmdArgs::parse_from(args);

        match cli_args.cmd {
            Command::Status(_) => (), // Expected outcome.
            _ => panic!("Expected Status command variant, got a different one"),
        }
    }
}
