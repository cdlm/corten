type Stack<T> = Vec<T>;

#[derive(Default)]
pub struct Interpreter<T> {
    data_stack: Stack<T>,
}

impl<T> Interpreter<T> {

    fn push(&mut self, value: T) {
        self.data_stack.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.data_stack.pop()
    }
}

struct Word<'a, T> {
    entry: &'a Fn(&mut Interpreter<T>) -> (),
}

impl<'a, T> Word<'a, T> {
    fn eval_within(&self, i: &mut Interpreter<T>) {
        (self.entry)(i)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Word;

    #[test]
    fn plus_word() {
        let mut interpreter = Interpreter::<i32>::default();
        let plus = Word {
            entry: &|i| {
                if let (Some(b), Some(a)) = (i.pop(), i.pop()) {
                    i.push(a + b)
                } else { panic!("Stack underflow") }
            }
        };
        interpreter.push(42);
        interpreter.push(51);
        plus.eval_within(&mut interpreter);
        assert_eq!(Some(93), interpreter.pop())
    }

    #[test]
    fn pops_nothing() {
        let mut interpreter = Interpreter::<i32>::default();
        assert_eq!(None, interpreter.pop())
    }

    #[test]
    fn pops_as_pushed() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.push(42);
        assert_eq!(Some(42), interpreter.pop());
        assert_eq!(None, interpreter.pop())
    }

    #[test]
    fn pops_in_reverse_order() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.push(42);
        interpreter.push(51);
        assert_eq!(Some(51), interpreter.pop());
        assert_eq!(Some(42), interpreter.pop());
        assert_eq!(None, interpreter.pop())
    }
}
