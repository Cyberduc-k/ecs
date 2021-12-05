use crate::component::Component;
use crate::entity::Entity;
use crate::storage::{AnyArchetypeStorage, ArchetypeStorage};
use std::any::TypeId;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArchetypeIndex(pub(crate) u32);

#[derive(Debug, Clone)]
pub struct Archetype {
    pub index: ArchetypeIndex,
    pub layout: Arc<ArchetypeLayout>,
    pub entities: Vec<Entity>,
}

#[derive(Default, Debug, Clone)]
pub struct ArchetypeLayout {
    pub components: Vec<TypeId>,
    pub constructors: Vec<fn() -> Box<dyn AnyArchetypeStorage>>,
}

pub trait ArchetypeDescriptor {
    fn layout() -> ArchetypeLayout;
}

impl Archetype {
    pub fn new(index: ArchetypeIndex, layout: ArchetypeLayout) -> Self {
        Self {
            index,
            layout: Arc::new(layout),
            entities: Vec::new(),
        }
    }
}

impl ArchetypeLayout {
    pub fn add<T: Component>(&mut self) {
        let ty = TypeId::of::<T>();

        assert!(!self.components.contains(&ty));
        self.components.push(ty);
        self.constructors.push(ArchetypeStorage::<T>::any);
    }

    pub fn add_any(&mut self, ty: TypeId, ctor: fn() -> Box<dyn AnyArchetypeStorage>) {
        assert!(!self.components.contains(&ty));
        self.components.push(ty);
        self.constructors.push(ctor);
    }

    pub fn contains(&self, components: &[TypeId]) -> bool {
        components.iter().all(|t| self.components.contains(t))
    }
}

impl PartialEq for ArchetypeLayout {
    fn eq(&self, other: &Self) -> bool {
        if self.components.len() != other.components.len() {
            return false;
        }

        for ty in &self.components {
            if !other.components.contains(ty) {
                return false;
            }
        }

        true
    }
}

macro_rules! impl_archetype_descriptor {
    ($head:ident) => {
        impl_archetype_descriptor!(@impl $head);
    };

    ($head:ident, $($tail:ident),+) => {
        impl_archetype_descriptor!($($tail),+);
        impl_archetype_descriptor!(@impl $head, $($tail),+);
    };

    (@impl $($ty:ident),+) => {
        impl<$($ty: Component),+> ArchetypeDescriptor for ($($ty,)+) {
            fn layout() -> ArchetypeLayout {
                let mut layout = ArchetypeLayout::default();
                $(layout.add::<$ty>();)+
                layout
            }
        }
    };
}

impl_archetype_descriptor!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);
