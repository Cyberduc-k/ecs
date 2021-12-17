use super::*;
use crate::filter::Component as ComponentFilter;
use crate::resource::Read;

pub enum ReadIter<'a, T: Component> {
    Empty,
    Iter {
        storage: &'a ArchetypeStorage<T>,
        components: Option<<T::Storage as Storage<'a, T>>::Iter>,
        archetypes: std::slice::Iter<'a, ArchetypeIndex>,
    },
}

impl<T: Component> IntoQuery for &T {
    type Fetch = Read<T>;
}

impl<T: Component> IntoQuery for Read<T> {
    type Fetch = Self;
}

impl<'a, T: Component> Fetch<'a> for Read<T> {
    type Item = &'a T;
    type Iter = ReadIter<'a, T>;

    fn fetch(components: &'a Components, _: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter {
        match components.get::<T>() {
            | None => ReadIter::Empty,
            | Some(storage) => ReadIter::Iter {
                storage,
                components: None,
                archetypes: index.iter(),
            },
        }
    }
}

impl<T: Component> Readonly for Read<T> {
}

impl<T: Component> FetchFilter for Read<T> {
    type Layout = ComponentFilter<T>;
}

impl<'a, T: Component> Iterator for ReadIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            | Self::Empty => None,
            | Self::Iter {
                storage,
                components,
                archetypes,
            } => match components {
                | Some(comps) => match comps.next() {
                    | Some(comp) => Some(comp),
                    | None => {
                        *components = None;
                        self.next()
                    },
                },
                | None => {
                    *components = storage.get(*archetypes.next()?).map(|s| s.iter());
                    self.next()
                },
            },
        }
    }
}
