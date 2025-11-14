pub struct StructureBlock {
    pub material: String,
    pub health: f32,
}

impl StructureBlock {
    pub fn take_damage(&mut self, amount: f32) {}
    pub fn is_destroyed(&self) -> bool { false }
}
