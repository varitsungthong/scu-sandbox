//mod builds the tree
//pub unlock model to talk together
//use creates the shortcut so you don't have to type out the full tree address every time

//how to organize code in Rust
//1. The lib.rs / main.rs Split
    //- most of the logic intro lib.rs
    //- the main.rs should only handle user input, call functions from lib.rs, and print results
//2. Group by "Domain," Not by "Type" 
    //- Group files by what they do (their Domain), not what they are.
//3. Use pub as defensive programming
    //- Use pub to make functions and types public only when they need to be.
//4. Use pub mode core and pub use to re-export
    //- Use pub mod core to make a module public, and pub use to re-export items from that module.
    fn main() {
        println!("Hello, world!");
    }