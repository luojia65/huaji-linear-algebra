use core::{ops::{Add, Sub, Mul}};

pub struct Det<T> {
    inner: [T]
}

impl<T> Det<T> {
    pub fn new<A: AsRef<[T]>>(inner: &A) -> &Det<T> {
        let inner = inner.as_ref();
        assert!(inner.len().is_power_of_two(), "length of inner array must be power of two");
        unsafe { &*(inner as *const [T] as *const Det<T>) }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.inner
    }
}

impl<T> Det<T> {
    pub fn cofactor(&self, i: usize, j: usize) -> &Det<T> {
        unimplemented!()
    }
}

impl<T> Det<T> 
where T: Add + Sub + Mul {

    pub fn arithmatic_cofactor(&self, i: usize, j: usize) -> &Det<T> {
        unimplemented!()
    }

    pub fn calc(&self) -> T {
        unimplemented!()
    }
}
