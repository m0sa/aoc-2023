use std::env;
use std::fs;
use std::path::Path;

pub fn resource(project_relative_path: &str) -> String {
    let current_dir = match env::current_dir() {
        Result::Ok(s) => match s.to_str() {
            Some(val) => Result::Ok(val.to_string()),
            None => Result::Err(env::VarError::NotPresent),
        },
        Result::Err(_) => Result::Err(env::VarError::NotPresent),
    };

    let cargo_dir = env::var("CARGO_MANIFEST_DIR");
    let root_dir = cargo_dir
        .or(current_dir)
        .expect("cannot resolve root resource directory");
    let final_path = Path::new(&root_dir).join(&project_relative_path);
    return fs::read_to_string(final_path)
        .expect(&format!("Could not read '{project_relative_path}'"));
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point2D {
    fn from(e: (i32, i32)) -> Point2D {
        return Point2D { x: e.0, y: e.1 };
    }
}
