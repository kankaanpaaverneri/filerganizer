use std::{cell::RefCell, path::Path, rc::Rc};

mod ui;
use filerganizer::Core;

fn main() -> std::io::Result<()> {
    let core = Core::new(Path::new("/"))?;
    let core_ref = Rc::new(RefCell::new(core));
    ui::view(core_ref)?;
    Ok(())
}
