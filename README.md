# oh
A collections of functions for highly efficient programmers.

## Examples
### input
```rust
extern crate oh;

fn main() {
    let name = oh::input("What's your name? : ").expect("Error at input");
    println!("Oh! So your name is {}!", name);
}
```

## Contributing 
To learn more about contributing to this project, read [CONTRIBUTING.md](https://github.com/yuulive/oh/blob/main/CONTRIBUTING.md)