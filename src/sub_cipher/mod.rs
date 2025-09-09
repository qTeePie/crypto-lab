mod caesar;
pub mod menu; // <- NOT pub on purpose

pub use menu::run; // 😎 Export just ONE function: run() from menu.rs
