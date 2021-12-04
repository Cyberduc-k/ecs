use crate::archetype::{Archetype, ArchetypeIndex, ArchetypeLayout, ArchetypeDescriptor};
use crate::component::{ComponentIndex, ComponentSource};
use crate::entity::Entity;
use crate::insert::{EntitySource, EntityInserter};
use crate::storage::Components;
use std::sync::atomic::AtomicU32;

#[derive(Default)]
pub struct World {
    archetypes: Vec<Archetype>,
    components: Components,
    entities: Vec<Entity>,
    entity_counter: AtomicU32,
    generation: u32,
}

impl World {
    pub fn create<T: ComponentSource>(&mut self, components: T) -> Entity {
        let archetype = self.get_archetype_index::<T>();
        let archetype = &mut self.archetypes[archetype.0 as usize];
        let edit = self.components.edit();
        let entities = EntitySource::new(&self.entity_counter, self.generation);
        let mut inserter = EntityInserter::new(edit, archetype, entities);

        components.insert_components(&mut inserter);

        let (component_index, entities) = inserter.inserted();

        for (i, &entity) in entities.iter().enumerate() {
            let component = ComponentIndex(component_index.0 + i as u32);
        }

        self.entities.extend_from_slice(entities);
        entities[0]
    }

    fn get_archetype_index<T: ArchetypeDescriptor>(&mut self) -> ArchetypeIndex {
        let layout = T::layout();

        match self.archetypes.iter().position(|a| &*a.layout == &layout) {
            Some(idx) => ArchetypeIndex(idx as u32),
            None => self.register_archetype(layout),
        }
    }

    fn register_archetype(&mut self, layout: ArchetypeLayout) -> ArchetypeIndex {
        let index = ArchetypeIndex(self.archetypes.len() as u32);
        let archetype = Archetype::new(index, layout);

        for (&ty, &ctor) in archetype.layout.components.iter().zip(&archetype.layout.constructors) {
            let storage = self.components.get_or_insert(ty, ctor);

            storage.register_archetype(index);
        }

        self.archetypes.push(archetype);
        index
    }
}