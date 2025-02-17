//! command line interpreting module based on [clap]
//!
//! Contains the text-command interface based on clap for the nebula command line tool.

use std::ffi::OsString;

use clap::{Parser, Subcommand, ValueEnum};

use color_eyre::{Section, eyre::Report};
use nebula_common::{
    NebulaCliState,
    api::{PackageStatus as ApiPackageStatus, Site as ApiSite},
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

    /// Tick rate, i.e. number of ticks per second
    #[cfg(feature = "tui")]
    #[arg(short, long, value_name = "FLOAT", default_value_t = 4.0)]
    pub tick_rate: f64,

    /// Frame rate, i.e. number of frames per second
    #[cfg(feature = "tui")]
    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]
    pub frame_rate: f64,
}

#[repr(u8)]
#[derive(ValueEnum, Debug, Clone, Default)]
pub enum Site {
    #[default]
    Local,

    Remote,
}

impl From<Site> for ApiSite {
    fn from(value: Site) -> Self {
        ApiSite::try_from(value as u8).unwrap()
    }
}

#[repr(u8)]
#[derive(ValueEnum, Debug, Clone, Default)]
pub enum PackageStatus {
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
    /// init the virtual environment in the given folder
    Init(ClapInitArgs),

    /// prints status information that depend on the registy and virtual environment
    Status(ClapStatusArgs),

    /// Installs a package
    Install(ClapInstallArgs),

    /// Updates a specific package or all packages
    Update(ClapUpdateArgs),

    /// Uninstall a specific package or all packages
    Uninstall(ClapUninstallArgs),

    /// search packages by specific criteria
    Search(ClapSearchArgs),

    /// list packages that fit specific criteria, e.g. installed, updatedable, etc.
    List(ClapListArgs),

    /// explore the details of a specific package
    Explore(ClapExploreArgs),

    /// Sync the local cache with the remote registry
    Sync(ClapSyncArgs),
    //Registry(RegistryArgs),
}

#[allow(dead_code)]
pub trait PostCommandHandler {
    fn on_init(&self) {}
    fn on_status(&self) {}
    fn on_install(&self) {}
    fn on_update(&self) {}
    fn on_uninstall(&self) {}
    fn on_search_packages(&self, _packages: Vec<PackageInfo>) {}
    fn on_list(&self, _packages: Vec<PackageInfo>) {}
    fn on_explore(&self) {}
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
}

impl PostCommandHandler for LegacyPostCommandHandler {
    fn on_search_packages(&self, packages: Vec<PackageInfo>) {
        if packages.is_empty() {
            println!("No package found.");
        } else {
            for (i, pi) in packages.iter().enumerate() {
                if i > 1 {
                    println!();
                }
                self.print_package_info(pi);
            }
        }
    }

    fn on_list(&self, packages: Vec<PackageInfo>) {
        if packages.is_empty() {
            println!("No packages found.");
        } else {
            for (i, pi) in packages.iter().enumerate() {
                if i > 1 {
                    println!();
                }
                self.print_package_info(pi);
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

                Command::Explore(explore_args) => explore(explore_args, state).await,
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
    fn test_site_conversion() {
        // Test conversion from Site::Local
        let local: ApiSite = Site::Local.into();
        // Since Site has #[repr(u8)] and Local is default variant (0)
        assert_eq!(local, ApiSite::Local);

        // Test conversion from Site::Remote
        let remote: ApiSite = Site::Remote.into();
        // Expect Remote to be represented as 1
        assert_eq!(remote, ApiSite::Remote);
    }

    #[test]
    fn test_package_status_conversion() {
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
