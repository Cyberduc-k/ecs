use super::*;
use crate::filter::Any;
use crate::resource::TryRead;

pub struct TryReadIter<'a, T: Component> {
    storage: Option<&'a ArchetypeStorage<T>>,
    archetypes: &'a [Archetype],
    index: std::slice::Iter<'a, ArchetypeIndex>,
    state: TryReadIterState<'a, T>,
}

pub enum TryReadIterState<'a, T: Component> {
    Occupied {
        components: <T::Storage as Storage<'a, T>>::Iter,
    },
    Empty {
        len: usize,
    }
}

impl<T: Component> IntoQuery for Option<&T> {
    type Fetch = TryRead<T>;
}

impl<T: Component> IntoQuery for TryRead<T> {
    type Fetch = Self;
}

impl<'a, T: Component> Fetch<'a> for TryRead<T> {
    type Item = Option<&'a T>;
    type Iter = TryReadIter<'a, T>;

    fn fetch(components: &'a Components, archetypes: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter {
        let mut iter = TryReadIter {
            state: TryReadIterState::Empty { len: 0 },
            storage: components.get::<T>(),
            index: index.iter(),
            archetypes,
        };

        let _ = iter.next_state();

        iter
    }
}

impl<T: Component> FetchFilter for TryRead<T> {
    type Layout = Any;
}

impl<'a, T: Component> Iterator for TryReadIter<'a, T> {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            TryReadIterState::Occupied { components } => match components.next() {
                Some(value) => Some(Some(value)),
                None => {
                    self.next_state()?;
                    self.next()
                }
            },
            TryReadIterState::Empty { len } => {
                if *len > 0 {
                    *len -= 1;
                    Some(None)
                } else {
                    self.next_state()?;
                    self.next()
                }
            }
        }
    }
}

impl<'a, T: Component> TryReadIter<'a, T> {
    fn next_state(&mut self) -> Option<()> {
        let archetype = *self.index.next()?;

        self.state = match self.storage {
            Some(storage) => match storage.get(archetype) {
                Some(components) => TryReadIterState::Occupied {
                    components: components.iter(),
                },
                None => TryReadIterState::Empty {
                    len: self.archetypes[archetype.0 as usize].entities.len(),
                },
            },
            None => TryReadIterState::Empty {
                len: self.archetypes[archetype.0 as usize].entities.len(),
            }
        };

        Some(())
    }
}
