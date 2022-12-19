mod plot;

use csv::StringRecord;
use plot::{Env, Plot};

fn read_data(path: &str) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut x = Vec::new();
    let mut y1 = Vec::new();
    let mut y2 = Vec::new();
    let mut y3 = Vec::new();
    let rdr = csv::Reader::from_path(path);
    for result in rdr.expect("Missing data file?").records().into_iter()  {
        let record = result.expect("Missing record?");
        x.push(get_f64(&record, 0));
        y1.push(get_f64(&record, 1));
        y2.push(get_f64(&record, 2));
        y3.push(get_f64(&record, 3));
    }
    (x, y1, y2, y3)
}

fn get_f64(record: &StringRecord, index: usize) -> f64 {
    record.get(index).expect(&*format!("Missing field {index}")).trim().parse::<f64>().unwrap()
}

fn main() {
    let (time, acc, pos, _ /*vel*/) = read_data("test-data.txt");

    let env = Env::new();
    let plot = Plot::new(&env);

    plot.plot(&time, &acc);
    plot.plot(&time, &pos);
    plot.grid(true);
    plot.xlabel("Time");
    plot.ylabel("Pos");
    plot.title("Est Pos vs Ref Pos");
    plot.set_ylim(-2.0, 2.0);
    plot.draw();
    plot.show();
}

