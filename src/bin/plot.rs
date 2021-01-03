use std::io;
use std::error::Error;
use gnuplot::{ AxesCommon, Caption, Figure };

fn main() -> Result<(), Box<dyn Error>> {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    let mut reader = csv::Reader::from_reader(io::stdin());

    for result in reader.records() {
        let record = result?;
        x.push(record[0].parse().unwrap());
        y.push(record[1].parse().unwrap());
    }

    let mut fg = Figure::new();
    fg.axes2d().points(x, y, &[Caption("Cat")]);
    fg.show();
    Ok(())
}

