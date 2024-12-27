enum Meal {
    Pizza,
    Pasta,
    Salad,
}

fn order(meal: Meal) {
    match meal {
        Meal::Pizza => println!("¡You liked the pizza!"),
        Meal::Pasta => println!("¡You like the pasta!"),
        Meal::Salad => println!("¡You like the salad!"),
    }
}

fn main() {
    let meal = Meal::Pizza;
    order(meal);
}
