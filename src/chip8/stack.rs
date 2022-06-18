pub struct Stack {
    stack: Vec<usize>,
}

impl Stack {
    pub fn new() -> Self {
        let stack = vec![];

        Self { stack }
    }

    pub fn push(&mut self, value: usize) {
        self.stack.push(value);
    }

    pub fn ret(&mut self) -> usize {
        self.stack.pop().unwrap()
    }

    pub fn stack(&self) -> &[usize] {
        self.stack.as_ref()
    }
}
