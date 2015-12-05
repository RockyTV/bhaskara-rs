use std::f64;
use std::io;

fn main() {
    let mut input: String = String::new();
    let (mut a, mut b, mut c): (i32, i32, i32) = (0, 0, 0);

    println!("Input A: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    if !input.contains("x^2") {
        panic!("Invalid value specified for A.");
    }
    
    let mut index = 0;
    let mut parse = true;

    if input.starts_with("+") {
        a = 1;
        index = 1;
    }

    if input.find('x') == Some(0) {
        a = 1;
        parse = false;
    }

    if input.starts_with("-") {
        a = -1;
        index = 1;
    }

    let x_index = input.find('x').unwrap() as usize;
    let mut first_char: i32 = 1;
    if parse {
        first_char = match input[index..x_index].trim().parse() {
            Ok(x) => {
                x
            },
            Err(_) => panic!("Invalid integer value for A"),
        };
    }
    a *= first_char;

    input = String::new();

    println!("Input B: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    
    let mut parse = true;

    if !input.contains("x") {
        panic!("B must contain x");
    }

    if input.starts_with("+") || input.starts_with("-") {
        let x_index = input.find('x').unwrap() as usize;
        let first_index = &input[0..1];
        let mut first_char: i32 = 1;

        if first_index == "+" {
            first_char = 1;
        }
        else if first_index == "-" {
            first_char = -1;
        }

        if x_index == 1usize {
            parse = false;
        }

        if parse {
            first_char = match input[1..x_index].trim().parse() {
                Ok(x) => x,
                Err(_) => panic!("Invalid integer value for B"),
            };
        }

        b = first_char;
    }
    
    input = String::new();
    println!("Input C: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let first_index = &input[0..1];
    if input.contains("x") {
        panic!("C must not contain any variables");
    }
    
    c = match input.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("Invalid value for C (C must be an integer)"),
    };

    // delta = b^2-4ac
    let delta: i32 = b.pow(2) - 4*a*c;
    let (mut x1, mut x2): (i32, i32) = (0, 0);

    if delta < 0 {
        println!("delta is {} (less than 0), therefore x does not exist", delta);
    }
    else {
        // x = -b +- sqrt(delta) / 2a
        x1 = ((-1*b) + (delta as f64).sqrt() as i32) / 2*a;
        x2 = ((-1*b) - (delta as f64).sqrt() as i32) / 2*a;
        println!("x is ({}, {})", x1, x2);
    }
}
