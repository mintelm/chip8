const STACK_SIZE: usize = 16;

pub struct Stack {
    data: [u16; STACK_SIZE],
    head: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            data: [0; STACK_SIZE],
            head: Default::default(),
        }
    }

    pub fn push(&mut self, data: u16) {
        debug_assert!(self.head < STACK_SIZE);

        self.data[self.head] = data;
        self.head += 1;
    }

    pub fn pop(&mut self) -> u16 {
        debug_assert!(self.head > 0);

        self.head -= 1;

        self.data[self.head]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let data = 9;
        let mut stack = Stack::new();

        stack.push(data);

        assert_eq!(stack.data[0], data);
        assert_eq!(stack.head, 1);
    }

    #[test]
    fn pop() {
        let data = 12;
        let mut stack = Stack::new();

        stack.push(data);

        assert_eq!(stack.pop(), data);
        assert_eq!(stack.head, 0);
    }

    #[test]
    #[should_panic]
    fn pop_empty() {
        let mut stack = Stack::new();

        stack.pop();
    }

    #[test]
    #[should_panic]
    fn push_full() {
        let data = 95;
        let mut stack = Stack::new();

        for _ in 0..STACK_SIZE + 1 {
            stack.push(data);
        }
    }
}
