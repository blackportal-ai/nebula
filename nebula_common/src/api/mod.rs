//! implementation of an API for the nebula command line interface

mod explore;
mod install;
mod list;
mod search;
mod status;
mod sync;
mod uninstall;
mod update;

pub(crate) mod state;

#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Site {
    #[default]
    Local,

    Remote,
}

#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PackageStatus {
    NotInstalled,

    #[default]
    Installed,

    Updatedable,
}

pub use list::ListArgs;
pub use list::list_package;

use num_enum::TryFromPrimitive;
pub use search::SearchArgs;
pub use search::search_package;
