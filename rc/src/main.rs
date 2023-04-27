use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List<T> where T: Debug {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T> List<T> where T: Debug {
    fn head(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(x, _) => Some(x)
        }
    }

    fn tail(&self) -> Option<&Rc<List<T>>> {
        match self {
            List::Nil => None,
            List::Cons(_, xs) => Some(xs)
        }
    }

    fn last(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(x, xs) => match xs.deref() {
                List::Nil => Some(x),
                _ => xs.last()
            }
        }
    }
}

impl<T: Display + Debug> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            List::Nil => write!(f, "[]").unwrap(),
            List::Cons(x, xs_box) => {
                write!(f, "[{x}").unwrap();
                let mut xs = xs_box.deref();
                while let List::Cons(x, xs_box) = xs {
                    write!(f, ", {x}").unwrap();
                    xs = xs_box.deref();
                }
                write!(f, "]").unwrap();
            }
        }
        Ok(())
    }
}

fn main() {
    let a = Rc::new(List::Cons(
        3,
        Rc::new(List::Cons(2, Rc::new(List::Cons(1, Rc::new(List::Nil))))),
    ));
    println!("head: {}", a.head().unwrap());
    println!("tail: {:?}", a.tail().unwrap());
    println!("last: {}", a.last().unwrap());
    println!();

    let b = List::Cons(4, Rc::clone(&a));
    println!("head: {}", b.head().unwrap());
    println!("tail: {:?}", b.tail().unwrap());
    println!("last: {}", b.last().unwrap());
}
