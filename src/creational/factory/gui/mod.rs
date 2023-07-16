pub mod gui;
pub mod html_gui;
pub mod init;
pub mod windows_gui;

#[cfg(test)]
mod tests {
    use super::init::initialize;

    #[test]
    fn test_gui() {
        let dialog = initialize();
        dialog.render();
        dialog.refresh();
    }
}
