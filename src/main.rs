use std::{cell::RefCell, path::Path, rc::Rc};

mod ui;
use filerganizer::Core;

fn main() -> std::io::Result<()> {
    let core = Core::new(Path::new("/"))?;
    ui::view(Rc::new(RefCell::new(core)))?;
    Ok(())
}
