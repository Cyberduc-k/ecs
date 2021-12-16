use crate::entity::Entity;
use crate::filter::AnyFilter;
use super::*;

pub struct EntityIter<'a> {
    entities: Option<std::slice::Iter<'a, Entity>>,
    archetypes: &'a [Archetype],
    index: std::slice::Iter<'a, ArchetypeIndex>,
}

impl IntoQuery for Entity {
    type Fetch = Self;
}

impl<'a> Fetch<'a> for Entity {
    type Item = Entity;
    type Iter = EntityIter<'a>;
    type Filter = AnyFilter;

    fn fetch(_: &'a Components, archetypes: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter {
        EntityIter {
            archetypes,
            index: index.iter(),
            entities: None,
        }
    }
}

impl Readonly for Entity {}

impl ComponentTypes for Entity {
    fn components() -> Vec<TypeId> {
        Vec::new()
    }
}

impl<'a> Iterator for EntityIter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        match self.entities {
            Some(ref mut entities) => match entities.next() {
                Some(entity) => Some(*entity),
                None => {
                    self.entities = None;
                    self.next()
                }
            },
            None => {
                self.entities = Some(self.archetypes[self.index.next()?.0 as usize].entities.iter());
                self.next()
            }
        }
    }
}