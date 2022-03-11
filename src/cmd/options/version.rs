use crate::cmd::{Control, Robusta};

pub fn version(_: &mut Robusta, _: &[String], idx: usize) -> (Control, usize) {
    eprintln!("robusta version \"{}\"", env!("CARGO_PKG_VERSION"));
    (Control::Exit, idx + 1)
}

pub fn show_version(_: &mut Robusta, _: &[String], idx: usize) -> (Control, usize) {
    eprintln!("robusta version \"{}\"", env!("CARGO_PKG_VERSION"));
    (Control::Continue, idx + 1)
}
