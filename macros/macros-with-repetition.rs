macro_rules! sum {
    ($($num:expr),*) => {
        {
            let mut total = 0;
            $(
                total += $num;
            )*
            total
        }
    };
}

fn main() {
    let result = sum!(1, 2, 3, 4);
    println!("Sum is: {}", result);
}
