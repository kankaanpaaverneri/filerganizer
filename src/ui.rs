use filerganizer::{Core, Message};
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use fltk::{
    app,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    tree,
    window::Window,
};

pub fn view(core: Rc<RefCell<Core>>) -> std::io::Result<()> {
    let app = app::App::default();

    let mut window = Window::new(100, 100, 500, 800, "Filerganizer");
    let mut tree = tree::Tree::default().with_size(500, 800).center_of_parent();
    tree.set_select_mode(tree::TreeSelect::Multi);
    tree.clear_visible_focus();
    tree.set_connector_style(tree::TreeConnectorStyle::None);

    tree.set_callback(move |mut t| {
        if let Some(item) = t.first_selected_item() {
            let path = t.item_pathname(&item).unwrap();
            let striped_path = &path[4..];
            core.borrow_mut()
                .update(Message::Append(PathBuf::from(striped_path)))
                .expect("Failed to update path");
            draw_tree(core.clone(), &mut t, Path::new(striped_path));
        }
    });
    window.make_resizable(true);
    window.end();
    window.show();
    app.run().unwrap();
    Ok(())
}

fn draw_tree(core: Rc<RefCell<Core>>, tree: &mut tree::Tree, target_path: &Path) {
    let mut current = &core.borrow_mut().root;
    // Filter root component
    let mut filtered_path = target_path
        .components()
        .filter(|component| match component {
            std::path::Component::RootDir => false,
            _ => true,
        });
    // Append directories
    while let Some(next) = filtered_path.next() {
        for directory in current.get_directories() {
            let file_path = directory.get_file_path();
            file_path.to_str().and_then(|name| tree.add(name));
            if let Some(last) = file_path.iter().last() {
                if last == next.as_os_str() {
                    current = directory;
                }
            }
        }
    }

    // Append last directories
    for directory in current.get_directories() {
        let file_path = directory.get_file_path();
        file_path.to_str().and_then(|name| tree.add(name));
    }
}
