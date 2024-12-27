fn main() {
    let x: i32 = 10;
    println!("The value of x is: {}", x);

    let mut y: f64 = 3.14;
    println!("The value of y is: {}", y);
    y = 2.71;
    println!("The value of y modified is: {}", y);

    let name: &str = "Ana";
    println!("The name is: {}", name);

    let seed: char = 'A';
    println!("La seed is: {}", seed);

    let activate: bool = true;
    println!("It is activate: {}", activate);

    //Shadowing (Ocultamiento):
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("El valor final de z is: {}", z);

    //const:
    const PI: f64 = 3.14159265359;
    println!("The value of PI is: {}", PI);

}