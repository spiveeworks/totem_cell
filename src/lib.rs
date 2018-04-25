use std::cell;


pub struct Totem {
    _private: ()
}

impl Totem {
    pub unsafe fn new() -> Self {
        Totem {
            _private: ()
        }
    }
}

pub struct TotemCell<T> {
    pub inner: cell::UnsafeCell<T>
}

impl<T> TotemCell<T> {
    pub fn new(val: T) -> Self {
        let inner = cell::UnsafeCell::new(val);
        TotemCell { inner }
    }

    pub fn into_inner(self: Self) -> T {
        self.inner.into_inner()
    }

    pub fn borrow(self: &Self, _totem: &Totem) -> &T {
        unsafe {
            &*self.inner.get()
        }
    }

    pub fn borrow_mut(self: &Self, _totem: &mut Totem) -> &mut T {
        unsafe {
            &mut *self.inner.get()
        }
    }
}

