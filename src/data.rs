pub trait Serialize {
    fn serialize(&self) -> String;
    fn deserialize(data: String) -> Self;
}

pub struct HeapData {
    pub creeps: Vec<HeapCreep>,
}

pub struct LocalPos {
    pub x: u8,
    pub y: u8,
}

pub struct HeapCreep {
    pub prevpos: LocalPos,
}

impl HeapData {
    pub fn init() -> Self {
        Self {
            creeps: Vec::<HeapCreep>::new(),
        }
    }
}
