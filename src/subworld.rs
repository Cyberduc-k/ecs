use crate::world::{StorageAccess, World};

pub struct SubWorld<'world> {
    pub(crate) world: &'world World,
}

pub trait AnyWorld {
    fn storage_access(&self) -> StorageAccess;
}

impl<'world> SubWorld<'world> {
    pub fn world(&self) -> &'world World {
        self.world
    }

    pub(crate) unsafe fn world_mut(&mut self) -> &'world mut World {
        &mut *(self.world as *const World as *mut World)
    }
}

impl<'world> AnyWorld for SubWorld<'world> {
    fn storage_access(&self) -> StorageAccess {
        self.world.storage_access()
    }
}
