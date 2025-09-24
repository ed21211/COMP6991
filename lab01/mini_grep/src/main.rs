use std::io;
fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;
    
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().is_empty() {
                    break
                
                } else if input.contains(pattern) {
                    print!("{input}")
                }
            }
            Err(e) => {
                print!("Error: {e}")
            }
        }
    }
}
