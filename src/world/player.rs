
use glam;
use super::super::utilities::Direction;

#[derive(Copy, Clone)]
pub enum PlayerAction {
    Idle,
    Walking,
    Running,
}

pub struct Player {
    pos: glam::IVec2,
    action: PlayerAction,
    direction: Direction,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: glam::IVec2::new(10,10),
            action: PlayerAction::Idle,
            direction: Direction::Down,
        }
    }
    pub fn current_action(&self) -> PlayerAction {return self.action}
    pub fn position(&self) -> glam::IVec2 {return self.pos}
    pub fn direction(&self) -> Direction {return self.direction}
}

