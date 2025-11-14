pub trait Ability {
    fn activate(&self);
    fn deactivate(&self);
    fn name(&self) -> &str;
}

pub struct BoostAbility;

impl Ability for BoostAbility {
    fn activate(&self) {}
    fn deactivate(&self) {}
    fn name(&self) -> &str { " Speed  Boost" }
}
