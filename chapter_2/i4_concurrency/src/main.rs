use num::Complex;

fn main() {
    println!("Hello, world!");

    square_loop(2.3);
}

fn square_loop(c: f64) {
    let x = 0.;

    loop {
        println!("{}", x = x * x + c);
    }
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };

    loop {
        println!("{}", z = z * z + c)
    }
}
