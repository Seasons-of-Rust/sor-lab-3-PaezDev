use core::time;
use std::thread;

fn main() {
    // Once you've set up the Shop and Card structs, you should be able to
    // uncomment this code
    //
    // let comic_book_shoppe = Shop {
    //     cards: [
    //         Card {
    //             price: 10,
    //             health: 10,
    //             damage: 10,
    //         },
    //         Card {
    //             price: 20,
    //             health: 20,
    //             damage: 20,
    //         },
    //         Card {
    //             price: 30,
    //             health: 30,
    //             damage: 30,
    //         },
    //     ],
    // };

    println!("Welcome to The Comic Book Shoppe!");
    println!("We've got three cards for you to check out.");

    println!(
        "Oh, you want to know how much the most expensive card we have is? Well, let's find out!"
    );

    // Remember to add some time to sleep so that the customer thinks we're doing real
    // work in the back!
    thread::sleep(time::Duration::from_millis(500));
    println!("...");
    thread::sleep(time::Duration::from_millis(500));
    println!("...");
    thread::sleep(time::Duration::from_millis(800));

    println!(
        "Looks like it comes to a total of {} dollars.",
        comic_book_shoppe.most_expensive()
    );

    // Here are some other stats, like the total damage and health of the shop
    println!("");
    println!("");
    println!("And here are some other stats about our store:");
    println!("");
    println!(
        "The total damage of the shop is {}.",
        comic_book_shoppe.total_damage()
    );
    println!(
        "The total health of the shop is {}.",
        comic_book_shoppe.total_health()
    );

    // Thank the customer for shopping at the Comic Book Shoppe
    println!("");
    println!("");
    println!("And that's all folks!");
    println!("Thanks for shopping at The Comic Book Shoppe!");
}

/// A Shop is a collection of 3 cards.
struct Shop {
    // TOOD: Add the field to this struct
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    fn most_expensive(&self) -> u32 {
        todo!()
    }

    /// Get the total damage of all cards in the shop
    fn total_damage(&self) -> u32 {
        todo!()
    }

    /// Get the total health of all cards in the shop
    fn total_health(&self) -> u32 {
        todo!()
    }
}

/// A Card is a card stores a price, health, and damage.
struct Card {
    // TODO: Add fields to this struct
}

#[cfg(test)]
mod tests {
    use crate::{Card, Shop};

    #[test]
    fn test_can_create_card() {
        // Create a card
        let card = Card {
            price: 10,
            health: 10,
            damage: 10,
        };
        assert_eq!(card.price, 10);
    }

    #[test]
    fn test_can_create_shop() {
        let shop = Shop {
            cards: [
                Card {
                    price: 10,
                    health: 10,
                    damage: 10,
                },
                Card {
                    price: 10,
                    health: 10,
                    damage: 10,
                },
                Card {
                    price: 10,
                    health: 10,
                    damage: 10,
                },
            ],
        };
        assert_eq!(shop.cards[0].health, 10);
    }

    #[test]
    fn test_most_expensive_card() {
        let shop = Shop {
            cards: [
                Card {
                    price: 12314,
                    health: 0,
                    damage: 0,
                },
                Card {
                    price: 1241,
                    health: 0,
                    damage: 0,
                },
                Card {
                    price: 312340,
                    health: 0,
                    damage: 0,
                },
            ],
        };
        assert_eq!(shop.most_expensive(), 312340);
    }

    #[test]
    fn test_total_damage() {
        let shop = Shop {
            cards: [
                Card {
                    price: 0,
                    health: 0,
                    damage: 12,
                },
                Card {
                    price: 0,
                    health: 0,
                    damage: 52,
                },
                Card {
                    price: 0,
                    health: 0,
                    damage: 21,
                },
            ],
        };
        assert_eq!(shop.total_damage(), 85);
    }

    #[test]
    fn test_total_health() {
        let shop = Shop {
            cards: [
                Card {
                    price: 0,
                    health: 24,
                    damage: 0,
                },
                Card {
                    price: 0,
                    health: 33,
                    damage: 0,
                },
                Card {
                    price: 0,
                    health: 74,
                    damage: 0,
                },
            ],
        };
        assert_eq!(shop.total_health(), 131);
    }
}
