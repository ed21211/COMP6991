# COMP991

cargo init "filename"
cargo new "filename"
cargo add "dependancies"

cargo run 
cargo build (without running)
cargo check (see if it compiles)

current; running in debug mode, tries to runs and touche all ends of code to make sure its runs proplerly

cargo run —release (compiles faster) makes shortcuts <br>

# WEEK 00 

**Printing lines**
```rust  
println!("hello!") //prints line with newline  
print!("hellooo~") //does not have newline  
eprint!("error or progress msg") 
```  

**Read stdin**  
```rust
use std::io;

let mut input = String::new();
let _ = io::stdin().read_line(&mut input)
```
# WEEK 01
**comand-line input**
```rust
let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;
```

**string analysis**
- empty (trim removes \n)
```rust
let pattern = "abc".to_string();

if input.trim().is_empty() {
    print!("empty string!")

} else if input.contains(pattern) {
    print!("input contains the pattern!")
    
}
```

**Structs**
```Rust
struct UniverseDetails {
    universe_name: String,
    universe_winner: String,
    universe_population: u32
}


let three = UniverseDetails {
    universe_name: "Star Wars".to_string(),
    universe_winner: "Rebellion".to_string(),
    universe_population: 4294967295,
};
```

**Option**
```Rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
```

**Match**
```Rust
match result {
    Ok(_) => {
        print!("{result}")
    }
    Err(e) => {
        print!("Error: {e}")
    }
}
```
