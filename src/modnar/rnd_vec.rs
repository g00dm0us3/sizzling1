use std::cell::{Cell, RefCell};
use std::ops::{Deref, DerefMut};
use crate::modnar::Modnar;
use crate::util::Len;

pub struct RndVec<T> {
    base: Vec<T>,
    rnd: RefCell<Modnar>
}

impl<T> RndVec<T> {
    pub fn new() -> Self {
        Self {
            base: Vec::<T>::new(),
            rnd: RefCell::new(Modnar::new_rng())
        }
    }

    pub fn select(&self) -> &T {
        let rnd_rng = 0..=(self.base.len().wrapping_sub(1).max(0) as u64);
        let rnd_idx = self.rnd.borrow_mut().gen(rnd_rng) as usize;
        &self.base[rnd_idx]
    }

    pub fn select_mut(&mut self) -> &mut T {
        let rnd_rng = 0..=(self.base.len().wrapping_sub(1).max(0) as u64);
        let rnd_idx = self.rnd.into_inner().gen(rnd_rng) as usize;
        &mut self.base[rnd_idx]
    }
}

impl<T> Deref for RndVec<T> where T: Default {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T> DerefMut for RndVec<T> where T: Default {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}