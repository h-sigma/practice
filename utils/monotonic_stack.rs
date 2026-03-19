/// Example implementation of increasing monotonic stack to count how many elements are popped
/// Created first for the "Number of Visible People in a Queue" problem, but can be reused for similar problems.
struct MonotonicStack<T> {
    stack: Vec<T>,
}

impl<T: PartialOrd> MonotonicStack<T> {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Pushes a new value and returns the count of elements popped
    /// plus whether a "larger" boundary element remains.
    fn push_and_count_popped(&mut self, value: T) -> i32 {
        let mut count = 0;

        // 1. Count and remove all elements smaller than the new value
        while let Some(top) = self.stack.last() {
            if *top < value {
                count += 1;
                self.stack.pop();
            } else {
                break;
            }
        }

        // 2. If the stack isn't empty, the new value can "see" 
        // the one remaining element that is >= to it.
        if !self.stack.is_empty() {
            count += 1;
        }

        self.stack.push(value);
        count
    }
}