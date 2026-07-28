#[cfg(debug_assertions)]
mod reloadable;

#[cfg(not(debug_assertions))]
mod embedded;
