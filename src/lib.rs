type Stack<T> = Vec<T>;

pub struct Interpreter<T> {
    data_stack: Stack<T>,
}

impl<T> Interpreter<T> {

    fn new() -> Interpreter<T> {
        Interpreter {
            data_stack: Stack::new(),
        }
    }

    fn push(&mut self, value: T) {
        self.data_stack.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.data_stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pops_nothing() {
        let mut interpreter = Interpreter::<i32>::new();
        assert_eq!(None, interpreter.pop())
    }

    #[test]
    fn pops_as_pushed() {
        let mut interpreter = Interpreter::<i32>::new();
        interpreter.push(42);
        assert_eq!(Some(42), interpreter.pop())
    }
}
