use crate::archetype::{Archetype, ArchetypeDescriptor, ArchetypeIndex, ArchetypeLayout};
use crate::component::{Component, ComponentSource};
use crate::entity::{Entity, EntityData, EntityMap};
use crate::insert::{EntityInserter, EntitySource};
use crate::storage::{Components, Storage};
use std::sync::atomic::AtomicU64;

#[derive(Default)]
pub struct World {
    archetypes: Vec<Archetype>,
    components: Components,
    entities: EntityMap,
    entity_counter: AtomicU64,
}

pub struct Entry<'a> {
    data: EntityData,
    world: &'a mut World,
}

impl World {
    pub fn create<T: ComponentSource>(&mut self, components: T) -> Entity {
        let arch_index = self.get_archetype_index::<T>();
        let archetype = &mut self.archetypes[arch_index.0 as usize];
        let entities = EntitySource::new(&self.entity_counter);
        let mut inserter = EntityInserter::new(self.components.edit(), archetype, entities);

        components.insert_components(&mut inserter);

        let (component, entities) = inserter.inserted();
        let replaced = self.entities.insert(entities, arch_index, component);
        let result = entities[0];

        for data in replaced {
            self.remove_data(data);
        }

        result
    }

    pub fn create_with_id<T: ComponentSource>(&mut self, id: Entity, components: T) {
        self.remove(id);

        let arch_index = self.get_archetype_index::<T>();
        let archetype = &mut self.archetypes[arch_index.0 as usize];
        let entities = EntitySource::from_id(id, &self.entity_counter);
        let mut inserter = EntityInserter::new(self.components.edit(), archetype, entities);

        components.insert_components(&mut inserter);

        let (component, entities) = inserter.inserted();

        self.entities.insert(entities, arch_index, component);
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.entities.contains(entity)
    }

    pub fn remove(&mut self, entity: Entity) -> bool {
        if let Some(data) = self.entities.remove(entity) {
            self.remove_data(data);
            true
        } else {
            false
        }
    }

    pub fn entry(&mut self, entity: Entity) -> Option<Entry> {
        self.entities
            .get(entity)
            .map(move |data| Entry { data, world: self })
    }

    pub fn components(&self) -> &Components {
        &self.components
    }

    pub fn archetypes(&self) -> &[Archetype] {
        &self.archetypes
    }

    fn remove_data(&mut self, data: EntityData) {
        let arch_index = data.archetype().0 as usize;
        let comp_index = data.component().0 as usize;
        let archetype = &mut self.archetypes[arch_index];
        let _ = archetype.entities.swap_remove(comp_index);

        for &ty in &archetype.layout.components {
            let storage = self.components.get_any_mut(ty).unwrap();

            storage.swap_remove(data.archetype(), data.component());
        }

        if comp_index < archetype.entities.len() {
            let swapped = archetype.entities[comp_index];
            self.entities.set(swapped, data);
        }
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

        for (&ty, &ctor) in archetype
            .layout
            .components
            .iter()
            .zip(&archetype.layout.constructors)
        {
            let storage = self.components.get_or_insert(ty, ctor);

            storage.register_archetype(index);
        }

        self.archetypes.push(archetype);
        index
    }
}

impl<'a> Entry<'a> {
    pub fn component<T: Component>(&self) -> Option<&T> {
        let component = self.data.component();
        let archetype = self.data.archetype();

        self.world
            .components
            .get::<T>()
            .and_then(|s| s.get(archetype))
            .and_then(|s| s.get(component))
    }

    pub fn component_mut<T: Component>(&mut self) -> Option<&mut T> {
        let component = self.data.component();
        let archetype = self.data.archetype();

        self.world
            .components
            .get_mut::<T>()
            .and_then(|s| s.get_mut(archetype))
            .and_then(|s| s.get_mut(component))
    }
}
