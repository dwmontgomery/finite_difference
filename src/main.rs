use plotters::prelude::*;

fn main() {
    
    let x: f64 = 1.0;
    let y_deriv = x.cos();

    // Results Vector (i_value, approx_derivitive, absolute_error)
    let mut results: Vec<Vec<f64>> = Vec::new();

    println!("|{:^6}|{:^15}|{:^15}|{:^15}|{:^15}|", "h", "x", "Approx. f'(x)", "Known f'(x)", "Abs. Error)");
    println!("|:{:-<4}:|{:-<14}:|{:-<14}:|{:-<14}:|{:-<14}:|", "-", "-", "-", "-", "-");
    for i in 1..30 {
        let h = h_value(i as f64);
        let y_approx = approx_derivative(x, h);
        let abs_error = (y_approx - y_deriv).abs();

        results.push(vec![i as f64, h, y_approx, abs_error]);
        println!("|2^-{:02} |{:>14.8} |{:>14.8} |{:>14.8} |{:>14.8} |", 
            i, x, y_approx, y_deriv, abs_error);
    }

    let cm = cleve_moler();
    println!("eps: {:12.8} ", cm);
    println!("sqrt(eps): {:12.8} ", cm.sqrt());

    plot_error(&results).expect("Failed to create plot");
    
}

// Convert 2^-i into the approximate value
fn h_value(i: f64) -> f64 {
    2.0_f64.powf(-i)
}

// computes the approximate derivative of sin(x) using the finite difference formula
// f'(x) = ( (f(x + h) - f(x) ) / h )
fn approx_derivative(x: f64, h: f64) -> f64 {
    ((x+h).sin() - x.sin()) / h 
}

// Compute the Cleve-Moler epsilon value
fn cleve_moler() -> f64 {
    let a:f64 = 4.0 / 3.0;
    let b = a - 1.0;
    let c = b + b + b; 
    (1.0-c).abs()
}

// Plots the Abs. Error (y-axis) vs. h (x-axis)
fn plot_error(values: &Vec<Vec<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Absolute Error vs. h", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d((1e-10f64..1e-0f64).log_scale(), (1e-16f64..1e0f64).log_scale())?;
    
    chart.configure_mesh().draw()?;
    
    let h_values: Vec<f64> = values.iter().map(|row| row[1]).collect();
    let abs_error_values: Vec<f64> = values.iter().map(|row| row[3]).collect();

    chart.draw_series(LineSeries::new(
        h_values.iter().copied().zip(abs_error_values.iter().copied()),
        &RED,
    ))?;
    
    root.present()?;
    Ok(())
}





