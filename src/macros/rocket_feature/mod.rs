#[cfg(debug_assertions)]
mod debug;

#[cfg(not(debug_assertions))]
mod release;
