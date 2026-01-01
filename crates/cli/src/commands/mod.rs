//! Command implementations for the GNN CLI

pub mod sweep;
pub mod parse;
pub mod train;
pub mod check;
pub mod compress;
pub mod generate;
pub mod info;
pub mod init;

// Re-export for convenience
pub use self::sweep::run as sweep;
pub use self::parse::run as parse;
pub use self::train::run as train;
pub use self::check::run as check;
pub use self::compress::run as compress;
pub use self::generate::run as generate;
pub use self::info::run as info;
pub use self::init::run as init;
