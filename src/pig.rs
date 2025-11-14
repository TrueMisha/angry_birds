pub struct Pig {
    pub health: f32,
    pub size: f32,
}

impl Pig {
    pub fn take_damage(&mut self, amount: f32) {}
    pub fn is_destroyed(&self) -> bool { false }
}
