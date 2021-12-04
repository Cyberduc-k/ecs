use crate::archetype::{Archetype, ArchetypeIndex};
use crate::component::{Component, ComponentIndex};
use crate::entity::Entity;
use crate::modify::{EditComponent, EditAnyComponent, EditComponents};
use std::any::TypeId;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct EntityInserter<'a> {
    edit: EditComponents<'a>,
    archetype: &'a mut Archetype,
    entities: EntitySource<'a>,
    entity_count: usize,
}

pub struct ComponentInserter<'a, T: Component> {
    edit: EditComponent<'a, T>,
    archetype: ArchetypeIndex,
}

pub struct AnyComponentInserter<'a> {
    edit: EditAnyComponent<'a>,
    archetype: ArchetypeIndex,
}

pub struct EntitySource<'a> {
    queue: Vec<Entity>,
    counter: &'a AtomicU32,
    generation: u32,
}

impl<'a> EntityInserter<'a> {
    pub fn new(edit: EditComponents<'a>, archetype: &'a mut Archetype, entities: EntitySource<'a>) -> Self {
        Self {
            entity_count: archetype.entities.len(),
            edit,
            archetype,
            entities,
        }
    }

    pub fn component<T: Component>(&mut self) -> ComponentInserter<'a, T> {
        ComponentInserter {
            edit: self.edit.get::<T>().unwrap(),
            archetype: self.archetype.index,
        }
    }

    pub fn any_component<T: Component>(&mut self, ty: TypeId) -> AnyComponentInserter<'a> {
        AnyComponentInserter {
            edit: self.edit.get_any(ty).unwrap(),
            archetype: self.archetype.index,
        }
    }

    pub fn finish_entity(&mut self) {
        let entity = self.entities.next();

        self.archetype.entities.push(entity);
    }

    pub fn inserted(&self) -> (ComponentIndex, &[Entity]) {
        (ComponentIndex(self.entity_count as u32), &self.archetype.entities[self.entity_count..])
    }
}

impl<'a, T: Component> ComponentInserter<'a, T> {
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, items: I) {
        self.edit.extend(self.archetype, items);
    }
}

impl<'a> AnyComponentInserter<'a> {
    pub unsafe fn extend_memcpy(&mut self, ptr: *const u8, len: usize) {
        self.edit.extend_memcpy(self.archetype, ptr, len);
    }
}

impl<'a> EntitySource<'a> {
    pub fn new(counter: &'a AtomicU32, generation: u32) -> Self {
        Self {
            queue: Vec::new(),
            counter,
            generation,
        }
    }

    pub fn next(&mut self) -> Entity {
        self.queue.pop().unwrap_or_else(|| {
            let id = self.counter.fetch_add(1, Ordering::SeqCst);

            Entity(id, self.generation)
        })
    }
}