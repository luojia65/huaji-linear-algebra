pub struct Det<T> {
    order: usize,
    arr: [T],
}

impl<T> Det<T> {
    pub fn new<A: AsRef<[T]> + ?Sized>(order: usize, arr: &A) -> &Det<T> {
        // 卧槽，order怎么办？？
        unsafe { &*(arr.as_ref() as *const [T] as *const Det<T>) }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.arr
    }
}

