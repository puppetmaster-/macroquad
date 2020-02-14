use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use crate::get_context;

#[derive(Debug)]
pub struct GameObject<T: Any + 'static> {
    id: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Clone for GameObject<T> {
    fn clone(&self) -> Self {
        GameObject {
            id: self.id,
            _marker: std::marker::PhantomData,
        }
    }
}
impl<T> Copy for GameObject<T> {}

pub(crate) struct GameObjectStorage(Vec<Option<Box<dyn Any>>>);

impl GameObjectStorage {
    pub fn new() -> GameObjectStorage {
        GameObjectStorage(vec![])
    }
}

pub struct GameObjectRef<T: Any + 'static> {
    gameobject: Option<Box<T>>,
    id: usize,
}

impl<T: Any + 'static> Drop for GameObjectRef<T> {
    fn drop(&mut self) {
        let gameobject = self.gameobject.take().unwrap();
        let context = get_context();

        context.gameobjects.0[self.id] = Some(gameobject as Box<dyn Any>);
    }
}
impl<T: Any> Deref for GameObjectRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.gameobject.as_ref().unwrap()
    }
}

impl<T: Any> DerefMut for GameObjectRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.gameobject.as_mut().unwrap()
    }
}

impl<T: Any + 'static> GameObject<T> {
    pub fn new(data: T) -> GameObject<T> {
        let context = get_context();

        let id = context.gameobjects.0.len();
        context.gameobjects.0.push(Some(Box::new(data)));
        GameObject {
            id: id,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get(&self) -> GameObjectRef<T> {
        let context = get_context();

        let gameobject = context.gameobjects.0[self.id]
            .take()
            .unwrap_or_else(|| panic!());

        GameObjectRef {
            gameobject: Some(gameobject.downcast::<T>().unwrap_or_else(|_| panic!())),
            id: self.id,
        }
    }
}
