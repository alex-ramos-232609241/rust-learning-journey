fn main() {
    for i in 1..5 {
        println!("Number: {}", i);
    }

    let mut counter = 0;
    while counter < 3 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    let mut sum = 0;
    loop {
        sum += 1;
        if sum == 10 {
            break;
        }
    }
    println!("Total sum: {}", sum);
}
