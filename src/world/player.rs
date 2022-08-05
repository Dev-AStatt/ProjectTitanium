
use glam;

pub struct Player {
    pos: glam::IVec2,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: glam::IVec2::new(500,500),
        }
    }
}

