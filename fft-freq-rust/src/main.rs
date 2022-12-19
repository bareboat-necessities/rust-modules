extern crate rustfft;

use rustfft::{FftPlanner, num_complex::Complex};

const PI: f64 = std::f64::consts::PI;

fn main() {
    // Set the sample rate and number of samples
    let sample_rate =  100;  // Hz
    let num_samples = 1024;

    // Generate some test data
    let mut signal: Vec<Complex<f64>> = Vec::new();
    for i in 0..num_samples {
        let t = i as f64 / sample_rate as f64;
        let value = (2.0 * PI * t).sin() + 5.0 * (2.0 * PI * 10.0 * t).sin();
        signal.push(Complex::new(value, 0.0));
    }
    let main_freq = main_freq(sample_rate, &mut signal);
    println!("Main frequency: {} Hz", main_freq);
}

fn main_freq(sample_rate_hz: i32, mut signal: &mut Vec<Complex<f64>>) -> f64 {
    // Create an FFT planner
    let mut planner = FftPlanner::new();

    // Plan an FFT of the data
    let fft = planner.plan_fft_forward(signal.len());

    // Perform the FFT in-place
    fft.process(&mut signal);

    // Find the main frequency
    let mut main_freq = 0;
    let mut main_power = 0.0;
    for (i, sample) in signal.iter().enumerate() {
        let power = sample.re * sample.re + sample.im * sample.im;
        if power > main_power {
            main_freq = i;
            main_power = power;
        }
    }

    // Convert the main frequency to Hz
    let main_freq = main_freq as f64 * sample_rate_hz as f64 / signal.len() as f64;
    main_freq
}
