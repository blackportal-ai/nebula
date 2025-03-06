//! Bridges [clap] based command-line interface to the nebula API implementation
//!
//! The module implements an [IPO model](https://en.wikipedia.org/wiki/IPO_model) and therefore calls
//! processing functions that are implemented in [nebula_common::api] module.
//!
//! The input uses a [From] implementation for data structure in this module that derive [Args]. Such that
//! a commandline argument structure, e.g. [ClapSearchArgs], can be converted to the corresponding nebula API
//! argument data structure, e.g. [SearchArgs].
//!
//! The process part in the running example calls [cli::search_package].
//!
//! The output path uses an implememtation of [PostCommandHandler] which is different for the legacy commaand line
//! and the [ratatui] based terminal user interface. See [crate::cli::LegacyPostCommandHandler] and [crate::tui::app::RatatuiPostCommandHandler]

use clap::Args;

use color_eyre::{
    Section as _,
    eyre::{Report, eyre},
};
use nebula_common::{
    NebulaCliState,
    api::{self, ListArgs, SearchArgs, SyncArgs},
};

use super::{PackageStatus, PackageType, PostCommandHandler};

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapInitArgs {}

pub async fn init(_args: ClapInitArgs, _state: &mut NebulaCliState) -> Result<(), Report> {
    Err(eyre!("not implemented"))
}

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapInstallArgs {
    package_name: String,
}

pub async fn install_package(
    _args: ClapInstallArgs,
    _state: &mut NebulaCliState,
) -> Result<(), Report> {
    Err(eyre!("not implemented"))
}

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapListArgs {
    /// filter by status of packages: All (default), (not)-installed, updateable
    #[arg(short('s'), long, default_value = "all")]
    package_status: PackageStatus,

    /// filter by type of package: dataset, model, both(default)
    #[arg(short('t'), long, default_value = "both")]
    package_type: PackageType,
}

impl From<ClapListArgs> for ListArgs {
    fn from(value: ClapListArgs) -> Self {
        ListArgs {
            package_status: value.package_status.into(),
            package_type: value.package_type.into(),
        }
    }
}

pub async fn list_packages<E: PostCommandHandler>(
    args: ClapListArgs,
    state: &mut NebulaCliState,
    pch: &mut E,
) -> Result<(), Report> {
    let args = args.into();
    let list_result = api::list_packages(args, state).await?;

    pch.on_list(list_result);

    Ok(())
}

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapSearchArgs {
    query: String,
    // todo
}

impl From<ClapSearchArgs> for SearchArgs {
    fn from(value: ClapSearchArgs) -> Self {
        Self { query: value.query }
    }
}

pub async fn search_packages<E: PostCommandHandler>(
    args: ClapSearchArgs,
    state: &mut NebulaCliState,
    pch: &mut E,
) -> Result<(), Report> {
    // input:
    let args = args.into();

    // processing
    let packages = api::search_package(args, state).await.map_err(|err| {
        err.with_note(|| "contribute here: https://github.com/blackportal-ai/nebula")
    })?;
    pch.on_search_packages(packages);

    Ok(())
}

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapStatusArgs {}

pub async fn status(_args: ClapStatusArgs, _state: &mut NebulaCliState) -> Result<(), Report> {
    Err(eyre!("not implemented"))
}

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapSyncArgs {}

pub async fn sync(_args: ClapSyncArgs, state: &mut NebulaCliState) -> Result<(), Report> {
    let _reval = api::sync_packages(SyncArgs { last_sync: None }, state).await?;
    Ok(())
}

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapUninstallArgs {
    all: bool,

    package_name: String,
}

//---

pub async fn uninstall_package(
    _args: ClapUninstallArgs,
    _state: &mut NebulaCliState,
) -> Result<(), Report> {
    Err(eyre!("not implemented"))
}

//---

#[derive(Args, Debug, Clone, Default)]
pub struct ClapUpdateArgs {
    /// flag indicating if all packages shall be updated, ignored if packagename is given
    #[arg(short, long, default_value_t = true)]
    all: bool,

    /// name of the package that shall be updated
    #[arg(short, long)]
    package_name: Option<String>,
}

pub async fn update_package(
    _args: ClapUpdateArgs,
    _state: &mut NebulaCliState,
) -> Result<(), Report> {
    Err(eyre!("not implemented"))
}

//---
