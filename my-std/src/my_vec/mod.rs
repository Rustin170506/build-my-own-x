use std::{
    alloc::{Allocator, Global},
    cmp, fmt, ops, ptr, slice,
};

use crate::raw_vec::RawVec;

/// A contiguous growable array.
///
/// # Examples
///
/// ```
/// use my_std::my_vec::MyVec;
///
/// let mut vec = MyVec::new();
/// vec.push(1);
/// vec.push(2);
/// assert_eq!(vec.pop(), Some(2));
/// assert_eq!(vec.len(), 1);
/// vec.push(3);
/// assert_eq!(vec.len(), 2);
/// assert_eq!(vec.pop(), Some(3));
/// assert_eq!(vec.pop(), Some(1));
/// assert_eq!(vec.pop(), None);
/// ```
pub struct MyVec<T, A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}

impl<T> MyVec<T> {
    /// Constructs a new, empty `MyVec<T>`.
    pub const fn new() -> Self {
        MyVec {
            buf: RawVec::NEW,
            len: 0,
        }
    }

    /// Constructs a new, empty `MyVec<T>` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, Global)
    }
}

impl Default for MyVec<u8> {
    fn default() -> Self {
        MyVec::new()
    }
}

impl<T, A: Allocator> MyVec<T, A> {
    /// Constructs a new, empty `MyVec<T>` with the specified capacity using a specified allocator.
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        MyVec {
            buf: RawVec::with_capacity_in(capacity, alloc),
            len: 0,
        }
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::my_vec::MyVec;
    ///
    /// let mut vec = MyVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec[0], 1);
    /// assert_eq!(vec[1], 2);
    /// ```
    pub fn push(&mut self, value: T) {
        // This will panic or abort if we would allocate more than `isize::MAX` bytes
        // or if the length increment would overflow for zero-sized types.
        if self.len == self.buf.capacity() {
            self.buf.reserve_for_push(self.len);
        }
        unsafe {
            let end = self.as_mut_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::my_vec::MyVec;
    ///
    /// let mut vec = MyVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// assert_eq!(vec.pop(), Some(2));
    /// assert_eq!(vec.pop(), Some(1));
    /// assert_eq!(vec.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.len -= 1;
                core::hint::assert_unchecked(self.len < self.buf.capacity());
                Some(ptr::read(self.as_mut_ptr().add(self.len)))
            }
        }
    }

    /// Returns a raw pointer to the vector's buffer, or a dangling raw pointer
    /// valid for zero sized reads if the vector didn't allocate.
    pub fn as_ptr(&self) -> *const T {
        self.buf.ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer, or a dangling raw pointer
    /// valid for zero sized reads if the vector didn't allocate.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.ptr()
    }

    /// Returns the number of elements the vector can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::my_vec::MyVec;
    ///
    /// let vec: MyVec<i32> = MyVec::with_capacity(10);
    /// assert_eq!(vec.capacity(), 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Forces the length of the vector to `new_len`.
    ///
    /// # Safety
    ///
    /// This is generally not safe, because it can lead to reading uninitialized memory.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.len = new_len;
    }

    /// Reserves capacity for at least `additional` more elements
    /// to be inserted in the given `MyVec<T>`.
    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(self.len, additional);
    }

    /// Inserts an element at position `index` within the vector,
    /// shifting all elements after it to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::my_vec::MyVec;
    ///
    /// let mut vec = MyVec::new();
    /// vec.push(1);
    /// vec.push(2);
    /// vec.insert(1, 3);
    /// assert_eq!(vec[0], 1);
    /// assert_eq!(vec[1], 3);
    /// assert_eq!(vec[2], 2);
    /// ```
    pub fn insert(&mut self, index: usize, element: T) {
        let len = self.len();

        unsafe {
            // infallible
            // The spot to put the new value
            {
                let p = self.as_mut_ptr().add(index);
                match index.cmp(&len) {
                    cmp::Ordering::Less => {
                        // Shift everything over to make space.
                        ptr::copy(p, p.add(1), len - index);
                    }
                    cmp::Ordering::Equal => {
                        // This is just a push
                    }
                    cmp::Ordering::Greater => {
                        panic!("insertion index (is {index}) should be <= len (is {len})");
                    }
                }
                // Write it in, overwriting the first copy of `index`th element.
                ptr::write(p, element);
            }
            self.set_len(len + 1);
        }
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::my_vec::MyVec;
    ///
    /// let mut v = MyVec::new();
    /// v.push(1);
    /// v.push(2);
    /// v.push(3);
    /// assert_eq!(v.remove(1), 2);
    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len();
        if index >= len {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        unsafe {
            // infallible
            let ret: T;
            {
                // the place we are taking from.
                let ptr = self.as_mut_ptr().add(index);
                // copy it out, unsafely having a copy of the value on
                // the stack and in the vector at the same time.
                ret = ptr::read(ptr);

                // Shift everything down to fill in that spot.
                ptr::copy(ptr.add(1), ptr, len - index - 1);
            }
            self.set_len(len - 1);
            ret
        }
    }
}

impl<T, A: Allocator> MyVec<T, A> {
    /// Use bubble sort to sort the vector.
    /// This is a simple sorting algorithm that repeatedly steps through the list,
    /// compares adjacent elements and swaps them if they are in the wrong order.
    /// The pass through the list is repeated until the list is sorted.
    /// This algorithm is not efficient for large lists.
    /// It has a worst-case and average-case complexity of O(n^2).
    /// It has a best-case complexity of O(n) when the list is already sorted.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::my_vec::MyVec;
    /// let mut v = MyVec::new();
    /// v.push(3);
    /// v.push(2);
    /// v.push(1);
    /// v.bubble_sort();
    /// assert_eq!(v[0], 1);
    /// assert_eq!(v[1], 2);
    /// assert_eq!(v[2], 3);
    /// ```
    pub fn bubble_sort(&mut self)
    where
        T: Ord,
    {
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 1..self.len() {
                if self[i] < self[i - 1] {
                    // swap
                    unsafe {
                        self.as_mut_ptr().add(i).swap(self.as_mut_ptr().add(i - 1));
                    }
                    sorted = false;
                }
            }
        }
    }

    /// Use insertion sort to sort the vector.
    /// This is a simple sorting algorithm that builds the final sorted list one item at a time.
    /// It has a worst-case and average-case complexity of O(n^2).
    /// It has a best-case complexity of O(n) when the list is already sorted.
    /// It is efficient for small lists.
    /// It is more efficient than bubble sort.
    /// It is stable, meaning that it preserves the order of equal elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_std::my_vec::MyVec;
    /// let mut v = MyVec::new();
    /// v.push(3);
    /// v.push(2);
    /// v.push(1);
    /// v.insertion_sort();
    /// assert_eq!(v[0], 1);
    /// assert_eq!(v[1], 2);
    /// assert_eq!(v[2], 3);
    /// ```
    pub fn insertion_sort(&mut self)
    where
        T: Ord,
    {
        for unsorted in 1..self.len() {
            let mut i = unsorted;
            while i > 0 && self[i - 1] > self[i] {
                // swap
                unsafe {
                    self.as_mut_ptr().add(i).swap(self.as_mut_ptr().add(i - 1));
                }
                i -= 1;
            }
        }
    }
}

impl<T, A: Allocator> ops::Deref for MyVec<T, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for MyVec<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut v = MyVec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_bubble_sort() {
        let mut v = MyVec::new();
        v.push(3);
        v.push(2);
        v.push(1);
        v.bubble_sort();
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = MyVec::new();
        v.push(3);
        v.push(2);
        v.push(1);
        v.insertion_sort();
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }
}
