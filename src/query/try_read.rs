use super::*;
use crate::resource::TryRead;

pub enum TryReadIter<'a, T: Component> {
    Occupied {
        storage: &'a ArchetypeStorage<T>,
        components: Option<<T::Storage as Storage<'a, T>>::Iter>,
        archetypes: std::slice::Iter<'a, ArchetypeIndex>,
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
        match components.get::<T>() {
            Some(storage) => {
                TryReadIter::Occupied {
                    storage,
                    components: None,
                    archetypes: index.iter(),
                }
            },
            None => {
                TryReadIter::Empty {
                    len: index.iter()
                        .map(|i| archetypes[i.0 as usize].entities.len())
                        .sum(),
                }
            },
        }
    }
}

impl<T: Component> Readonly for TryRead<T> {
}

impl<T: Component> ComponentTypes for TryRead<T> {
    fn components() -> Vec<TypeId> {
        vec![TypeId::of::<T>()]
    }
}

impl<'a, T: Component> Iterator for TryReadIter<'a, T> {
    type Item = Option<&'a T>;

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
                | None => {
                    *components = storage.get(*archetypes.next()?).map(|s| s.iter());
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
