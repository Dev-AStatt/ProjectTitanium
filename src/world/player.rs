
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
            action: PlayerAction::Walking,
            direction: Direction::Down,
        }
    }

    pub fn movement_speed(&self) -> f32 {
        match self.action {
            PlayerAction::Idle => {return 1.0}
            PlayerAction::Walking => {return 1.0}
            PlayerAction::Running => {return 2.0}
        }
    }

    //Function will set player direction and incriment postion in 
    //that direction
    pub fn inc_position_in_direction(self: &mut Self, d: Direction) {
        self.set_direction(d);
        match d {
            Direction::Up => {self.pos.y -= 1}
            Direction::Down => {self.pos.y += 1}
            Direction::Left => {self.pos.x -= 1}
            Direction::Right => {self.pos.x += 1}
        }
    }


    pub fn current_action(&self) -> PlayerAction {return self.action}
    pub fn position(&self) -> glam::IVec2 {return self.pos}
    pub fn direction(&self) -> Direction {return self.direction}

    pub fn set_direction(&mut self, d: Direction) {self.direction = d}
}

