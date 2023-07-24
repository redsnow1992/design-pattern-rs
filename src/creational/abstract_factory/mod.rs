pub mod app;
pub mod gui;
pub mod mac;
pub mod windows;

pub use gui::lib::*;

#[cfg(test)]
mod tests {
    use super::{
        app::{render_dynamic, render_static},
        mac::MacFactory,
        windows::WindowsFactory,
        GuiFactoryDynamic,
    };

    #[test]
    fn test_static() {
        let windows = true;
        if windows {
            render_static(WindowsFactory);
        } else {
            render_static(MacFactory);
        }
    }

    #[test]
    fn test_dynamic() {
        let windows = false;
        let factory: &dyn GuiFactoryDynamic = if windows {
            &WindowsFactory
        } else {
            &MacFactory
        };

        let button = factory.create_button();
        button.press();
        render_dynamic(factory);
    }
}
