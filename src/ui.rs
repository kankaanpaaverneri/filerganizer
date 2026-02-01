use filerganizer::{Core, Message};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::{Path, PathBuf, Component};

use fltk::{
    group,
    app,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    tree,
    window::Window,
    enums::Color,
    button::Button
    
};

pub fn view(core: Rc<RefCell<Core>>) -> std::io::Result<()> {
    let app = app::App::default();

    let mut window = Window::new(100, 100, 500, 800, "Filerganizer");
    let mut flex = group::Flex::new(0, 0, 500, 800, None);
    flex.set_type(group::FlexType::Row);
    let mut tree = tree::Tree::default().with_size(500, 800).center_of_parent();
    let mut move_selected_to_directory_button = Button::new(160, 180, 40, 80, "Move selected to directory");
    move_selected_to_directory_button.hide();
    tree.set_select_mode(tree::TreeSelect::Multi);
    tree.clear_visible_focus();
    tree.set_connector_style(tree::TreeConnectorStyle::None);
    let core_clone = Rc::clone(&core);
    tree.set_callback(move |mut t| {
        if let Some(item) = t.first_selected_item() {
            let pathname = t.item_pathname(&item).unwrap();
            let mut raw_path = Path::new(&pathname);
            let mut components = raw_path.components();
            components.next();
            let filtered_path = components.as_path().to_path_buf();
            
            let mut core_borrow = core_clone.borrow_mut();
            core_borrow
                .update(Message::Append(filtered_path.to_owned()))
                .expect("Failed to update path");
            drop(core_borrow);
            draw_tree(&core_clone, &mut t, filtered_path.as_path());
        }
        if let Some(items) = t.get_selected_items() {
            if items.len() == 0 {
                move_selected_to_directory_button.hide();
            } else {
                move_selected_to_directory_button.show();
            }
            flex.end();
        }
        
    });

    
    window.make_resizable(true);
    window.end();
    window.show();
    app.run().unwrap();
    Ok(())
}

fn draw_tree(core: &Rc<RefCell<Core>>, tree: &mut tree::Tree, target_path: &Path) {
    let mut current = &core.borrow_mut().root;
    // Filter root component
    let mut filtered_path = target_path
        .components()
        .filter(|component| match component {
            std::path::Component::RootDir => false,
            _ => true,
        });
    while let Some(next) = filtered_path.next() {
        // Append directories
        for directory in current.get_directories() {
            let file_path = directory.get_file_path();
            file_path.to_str().and_then(|name| tree.add(name));
            if let Some(last) = file_path.iter().last() {
                if last == next.as_os_str() {
                    current = directory;
                }
            }
        }
        // Append files
        for file_path in current.get_files() {
            file_path.to_str().and_then(|path| tree.add(path));
        }
    }

    // Append last directories
    for directory in current.get_directories() {
        let file_path = directory.get_file_path();
        file_path.to_str().and_then(|name| tree.add(name));
    }
}
