use super::*;
use crate::resource::TryWrite;

pub enum TryWriteIter<'a, T: Component> {
    Occupied {
        storage: &'a ArchetypeStorage<T>,
        components: Option<<T::Storage as Storage<'a, T>>::IterMut>,
        archetypes: std::slice::Iter<'a, ArchetypeIndex>,
    },
    Empty {
        len: usize,
    }
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
        match components.get::<T>() {
            Some(storage) => {
                TryWriteIter::Occupied {
                    storage,
                    components: None,
                    archetypes: index.iter(),
                }
            },
            None => {
                TryWriteIter::Empty {
                    len: index.iter()
                        .map(|i| archetypes[i.0 as usize].entities.len())
                        .sum(),
                }
            },
        }
    }
}

impl<T: Component> ComponentTypes for TryWrite<T> {
    fn components() -> Vec<TypeId> {
        vec![TypeId::of::<T>()]
    }
}

impl<'a, T: Component> Iterator for TryWriteIter<'a, T> {
    type Item = Option<&'a mut T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            | Self::Occupied {
                storage,
                components,
                archetypes,
            } => match components {
                | Some(comps) => match comps.next() {
                    | Some(comp) => Some(Some(comp)),
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
            | Self::Empty { len } => {
                if *len > 0 {
                    *len -= 1;
                    Some(None)
                } else {
                    None
                }
            }
        }
    }
}
