pub mod entity;
pub mod component;
pub mod storage;
pub mod system;

use std::collections::HashMap;
use std::any::{TypeId};
use std::cell::RefCell;
use std::marker::PhantomData;
use crate::component::Component;
use crate::storage::TrackedStorage;
use std::collections::hash_map::Entry;
use std::cell::Ref;
use std::cell::RefMut;
use std::ops::{Deref, DerefMut};
use mopa::Any;

mod __resource_mopafy_scope {
    use mopa::mopafy;
    use super::Resource;
    mopafy!(Resource);
}

pub struct World {
    resources: HashMap<ResourceId, RefCell<Box<dyn Resource>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn register<C: Component>(&mut self) {
        self.register_with_storage::<_, C>(Default::default);
    }

    pub fn register_with_storage<F, C>(&mut self, storage: F)
    where
        F: FnOnce() -> C::Storage,
        C: Component,
    {
        self.insert(RefCell::new(Box::new(storage())));
    }

    pub fn fetch<R: Resource>(&mut self) -> Fetch<R> {
        Fetch {
            inner: self.resources.get(&ResourceId::new::<R>()).unwrap().borrow(),
            phantom: PhantomData
        }
    }

    pub fn fetch_mut<R: Resource>(&mut self) -> FetchMut<R> {
        FetchMut {
            inner: self.resources.get(&ResourceId::new::<R>()).unwrap().borrow_mut(),
            phantom: PhantomData
        }
    }

    pub fn insert<R>(&mut self, resource: R)
    where 
        R: Resource,
    {
        self.insert_by_id(ResourceId::new::<R>(), resource)
    }

    pub fn insert_by_id<R>(&mut self, resource_id: ResourceId, resource: R)
    where 
        R: Resource,
    {
        resource_id.assert_type_id::<R>();
        self.resources.insert(resource_id, RefCell::new(Box::new(resource)));
    }

    pub fn remove<R>(&mut self, resource: R)
    where 
        R: Resource,
    {
        self.remove_by_id::<R>(ResourceId::new::<R>());
    }

    pub fn remove_by_id<R>(&mut self, resource_id: ResourceId)
    where 
        R: Resource, 
    {
        resource_id.assert_type_id::<R>();
        self.resources.remove(&resource_id);
    }

    pub fn entry<R>(&mut self) -> ResEntry<R> 
    where 
        R: Resource
    {
        create_entry(self.resources.entry(ResourceId::new::<R>()))
    }

    // pub fn fetch<T>(&self) -> &T 
    // where
    //     T: Resource
    // {
    //     self.try_fetch::<T>().unwrap()
    // }

    // pub fn try_fetch<T>(&self) -> Option<&T> 
    // where
    //     T: Resource
    // {
    //     let resource_type_id = ResourceId::new::<T>();
    //     if let Some(b) = self.resources.get(&resource_type_id).map(|b| b.borrow().as_any().downcast_ref::<T>()) {
    //         return b;
    //     }
    //     None
    // }

    // pub fn fetch_mut<T>(&mut self) -> &mut T 
    // where
    //     T: Resource
    // {
    //     self.try_fetch_mut::<T>().unwrap()
    // }

    // pub fn try_fetch_mut<T>(&mut self) -> Option<&mut T> 
    // where
    //     T: Resource
    // {
    //     let resource_type_id = ResourceId::new::<T>();
    //     if let Some(b) = self.resources.get_mut(&resource_type_id).map(|b| b.borrow_mut().as_any_mut().downcast_mut::<T>()) {
    //         return b;
    //     }
    //     None
    // }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId {
    type_id: TypeId,
}

impl ResourceId {
    pub fn new<T: Resource>() -> Self {
        Self {
            type_id: TypeId::of::<T>()
        }
    }

    pub fn assert_type_id<T: Resource>(&self) {
        let test_id = ResourceId::new::<T>();
        assert_eq!(test_id.type_id, self.type_id);
    }

    pub fn check_type_id<T: Resource>(&self) -> bool {
        let test_id = ResourceId::new::<T>();
        return test_id.type_id == self.type_id;
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentId {
    type_id: TypeId,
}

impl ComponentId {
    pub fn new<T: Component>() -> Self {
        Self {
            type_id: TypeId::of::<T>()
        }
    }

    pub fn assert_type_id<T: Component>(&self) {
        let test_id = ComponentId::new::<T>();
        assert_eq!(test_id.type_id, self.type_id);
    }

    pub fn check_type_id<T: Component>(&self) -> bool {
        let test_id = ComponentId::new::<T>();
        return test_id.type_id == self.type_id;
    }
}

pub trait Resource: 'static + Any {}

impl<T> Resource for T where T: Any {}

pub struct ResEntry<'a, T: 'a> {
    inner: Entry<'a, ResourceId, RefCell<Box<dyn Resource>>>,
    phantom: PhantomData<T>,
}

pub fn create_entry<T>(entry: Entry<ResourceId, RefCell<Box<dyn Resource>>>) -> ResEntry<T> {
    ResEntry {
        inner: entry,
        phantom: PhantomData,
    }
}

impl<'a, T> ResEntry<'a, T> 
where
    T: Resource + 'a
{
    // pub fn or_insert(self, v: T) -> &'a mut Box<T> {
    //     self.or_insert_with(move || v)
    // }

    // pub fn or_insert_with<F>(self, f: F) -> &'a mut Box<T>
    // where
    //     F: FnOnce() -> T,
    // {
    //     let value = self
    //         .inner
    //         .or_insert_with(move || RefCell::new(Box::new(f())));
    //     let inner = RefMut::map(value.borrow_mut(), Box::as_mut);

    //     inner
    // }
}

pub struct Fetch<'a, T: 'a> {
    pub inner: Ref<'a, dyn Resource>,
    pub phantom: PhantomData<&'a T>,
}

impl<'a, T> Deref for Fetch<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> Deref for FetchMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> DerefMut for FetchMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.inner.downcast_mut_unchecked() }
    }
}

pub struct FetchMut<'a, T: 'a> {
    pub inner: RefMut<'a, dyn Resource>,
    pub phantom: PhantomData<&'a T>,
}