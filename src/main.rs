use hyprland::data::Workspace;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::shared::HyprDataActive;

slint::include_modules!();

/// Trim the workspace name to remove the workspace id
fn trim_workspace_name(input: &str) -> String {
    if let Some(index) = input.find(':') {
        input[(index + 2)..].to_string()
    } else {
        String::new()
    }
}

/// Format the workspace name to include the workspace id
fn format_workspace_name(id: i32, name: &str) -> String {
    match name.is_empty() {
        true => id.to_string(),
        false => format!("{}: {}", id, name),
    }
}

/// Rename the workspace with the new name
fn rename_workspace(id: i32, new_name: &str) {
    let message = DispatchType::RenameWorkspace(id, Some(new_name));

    Dispatch::call(message).expect("Failed to rename workspace");
}

/// Get the active workspace id and name
fn get_workspace() -> (i32, String) {
    let workspace = Workspace::get_active().expect("Failed to get active workspace");
    let id = workspace.id;
    let name = trim_workspace_name(&workspace.name);
    (id, name)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (id, name) = get_workspace();

    let ui = MainWindow::new()?;
    ui.invoke_set_name(name.into());

    // Set callback for when the user accepts the new name
    let ui_weak = ui.as_weak();
    ui.on_accepted(move |name| {
        let ui = ui_weak.unwrap();

        let new_name = format_workspace_name(id, &name);
        rename_workspace(id, &new_name);

        ui.hide().expect("Failed to close the window");
    });

    // Set callback for when the user rejects the new name
    let ui_weak = ui.as_weak();
    ui.on_rejected(move || {
        let ui = ui_weak.unwrap();
        ui.hide().expect("Failed to close the window");
    });

    ui.run()?;
    Ok(())
}
