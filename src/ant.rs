#[derive(PartialEq, Clone)]
pub struct Ant {
    pub x: u32,
    pub y: u32,
    pub cluster: u32,
}

impl Ant {
    pub fn new(x: u32, y: u32, cluster: u32) -> Ant {
        Ant { x, y, cluster: 0 }
    }
}
