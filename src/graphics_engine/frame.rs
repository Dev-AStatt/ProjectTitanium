use super::super::utilities::Direction;

#[derive(PartialEq, Clone, Copy)]
pub enum FrameState {
    Full,
    Mid,
}
impl FrameState {
    pub fn flip(&self) -> FrameState {
        match self {
            FrameState::Full => {return FrameState::Mid}
            FrameState::Mid => {return FrameState::Full}
        }
    }
}


pub struct Frame {
    frame_state: FrameState,
    offset: glam::Vec2,
    direction: Option<Direction>,
    scale: i32,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            frame_state: FrameState::Full,
            offset: glam::Vec2::new(0.0,0.0),
            direction: None,
            scale: 3,
        }
    }

    pub fn update(
        self: &mut Self,
        tile_size: f32,
    ) {
        if let Some(d) = self.direction {
            self.move_offset(d, tile_size);
            self.frame_state = FrameState::Full;
            self.direction = None;
        }
    }
    pub fn move_frame(
        self: &mut Self, 
        new_direction: Direction,
        tile_size: f32,
    ) {
        self.frame_state = FrameState::Mid;
        self.direction = Some(new_direction);
        self.move_offset(new_direction, tile_size);
    }

    fn move_offset(
        self: &mut Self, 
        new_direction: Direction,
        tile_size: f32,
    ) {
        let mut adj: f32 = 0.5 * tile_size * self.scale_f32();
        match new_direction {
            Direction::Up => {self.inc_offset((0.0, adj))}
            Direction::Down => {self.inc_offset((0.0, -1.0 * adj))}
            Direction::Left => {self.inc_offset((adj, 0.0))}
            Direction::Right => {self.inc_offset((-1.0 * adj, 0.0))}
        }
    }
    
    fn inc_offset(self: &mut Self, adj: (f32,f32)) {
        self.offset.x += adj.0; 
        self.offset.y += adj.1; 
    }
    pub fn in_movement(&self) -> bool {self.frame_state == FrameState::Full}
    pub fn offset(&self) -> glam::Vec2 {self.offset}
    pub fn state(&self) -> FrameState {self.frame_state}
    pub fn scale_i32(&self) -> i32 {self.scale}
    pub fn scale_f32(&self) -> f32 {self.scale as f32}
    pub fn inc_scale(self: &mut Self, inc: i32) {self.scale += inc}
}

