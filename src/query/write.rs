use super::*;
use crate::filter::Component as ComponentFilter;
use crate::resource::Write;

pub enum WriteIter<'a, T: Component> {
    Empty,
    Iter {
        storage: &'a ArchetypeStorage<T>,
        components: Option<<T::Storage as Storage<'a, T>>::IterMut>,
        archetypes: std::slice::Iter<'a, ArchetypeIndex>,
    },
}

impl<T: Component> IntoQuery for &mut T {
    type Fetch = Write<T>;
}

impl<T: Component> IntoQuery for Write<T> {
    type Fetch = Self;
}

impl<'a, T: Component> Fetch<'a> for Write<T> {
    type Item = &'a mut T;
    type Iter = WriteIter<'a, T>;

    fn fetch(components: &'a Components, _: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter {
        match components.get::<T>() {
            | None => WriteIter::Empty,
            | Some(storage) => WriteIter::Iter {
                storage,
                components: None,
                archetypes: index.iter(),
            },
        }
    }
}

impl<T: Component> FetchFilter for Write<T> {
    type Layout = ComponentFilter<T>;
}

impl<'a, T: Component> Iterator for WriteIter<'a, T> {
    type Item = &'a mut T;

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
                | None => unsafe {
                    *components = storage.get_mut_unchecked(*archetypes.next()?).map(|s| s.iter_mut());
                    self.next()
                },
            },
        }
    }
}
