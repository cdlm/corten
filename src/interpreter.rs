use std::str::FromStr;
use std::collections::HashMap;
use std::rc::Rc;

type Stack<T> = Vec<T>;

#[derive(Default)]
pub struct Interpreter<'a, T> {
    data_stack: Stack<T>,
    vocabulary: HashMap<&'a str, Rc<Word<T>>>,
}

impl<'a, T> Interpreter<'a, T> where T: FromStr {

    fn push(&mut self, value: T) {
        self.data_stack.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.data_stack.pop()
    }

    fn parse(&mut self, input: &str) {
        for token in input.split_whitespace() {
            self.eval_token(token)
        }
    }

    fn eval_token(&mut self, token: &str) {
        if let Ok(literal) = token.parse::<T>() {
            self.push(literal);
        } else {
            if let Some(word) = self.lookup(token) {
                word.eval_within(self);
            }
        }
    }

    fn define_word(&mut self, name: &'a str, word: Rc<Word<T>>) {
        self.vocabulary.insert(name, word);
    }

    fn lookup(&self, token: &str) -> Option<Rc<Word<T>>> {
        // what's supposed to happen with undefined words?
        if let Some(word) = self.vocabulary.get(token) {
            Some(word.clone())
        } else { None }
    }
}

struct Word<T> {
    entry: Box<Fn(&Word<T>, &mut Interpreter<T>) -> ()>,
    definition: Vec<Rc<Word<T>>>, // should be Weak, but it's unstable
}

impl<T> Word<T> {
    fn eval_within(&self, i: &mut Interpreter<T>) {
      (self.entry)(self, i)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Word;
    use std::rc::Rc;

    fn fixture_plus() -> Rc<Word<i32>> {
        Rc::new(Word {
            entry: Box::new(|ref w, ref mut i| {
                if let (Some(b), Some(a)) = (i.pop(), i.pop()) {
                    i.push(a + b)
                } else { panic!("Stack underflow") }
            }),
            definition: vec![],
        })
    }

    fn fixture_dup() -> Rc<Word<i32>> {
        Rc::new(Word {
            entry: Box::new(|ref w, ref mut i| {
                if let Some(x) = i.pop() {
                    i.push(x);
                    i.push(x);
                } else { panic!("Stack underflow") }
            }),
            definition: vec![],
        })
    }

    #[test]
    fn user_word() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.define_word("+", fixture_plus());
        interpreter.define_word("dup", fixture_dup());
        let double = Rc::new(Word {
            entry: Box::new(|ref w, ref mut i| {
                for subw in &w.definition {
                    subw.eval_within(i);
                }
            }), // FIXME needs access to the word itself
            definition: vec![
                interpreter.lookup("dup").expect("duh").clone(),
                interpreter.lookup("+").expect("duh").clone(),
                ],
        });
        interpreter.define_word("double", double);
        interpreter.parse("2 double");
        assert_eq!(Some(4), interpreter.pop());
    }

    #[test]
    fn eval_interleaved_expressions() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.parse("2 3");
        interpreter.define_word("+", fixture_plus());
        interpreter.define_word("dup", fixture_dup());
        interpreter.parse("+ dup +");
        assert_eq!(Some(10), interpreter.pop())
    }

    #[test]
    fn eval_successive_words() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.define_word("+", fixture_plus());
        interpreter.define_word("dup", fixture_dup());
        interpreter.eval_token("2");
        interpreter.eval_token("dup");
        interpreter.eval_token("+");
        assert_eq!(Some(4), interpreter.pop())
    }

    #[test]
    fn register_word() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.define_word("+", fixture_plus());
        interpreter.push(51);
        interpreter.push(42);
        interpreter.eval_token("+");
        assert_eq!(Some(93), interpreter.pop())
    }

    #[test]
    fn plus_word() {
        let mut interpreter = Interpreter::<i32>::default();
        let plus = fixture_plus();
        interpreter.push(42);
        interpreter.push(51);
        plus.eval_within(&mut interpreter);
        assert_eq!(Some(93), interpreter.pop())
    }

    #[test]
    fn push_literal() {
        let mut interpreter = Interpreter::<i32>::default();
        interpreter.eval_token("42");
        assert_eq!(Some(42), interpreter.pop());
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
