use crate::cell::Cell;
use std::{marker::PhantomData, ptr::NonNull};

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        Rc {
            // SAFETY: Box does not give us a null pointer.
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // Drop the reference to the Box before we drop the Box to make sure we
            // don't have a danling reference when cleanup the Box in the next line.
            #[allow(clippy::drop_ref)]
            drop(inner);
            // SAFETY: we are the _only_ Rc left, sand we are being dropped.
            // therefore, after us, there will be no Rcs, and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other Rcs, so don't drop the Box
            inner.refcount.set(c - 1);
        }
    }
}

// #[cfg(test)]
// mod test {

//     #[test]
//     fn bad() {
//         let (y, x);
//         x = String::from("foo");
//         y = super::Rc::new(&x);
//     }
// }
