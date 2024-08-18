use std::io::{self, Write};

fn main() {
    let mut coefs: [f32; 3] = [0.0; 3];

    println!("ax^2 + bx + c = 0");

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

        if coefs[0] == 0.0 {
            println!("AHEM. What made you think that was smart?");
            panic!("User set a = 0");
        }
    }

    println!("\nInputted {:?} -> [a, b, c]\n", coefs);
    print!("The solutions to the equation {}x^2 + {}x + {} = 0 are ", coefs[0], coefs[1], coefs[2]);
    if (coefs[1] * coefs[1]) - (4.0 * coefs[0] * coefs[2]) >= 0.0 {
        solve_real(coefs);
    } else {
        solve_complex(coefs);
    }


    io::stdout().flush().unwrap();
}

fn solve_real(coefs: [f32; 3]) {
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
    println!("{} and {}", x1, x2);
    io::stdout().flush().unwrap();
}
fn solve_complex(coefs: [f32; 3]) {
    print!(
        "({} {} {}{}) / {}",
        -1.0 * coefs[1],
        '+',
        (
            coefs[1] * coefs[1] - 4.0 * coefs[0] * coefs[2] 
        ).abs().sqrt(),
        "i",
        2.0 * coefs[0]
    );

    print!(" and ");

    print!(
        "({} {} {}{}) / {}",
        -1.0 * coefs[1],
        '-',
        (
            coefs[1] * coefs[1] - 4.0 * coefs[0] * coefs[2] 
        ).abs().sqrt(),
        "i",
        2.0 * coefs[0]
    );
    
    print!("\n");
    io::stdout().flush().unwrap();
    
}
