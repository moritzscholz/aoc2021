#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(xy: (i32, i32)) -> Self {
        Self { x: xy.0, y: xy.1 }
    }
}

impl From<(i32, i32)> for Velocity {
    fn from(vxvy: (i32, i32)) -> Self {
        Self {
            x: vxvy.0,
            y: vxvy.1,
        }
    }
}
