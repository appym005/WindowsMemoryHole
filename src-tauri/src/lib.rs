pub mod db;
pub mod errors;
pub mod ingest;
pub mod models;
pub mod open;
pub mod paths;

#[cfg(feature = "windows-app")]
pub mod hotkeys;
#[cfg(feature = "windows-app")]
pub mod windows;

#[cfg(test)]
mod tests;
