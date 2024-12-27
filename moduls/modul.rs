mod greetings {
    pub fn greet() {
        println!("¡The function is inside a module!");
    }
}

fn main() {
    greetings::greet();
}
