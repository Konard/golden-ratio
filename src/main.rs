fn golden_ratio_loop(iterations: u32) -> f64 {
    let mut approximation = 1.0;

    for _ in 0..iterations {
        approximation = 1.0 + 1.0 / approximation;
    }

    approximation
}

fn golden_ratio_recursion(iterations: u32) -> f64 {
    if iterations <= 0 {
        return 1.0;
    }
    1.0 + 1.0 / golden_ratio_recursion(iterations - 1)
}

fn main() {
    let iterations = 30;  // Increase the number of iterations for a more accurate result.
    let phi = golden_ratio_loop(iterations);
    println!("The approximate value of the golden ratio using loop is: {}", phi);
    let phi = golden_ratio_recursion(iterations);
    println!("The approximate value of the golden ratio using recursion is: {}", phi);
}