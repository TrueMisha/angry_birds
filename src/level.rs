use crate::bird::Bird;
use crate::pig::Pig;
use crate::structure_block::StructureBlock;

pub struct Level {
    pub birds: Vec<Bird>,
    pub pigs: Vec<Pig>,
    pub blocks: Vec<StructureBlock>,
}

impl Level {
    pub fn start(&self) {}
    pub fn check_victory(&self) -> bool { false }
}
