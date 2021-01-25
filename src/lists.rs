use im::Vector;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// An immutable vector data type.
pub struct List<T: Clone + Eq, M: Clone + Eq> {
    pub(crate) values: Vector<T>,
    pub(crate) meta: M,
}

impl<T: Clone + Eq, M: Clone + Eq> List<T, M> {
    pub fn new(values: impl Into<Vector<T>>, meta: impl Into<M>) -> List<T, M> {
        List { values: values.into(), meta: meta.into() }
    }

    /// Return a copy with this value inserted at the front.
    pub fn cons(&self, value: T) -> List<T, M> {
        let mut new = self.clone();
        new.values.push_front(value);
        new
    }

    /// Return a copy without the front value, if any.
    pub fn uncons(&self) -> List<T, M> {
        let mut new = self.clone();
        new.values.pop_front();
        new
    }

    /// Return a copy with this value inserted at the back.
    pub fn push(&self, value: T) -> List<T, M> {
        let mut new = self.clone();
        new.values.push_back(value);
        new
    }

    /// Return a copy without the back value, if any.
    pub fn pop(&self) -> List<T, M> {
        let mut new = self.clone();
        new.values.pop_back();
        new
    }

    /// Return a ref to the front value, if any.
    pub fn front(&self) -> Option<&T> {
        self.values.front()
    }

    /// Return a ref to the back value, if any.
    pub fn back(&self) -> Option<&T> {
        self.values.last()
    }

}
