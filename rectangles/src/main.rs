#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {
        Rect {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let scale = 2;

    let rect = Rect {
        width: dbg!(3 * scale),
        height: 4,
    };
    dbg!(&rect);
    println!("Area is: {}", rect.area());

    let rect2 = Rect {
        width: 5,
        height: 3,
    };
    dbg!(rect.can_hold(&rect2));
    dbg!(rect2.can_hold(&rect));

    let square = Rect::square(4);

    dbg!(rect.can_hold(&square));
    dbg!(rect2.can_hold(&square));
}
