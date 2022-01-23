#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod models;
mod patterns;
#[cfg(feature = "regex")]
mod regex_patterns;
pub mod parser;
#[cfg(feature = "editor")]
pub mod editor;
