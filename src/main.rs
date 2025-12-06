use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("weather_stations.csv").unwrap();
    let f = BufReader::new(f);
    let mut stats = HashMap::<String, (f64, f64, f64, usize)>::new();
    for line in f.lines() {
        let line = line.unwrap();
        print!("{}", line);
        let (station, temprature) = line.split_once(";").unwrap();
        let temprature = temprature.parse().unwrap();
        let stats = stats.entry(station.to_string()).or_default();
        stats.0 = stats.0.min(temprature);
        stats.1 = stats.1.max(temprature);

        stats.2 += temprature;
        stats.3 += 1;
    }

    print!("{{");
    let stats = BTreeMap::from_iter(stats);
    let mut stats = stats.into_iter().peekable();
    while let Some((station, (min, max, total, count))) = stats.next() {
        print!(
            "{station}={min:.1}/{:.1}/{max:.1}",
            total / { count as f64 }
        );
        if stats.peek().is_some() {
            print!(", ")
        }
    }
    print!("}}");
}
