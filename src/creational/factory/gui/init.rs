use crate::creational::factory::gui::{html_gui::HtmlDialog, windows_gui::WindowsDialog};

use super::gui::Dialog;

pub fn initialize() -> &'static dyn Dialog {
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating the HTML GUI --");
        &HtmlDialog
    }
}
