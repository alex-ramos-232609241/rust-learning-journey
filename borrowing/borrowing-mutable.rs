fn main() {
    let mut s = String::from("Hello");
    let s2 = &mut s;

    s2.push_str(", ing Alex - borrowing mutable");
    println!("{}", s2);
}
