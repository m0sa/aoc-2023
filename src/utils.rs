use std::env;
use std::fs;
use std::ops;
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

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, PartialOrd, Ord)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Point2D {
        (x, y).into()
    }

    pub fn zero() -> Point2D {
        (0, 0).into()
    }

    pub fn manhattan_len(self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    pub fn manhattan_normalize(self) -> (usize, Point2D) {
        let m_len = self.manhattan_len();
        let len = m_len as i32;
        (m_len, (self.x / len, self.y / len).into())
    }

    pub fn clockwise(self) -> Point2D {
        (self.y * -1, self.x).into()
    }

    pub fn counter_clockwise(self) -> Point2D {
        (self.y, self.x * -1).into()
    }
}

impl From<(i32, i32)> for Point2D {
    fn from(e: (i32, i32)) -> Point2D {
        return Point2D { x: e.0, y: e.1 };
    }
}

impl_op_ex!(+ |a: &Point2D, b: &Point2D| -> Point2D {
    Point2D { x: a.x + b.x, y: a.y + b.y }
});
