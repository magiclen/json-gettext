#[cfg(debug_assertions)]
mod reloadable;

#[cfg(not(debug_assertions))]
mod embedded;

#[cfg(not(debug_assertions))]
pub use embedded::*;
#[cfg(debug_assertions)]
pub use reloadable::*;
