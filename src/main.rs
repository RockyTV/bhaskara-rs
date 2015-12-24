use std::io;

fn main() {
    let mut input: String = String::new();

    println!("Input A: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let a: i32 =  match input.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("Invalid value for A"),
    };

    // If A is zero, then later we would have a division by zero.
    if a == 0 {
        panic!("A must be different than 0");
    }

    input = String::new();
    println!("Input B: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let b: i32 = match input.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("Invalid value for B"),
    };

    input = String::new();
    println!("Input C: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let c: i32 = match input.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("Invalid value for C"),
    };

    // delta = b² - 4ac
    let delta: i32 = b.pow(2) - 4*a*c;
    let (mut x1, mut x2): (i32, i32) = (0, 0);

    let mut equation = "{0}x^2{1}x{2}".replace("{0}", &a.to_string());

    let mut b_str = b.to_string();
    let mut c_str = c.to_string();
    
    if b.is_positive() {
        b_str.insert(0, '+');
    }
    equation = equation.replace("{1}", &b_str);

    if c.is_positive() {
        c_str.insert(0, '+');
    }
    equation = equation.replace("{2}", &c_str);

    if delta < 0 {
        println!("The quadratic equation {} has no roots", equation);
    } else {
        x1 = ((-1*b) + (delta as f64).sqrt() as i32) / 2*a;
        x2 = ((-1*b) - (delta as f64).sqrt() as i32) / 2*a;
        println!("roots for {} are:", equation);
        println!("{}, {}", x1, x2);
    }
}


