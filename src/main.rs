use rust_website_gen::{App, Route};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

mod blog;
mod markdown;
mod root;
mod template;

fn main() {
    App::new()
        .route("/", root::root())
        .route("/blog", blog::app())
        .route("/assets", ServeDir("assets".into()))
        .build("target/html")
        .unwrap()
}

struct ServeDir(PathBuf);

impl Route for ServeDir {
    fn build(&self, path: &std::path::Path) -> io::Result<()> {
        walk_dir(&self.0, &mut |entry_path| {
            let dest = path.join(entry_path.strip_prefix(&self.0).unwrap());
            _ = fs::create_dir_all(dest.parent().unwrap());
            _ = fs::copy(entry_path, dest);
        })
    }
}

fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&Path)) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_dir(&path, cb)?;
        } else {
            cb(&path)
        }
    }

    Ok(())
}
