use std::io::{self, Write};

fn main() {
    let mut coefs: [f32; 3] = [0.0; 3];

    println!("Enter a, b, c where the quadratic is in the form ax^2 + bx + c = 0:\n");

    for i in 0..coefs.len() {
        let label = match i {
            0 => "a",
            1 => "b",
            2 => "c",
            _ => "Something's wrong!"
        };

        print!("Enter {}: ", label);
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input to buffer.");

        coefs[i] = buffer
            .trim()
            .parse()
            .expect("Enter only a single floating-point or integer number, prefferably without leading/trailing whitespace.");
    }

    println!("\nInputted [a, b, c] = {:?}\n", coefs);
    println!("The solutions to the equation {}x^2 + {}x + {} = 0 are {:?}", coefs[0], coefs[1], coefs[2], solve(coefs));
    io::stdout().flush().unwrap();
}

fn solve(coefs: [f32; 3]) -> (f32, f32) {
    if (coefs[1] * coefs[1]) - (4.0 * coefs[0] * coefs[2]) < 0.0 {
        println!("There are no real solutions to this equation. Throwing error.");
        println!("TODO: add implementation for solution with positive square root * i");
        panic!("Negative discriminant. Currently work in progress.");   
    }


    let x1 = {
        (
            (-1.0 * coefs[1]) + (
                (coefs[1] * coefs[1]) - (4.0 * coefs[0] * coefs[2])
            ).sqrt()
        ) / (2.0 * coefs[0])
    };

    let x2 = {
        (
            (-1.0 * coefs[1]) - (
                (coefs[1] * coefs[1]) - (4.0 * coefs[0] * coefs[2])
            ).sqrt()
        ) / (2.0 * coefs[0])
    };

    (x1, x2)
}
