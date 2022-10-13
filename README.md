# Summer of Rust Lab 3: The Comic Book Shoppe 1

In this lab, we're going to jump back and forth a bit. We'll start with [part of
chapter 4](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), and
learn some of the basics of ownership. Then, before we get too far, we'll dive
into [structs in chapter
5](https://doc.rust-lang.org/book/ch05-00-structs.html).

## The Comic Book Shoppe

The Comic Book Shoppe is a store in Ottawa that sells board games, comics, and
various other cool stuff 🎲 Today we're going to set up R.U.S.T. (Realtime and
Updated Shop Tender) for them, a system to help them answer customer questions.
We care about a few questions for the store:

- How expensive are all the cards in the store? 💰
- What is the total damage done by all the cards in the store? ⚔️
- What is the total health of all the cards in the store? 🍎

### Set up the Card and Shop

First, we need to set up the data structures. There are two that we need to
write: Shop and Card.

Shop is a struct that holds 3 cards. We'll use an array to store the cards for
now. This has a bit of a downside; Rust doesn't have "null" values, so we can't
just store `[Card, Card, null]` in the array. For now, we'll just have a store
that always has 3 cards. However, in a few sessions, we'll explore `Option`s in
Rust, which gives us a really nifty way to improve this!

Card is a struct that holds three fields:

- `price`
- `damage`
- `health`

You can choose the data type that makes the most sense to hold these!

Once you're done setting up the structs, we can uncomment the code at the
beginning of the main function (in VS Code, select it all, and press
`ctrl|command + /`). It won't work just yet, since we need to fix some
`todo!()`s, but it should be less red at least!

### impl Shop

Next, we need to add functionality to the Shop struct. With the Shop, we want to
be able to easily find three things:

- The price of the most expensive Card in the shop
- The sum of the damage from all cards in a shop
- The sum of the health from all cards in a shop

For this, we're going to work in the `impl Shop` block. You'll notice that there
are already three methods there with `todo!()`s. This is a bit for you to know
what needs to be implemented, but also because there's a sneaky `&` in the
parameters.

```rust
impl Shop {
    fn most_expensive(&self) -> u32 {
        todo!()    // ^ The sneaky ampersand!
    }
    ...
}
```

Although we haven't gone through a lot of the borrow checker yet, we can quickly
look at what it's doing. When getting `self` in a method, we have three
different ways of accessing it:

- `self`: We're going to move `self` (if we don't return it explicitly, then we
  consume it!)
- `&self`: We're going to borrow `self` (but after this method is over, the
  place that called this method can keep it)
- `&mut self`: We're going to borrow `self` mutably (so we can change parts of
  the struct)

Anyways, since we're just querying the data, we can just use `&self` and it'll
be just fine.

Also, you'll notice that the word "function" and "method" might be getting
thrown around a bit more here. A method is a function that is "implemented" on a
struct, and takes `self` as the first parameter.

```rust
struct Dog {} // 🐶

impl Dog {
    fn bark(&self) { // <-- this is a method
        println!("Woof!");
    }
}

fn main() { // <-- this is a function
    let d = Dog {};
    
    d.bark();
}
```

Now that we know some more, let's work on each of the three methods:

- The price of the most expensive Card in the shop
- The sum of the damage from all cards in a shop
- The sum of the health from all cards in a shop

### Check the program

The tests for this lab are a bit longer than last time. This time, we have them
split into a test for each idea that needs to be tested. This should make it
easier to know what needs to be fixed if a test doesn't work!

Remember, you can either run tests with Rust Analyzer in VS Code, or with `cargo
test` in the terminal.

### Formatting and checks

As part of this lab, there will also be tests to make sure your code is
formatted and there are not warnings! For this, make sure you run `cargo fmt`
and `cargo check` before submitting. This will make sure your code is looking
the best it can 💯

### Next Stop: Rustlings

![](https://c.tenor.com/09_MQFWe3zQAAAAC/crab-crab-petting.gif)

From this week on, we'll have some **rustlings** to do. Rustlings are small
exercises that get you used to reading and fixing small amounts of Rust code. It
follows the general structure of the book, and so there will be a lot to do as
we keep learning!

Start by installing rustlings. If you have issues getting it installed, please
ask for help on the Discord server! Here is the [rustlings Project
Link](https://github.com/rust-lang/rustlings).

Once you run rustlings, it will automatically guide you through the exercises.
For now, we'll do everything up to quiz one.

### That's all!

See you next week 🏖️

## License

The Summer of Rust Labs are duel-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))