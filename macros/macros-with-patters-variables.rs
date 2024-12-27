macro_rules! greet {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    () => {
        println!("Hello, Alex!");
    };
}

fn main() {
    greet!("Enrique");
    greet!();
}
