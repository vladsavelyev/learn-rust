#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<Color>,
}

impl Inventory {
    pub fn most_stocked(&self) -> Color {
        let mut num_blue = 0;
        let mut num_red = 0;
        for color in &self.shirts {
            match &color {
                Color::Blue => {
                    num_blue += 1;
                }
                Color::Red => {
                    num_red += 1;
                }
            }
        }
        if num_blue > num_red {
            Color::Blue
        } else {
            Color::Red
        }
    }

    pub fn giveaway(&self, preference: Option<Color>) -> Color {
        preference.unwrap_or(self.most_stocked())
    }
}

fn main() {
    let shirts = vec![Color::Blue, Color::Blue, Color::Red, Color::Blue];
    let inventory = Inventory { shirts };

    let pref = Some(Color::Red);
    let tshirt = inventory.giveaway(pref);
    println!("For {:?}, giving away {:?}", pref, tshirt);

    let pref = None;
    let tshirt = inventory.giveaway(pref);
    println!("For {:?}, giving away {:?}", pref, tshirt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_giveaway() {
        let shirts = vec![Color::Blue, Color::Blue, Color::Red, Color::Blue];
        let inventory = Inventory { shirts };
        assert_eq!(inventory.giveaway(Some(Color::Red)), Color::Red);
        assert_eq!(inventory.giveaway(None), Color::Blue);
    }
}
