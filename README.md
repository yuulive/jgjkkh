# lazyfunctions
A collections of functions for lazy people, or highly efficient programmers.

## Examples
### input
```rust
extern crate lazyfunctions;

fn main() {
    let name = lazyfunctions::input("What's your name? : ").expect("Error at input");
    println!("Oh! So your name is {}!", name);
}
```

## Contributing 
To learn more about contributing to this project, read [CONTRIBUTING.md](https://github.com/dakokonutboi/lazyfunctions/blob/main/CONTRIBUTING.md)