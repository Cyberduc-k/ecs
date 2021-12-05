use crate::archetype::ArchetypeIndex;
use crate::component::ComponentIndex;
use std::collections::HashSet;
use std::mem::MaybeUninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Entity(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityData(pub(crate) ArchetypeIndex, pub(crate) ComponentIndex);

#[derive(Default, Debug)]
pub struct EntityMap {
    entities: Vec<MaybeUninit<EntityData>>,
    free: HashSet<u64>,
}

impl EntityData {
    pub fn archetype(&self) -> ArchetypeIndex {
        self.0
    }

    pub fn component(&self) -> ComponentIndex {
        self.1
    }
}

impl EntityMap {
    pub fn contains(&self, entity: Entity) -> bool {
        if self.entities.len() <= entity.0 as usize {
            false
        } else {
            !self.free.contains(&entity.0)
        }
    }

    pub fn get(&self, entity: Entity) -> Option<EntityData> {
        if self.contains(entity) {
            Some(unsafe { self.entities[entity.0 as usize].assume_init() })
        } else {
            None
        }
    }

    pub fn insert(
        &mut self,
        ids: &[Entity],
        archetype: ArchetypeIndex,
        ComponentIndex(base): ComponentIndex,
    ) -> Vec<EntityData> {
        let mut removed = Vec::new();

        for (i, entity) in ids.iter().enumerate() {
            let idx = entity.0 as usize;
            let data = EntityData(archetype, ComponentIndex(base + i as u32));

            if idx >= self.entities.len() {
                self.free.extend(self.entities.len() as u64..idx as u64 + 1);
                self.entities.resize(idx + 1, MaybeUninit::uninit());
            }

            if self.free.remove(&entity.0) {
                self.entities[idx] = MaybeUninit::new(data);
            } else {
                removed.push(std::mem::replace(
                    unsafe { self.entities[idx].assume_init_mut() },
                    data,
                ));
            }
        }

        removed
    }

    pub fn set(&mut self, entity: Entity, data: EntityData) {
        self.insert(&[entity], data.archetype(), data.component());
    }

    pub fn remove(&mut self, entity: Entity) -> Option<EntityData> {
        if self.contains(entity) {
            Some(unsafe {
                std::mem::replace(&mut self.entities[entity.0 as usize], MaybeUninit::uninit())
                    .assume_init()
            })
        } else {
            None
        }
    }
}
