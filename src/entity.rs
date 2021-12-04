#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub(crate) u32, pub(crate) u32);

impl Entity {
    pub fn id(self) -> u32 {
        self.0
    }

    pub fn generation(self) -> u32 {
        self.1
    }
}