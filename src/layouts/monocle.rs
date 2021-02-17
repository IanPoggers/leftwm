use crate::models::Window;
use crate::models::Workspace;

/// Layout which fills the workspace with the selected window
pub fn update(workspace: &Workspace, windows: &mut Vec<&mut &mut Window>) {
    let window_count = windows.len();
    if window_count == 0 {
        return;
    }

    let mut iter = windows.iter_mut();
    let w = iter.next().unwrap();
    w.set_height(workspace.height());
    w.set_width(workspace.width());
    w.set_x(workspace.x());
    w.set_y(workspace.y());

    for w in iter {
        w.set_visible(false);
    }
}
