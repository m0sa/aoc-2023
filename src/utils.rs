use std::env;
use std::fs;
use std::path::Path;

pub fn resource(project_relative_path: &str) -> String {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let final_path = Path::new(&cargo_dir).join(&project_relative_path);
    return fs::read_to_string(final_path)
        .expect(&format!("Could not read '{project_relative_path}'"));
}
