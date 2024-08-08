use slint::{SharedString, Weak};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle: Weak<AppWindow> = ui.as_weak();
    // on_ or invoke_ can be used on callbacks in the slint appwindow file in the main Window component
    ui.on_process_input({
        move |string: SharedString| {
            let ui: AppWindow = ui_handle.unwrap();
            //set_ and get_ can be used on the properties of the main window component in the appwindow file
            ui.set_output(string.trim().into());
        }
    });

    ui.run()
}
