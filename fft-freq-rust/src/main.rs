extern crate num;
extern crate rustfft;

use rustfft::{FftPlanner, num_complex::Complex};

const PI: f64 = std::f64::consts::PI;

fn main() {
    // Set the sample rate and number of samples
    let sample_rate =   100;  // Hz
    let num_samples = 1024;

    // Generate some test data
    let mut data: Vec<Complex<f64>> = Vec::new();
    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let value = (2.0 * PI * t).sin() + 5.0 * (2.0 * PI * 10.0 * t).sin();
        data.push(Complex::new(value, 0.0));
    }

    // Create an FFT planner
    let mut planner = FftPlanner::new();

    // Plan an FFT of the data
    let fft = planner.plan_fft_forward(num_samples);

    // Perform the FFT
    fft.process(&mut data);

    // Find the main frequency
    let mut main_freq = 0;
    let mut main_power = 0.0;
    for (i, sample) in data.iter().enumerate() {
        let power = sample.re * sample.re + sample.im * sample.im;
        if power > main_power {
            main_freq = i;
            main_power = power;
        }
    }

    // Convert the main frequency to Hz
    let main_freq = main_freq as f64 * sample_rate as f64 / num_samples as f64;

    println!("Main frequency: {} Hz", main_freq);
}
