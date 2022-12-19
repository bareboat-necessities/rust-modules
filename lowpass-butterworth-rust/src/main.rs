use iir_filters::filter_design::FilterType;
use iir_filters::filter_design::butter;
use iir_filters::sos::zpk2sos;
use iir_filters::filter::DirectForm2Transposed;
use iir_filters::filter::Filter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let order = 5;
    let cutoff_low = 1.0;
    let fs = 81.0;

    let zpk = butter(order, FilterType::LowPass(cutoff_low), fs)?;
    let sos = zpk2sos(&zpk, None)?;

    let mut dft2 = DirectForm2Transposed::new(&sos);

    let input:Vec<f64>  = vec![1.0, 2.0, 3.0];
    let mut output:Vec<f64> = vec![];

    for x in input.iter() {
        output.push( dft2.filter(*x) );
    }
    return Ok(());
}
