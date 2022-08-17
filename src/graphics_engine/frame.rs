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
    tile_size: f32,
    scale: i32,
    acc_shift: f32,
    just_flipped: bool,
}

impl Frame {
    pub fn new(tile_size: f32) -> Frame {
        Frame {
            frame_state: FrameState::Full,
            offset: glam::Vec2::new(0.0,0.0),
            direction: None,
            scale: 3,
            acc_shift: 0.0,
            tile_size,
            just_flipped: false,
        }
    }

    pub fn update(
        self: &mut Self,
    ) {
        self.just_flipped = false;
        if self.frame_state == FrameState::Mid 
        && self.acc_shift < self.tile_size {
            if let Some(d) = self.direction {
                self.move_offset(d);
                self.acc_shift += 1.0;
            }
        }
        //if the accumulated frame for movement exceeds 15 flip
        if self.acc_shift >= self.tile_size {
            self.frame_state = FrameState::Full;
            self.direction = None;
            self.acc_shift = 0.0;
            self.just_flipped = true;
        }
    }


    pub fn move_frame(
        self: &mut Self, 
        new_direction: Direction,
    ) {
        if self.frame_state == FrameState::Mid {return}
        self.frame_state = FrameState::Mid;
        self.direction = Some(new_direction);
        self.acc_shift = 0.0;
    }

    fn move_offset(
        self: &mut Self, 
        new_direction: Direction,
    ) {
        let adj: f32 = (self.tile_size * self.scale_f32()) / self.tile_size as f32;
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
    pub fn in_movement(&self) -> bool {self.frame_state == FrameState::Mid}
    pub fn offset(&self) -> glam::Vec2 {self.offset}
    pub fn state(&self) -> FrameState {self.frame_state}
    pub fn scale_i32(&self) -> i32 {self.scale}
    pub fn scale_f32(&self) -> f32 {self.scale as f32}
    pub fn just_flipped(&self) -> bool {return self.just_flipped}
    
    pub fn inc_scale(self: &mut Self, inc: i32) {self.scale += inc}
}

