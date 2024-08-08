use slint::{SharedString, Weak};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle: Weak<AppWindow> = ui.as_weak();

    ui.on_process_input({
        move |string: SharedString| {
            let ui: AppWindow = ui_handle.unwrap();
            ui.set_output(string.trim().into());
        }
    });

    ui.run()
}
