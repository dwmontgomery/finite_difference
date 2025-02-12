fn main() {
    
    let x = 1.0;
    let y = x.sin();
    let y_deriv = x.cos();

    let mut results: Vec<(f64, f64, f64)> = vec::new();

    for i in 1..30 {
        let h = 2.0_f64.powf(-i as f64);
        let y_approx = approx_derivative(x, h);
        let abs_error = (y_approx - y_deriv).abs();

        results.push((h, y_approx, abs_error));
        println!(h + "\t" + y_approx + "\t" + abs_error);
    }
    
}

// computes the approximate derivative of sin(x) using the finite difference formula
// f'(x) = ( (f(x + h) - f(x) ) / h )
fn approx_derivative(x: f64, h: f64) -> f64 {
    ((x+h).sin() - x.sin()) / h 
}

// Compute the Cleve-Moler epsilon value
fn cleve_moler() -> f64 {
    let a = 4.0 / 3.0;
    let b = a - 1.0;
    let c = b + b + b; 
    (1.0-c).abs();
}





