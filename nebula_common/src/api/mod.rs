//! implementation of an API for the nebula command line interface

mod install;
mod list;
mod search;
mod status;
mod sync;
mod uninstall;
mod update;

pub(crate) mod state;

pub use list::ListArgs;
pub use list::ListResult;
pub use list::list_packages;

pub use search::SearchArgs;
pub use search::search_package;

pub use sync::SyncArgs;
pub use sync::SyncRe;
pub use sync::sync_packages;
