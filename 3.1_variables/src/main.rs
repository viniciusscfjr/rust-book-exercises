fn main() {
    println!("Mutable Variable");
    mutable_variable();
    println!("-----------------");

    println!("Shadowing");
    shadowing();
    println!("-----------------");
    
    println!("Reassign Variable");
    reassign_variable();
    println!("-----------------");
}

fn mutable_variable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn reassign_variable() {
    let spaces = "   ";
    println!("The value of spaces is: {spaces}");

    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}