use crate::component::Component;
use std::any::TypeId;
use std::marker::PhantomData;

pub trait LayoutFilter {
    fn matches(&self, components: &[TypeId]) -> bool;
}

pub struct AnyFilter;

pub struct ComponentFilter<T: Component>(PhantomData<T>);

impl LayoutFilter for AnyFilter {
    fn matches(&self, _: &[TypeId]) -> bool {
        true
    }
}

impl<T: Component> LayoutFilter for ComponentFilter<T> {
    fn matches(&self, components: &[TypeId]) -> bool {
        components.contains(&TypeId::of::<T>())
    }
}