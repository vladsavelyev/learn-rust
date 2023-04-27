pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    max: usize,
    value: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        LimitTracker {
            messenger,
            max,
            value: 0,
        }
    }

    pub fn set_value(&mut self, val: usize) {
        self.value = val;
        if self.value > self.max {
            self.messenger
                .send("Warning: you are over quota!");
        } else if self.value == self.max {
            self.messenger.send("Warning: quota reached!");
        } else {
            let used = 100 * val / self.max;
            if used > 75 {
                self.messenger
                    .send(&format!("Warning: used up {used}% of quota"));
            }
        }
    }
}

struct Printer {}
impl Messenger for Printer {
    fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}

struct Huinter {}
impl Messenger for Huinter {
    fn send(&self, msg: &str) {
        println!("Huyarning: {}", msg);
    }
}

fn main() {
    let messenger = Printer {};
    let mut tracker = LimitTracker::new(&messenger, 1000);
    tracker.set_value(600);
    tracker.set_value(900);
    tracker.set_value(1000);
    tracker.set_value(1200);

    let huinter = Huinter {};
    let mut huecker = LimitTracker::new(&huinter, 500);
    huecker.set_value(600);
    huecker.set_value(900);
    huecker.set_value(1000);
    huecker.set_value(1200);
}

#[cfg(test)]
mod tests {
    use crate::{LimitTracker, Messenger};
    use std::cell;

    struct MockMessenger {
        messages: cell::RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                messages: cell::RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_owned());
        }
    }

    #[test]
    fn test_messenger() {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 1000);
        tracker.set_value(900);
        assert_eq!(mock_messenger.messages.borrow().len(), 1);
    }
}
