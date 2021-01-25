use im::Vector;
use std::iter::Iterator;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StackPos {
    pub vector_index: usize,
    pub debruijn_index: usize,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
/// This stack is not very user friendly, it panics if you do something dumb.
pub struct Stack<T: Clone, F: Clone> {
    values: Vector<T>,
    frames: Vector<(F, usize)>,
}

impl<T: Clone, F: Clone> Stack<T, F> {

    /// Create an empty stack. This shouldn't allocate.
    pub fn new() -> Stack<T, F> {
        Stack { values: Vector::new(), frames: Vector::new() }
    }

    /// Start a new frame.
    pub fn enter(&mut self, frame: F) {
        self.frames.push_back((frame, self.values.len()));
    }

    /// Rewind to before the current frame.
    pub fn exit(&mut self) -> F {
        debug_assert!(!self.frames.is_empty());
        let (frame, index) = self.frames.pop_back().unwrap();
        debug_assert!(self.values.len() >= index);
        self.frames.truncate(index);
        frame
    }

    /// Rewind to before the current frame, returning the values in this frame.
    pub fn leave(&mut self) -> (F, Vector<T>) {
        debug_assert!(!self.frames.is_empty());
        let (frame, index) = self.frames.pop_back().unwrap();
        debug_assert!(index <= self.values.len());
        (frame, self.values.slice(index..))
    }

    /// Push a value in the current frame.
    pub fn push(&mut self, value: T) {
        debug_assert!(!self.frames.is_empty());
        self.values.push_back(value);
    }

    /// Pop a value in the current frame.
    pub fn pop(&mut self) -> T {
        debug_assert!(!self.frames.is_empty());
        let (_, index) = self.frames.back().unwrap();
        debug_assert!(*index < self.values.len());
        self.values.pop_back().unwrap()
    }

    /// Swaps the order of the top two items on the stack.
    pub fn swap(&mut self) {
        let x = self.pop();
        let y = self.pop();
        self.push(x);
        self.push(y);
    }

    /// Rotates the top three items on the stack to the left: x y z -> y z x.
    pub fn rotl(&mut self) {
        let x = self.pop();
        let y = self.pop();
        let z = self.pop();
        self.push(y);
        self.push(z);
        self.push(x);
    }

    /// Rotates the top three items on the stack to the right: x y z -> z x y.
    pub fn rotr(&mut self) {
        let x = self.pop();
        let y = self.pop();
        let z = self.pop();
        self.push(z);
        self.push(x);
        self.push(y);
    }

    /// The total number of values on the stack
    pub fn count_values(&self) -> usize {
        self.values.len()
    }

    /// The total number of frames on the stack
    pub fn count_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn count_values_in_frame(&self) -> usize {
        self.values.len() - self.frames.back().map(|f| f.1).unwrap_or(0)
    }

    /// Looks up a value by predicate, returning a reference to it along with its position.
    pub fn find(&self, mut pred: impl FnMut(&T) -> bool) -> Option<(StackPos, &T)> {
        self.values.iter().enumerate().rev().enumerate()
            .find(|(_,(_,v))| pred(v))
            .map(|(dbi,(vector_index,v))| (StackPos { vector_index, debruijn_index: dbi + 1 }, v))
    }

    /// Looks up a value by predicate, returning a mutable reference to it along with its position.
    pub fn find_mut(&mut self, mut pred: impl FnMut(&T) -> bool) -> Option<(StackPos, &mut T)> {
        self.values.iter_mut().enumerate().rev().enumerate()
            .find(|(_,(_,v))| pred(v))
            .map(|(dbi,(vector_index,v))| (StackPos { vector_index, debruijn_index: dbi + 1 }, v))
    }

    /// Looks up value on the stack by its de Bruijn index.
    pub fn get(&self, debruijn: usize) -> &T {
        self.values.get(self.values.len() - debruijn).unwrap()
    }
}

