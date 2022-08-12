
#[derive(PartialEq)]
pub enum StateType {
    Overworld,
    Battle,
    Menu,
    Bag,
}



pub struct GameState {
    on_start:       bool,
    state_type:     StateType,
    screen_size:    glam::Vec2,
}

impl GameState {

    pub fn new() -> Self {
       Self {
            on_start: true,
            state_type: StateType::Overworld,
            screen_size: glam::Vec2::new(0.0,0.0),
        }
    }

    //Getters and Setters
    pub fn on_start(&self) -> bool {return self.on_start}
    pub fn state_type(&self) -> &StateType {return &self.state_type}
    pub fn screen_size(&self) -> glam::Vec2 {return self.screen_size}

    pub fn set_on_start_false(self: &mut Self) {self.on_start = false}
    pub fn set_state_type(self: &mut Self, s: StateType) {self.state_type = s}
    pub fn set_screen_size(self: &mut Self, size: glam::Vec2) {self.screen_size = size}
}

