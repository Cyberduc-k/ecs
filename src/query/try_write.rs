use super::*;
use crate::filter::Any;
use crate::resource::TryWrite;

pub struct TryWriteIter<'a, T: Component> {
    storage: Option<&'a ArchetypeStorage<T>>,
    archetypes: &'a [Archetype],
    index: std::slice::Iter<'a, ArchetypeIndex>,
    state: TryWriteIterState<'a, T>,
}

pub enum TryWriteIterState<'a, T: Component> {
    Occupied {
        components: <T::Storage as Storage<'a, T>>::IterMut,
    },
    Empty {
        len: usize,
    },
}

impl<T: Component> IntoQuery for Option<&mut T> {
    type Fetch = TryWrite<T>;
}

impl<T: Component> IntoQuery for TryWrite<T> {
    type Fetch = Self;
}

impl<'a, T: Component> Fetch<'a> for TryWrite<T> {
    type Item = Option<&'a mut T>;
    type Iter = TryWriteIter<'a, T>;

    fn fetch(components: &'a Components, archetypes: &'a [Archetype], index: &'a [ArchetypeIndex]) -> Self::Iter {
        let mut iter = TryWriteIter {
            state: TryWriteIterState::Empty { len: 0 },
            storage: components.get::<T>(),
            index: index.iter(),
            archetypes,
        };

        let _ = iter.next_state();

        iter
    }
}

impl<T: Component> FetchFilter for TryWrite<T> {
    type Layout = Any;
}

impl<'a, T: Component> Iterator for TryWriteIter<'a, T> {
    type Item = Option<&'a mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            | TryWriteIterState::Occupied { components } => match components.next() {
                | Some(value) => Some(Some(value)),
                | None => {
                    self.next_state()?;
                    self.next()
                },
            },
            | TryWriteIterState::Empty { len } => {
                if *len > 0 {
                    *len -= 1;
                    Some(None)
                } else {
                    self.next_state()?;
                    self.next()
                }
            },
        }
    }
}

impl<'a, T: Component> TryWriteIter<'a, T> {
    fn next_state(&mut self) -> Option<()> {
        let archetype = *self.index.next()?;

        self.state = match self.storage {
            | Some(storage) => match unsafe { storage.get_mut_unchecked(archetype) } {
                | Some(components) => TryWriteIterState::Occupied {
                    components: components.iter_mut(),
                },
                | None => TryWriteIterState::Empty {
                    len: self.archetypes[archetype.0 as usize].entities.len(),
                },
            },
            | None => TryWriteIterState::Empty {
                len: self.archetypes[archetype.0 as usize].entities.len(),
            },
        };

        Some(())
    }
}
