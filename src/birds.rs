use crate::ability::Ability;

pub struct Bird {
    pub color: String,
    pub size: f32,
    pub weight: f32,
    pub speed: f32,
    pub ability: Option<Box<dyn Ability>>,
}

impl Bird {
    pub fn fly(&self) {}
    pub fn activate_ability(&self) {}
    pub fn collide(&self) {}
}
