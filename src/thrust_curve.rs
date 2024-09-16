use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct ThrustCurve {
    pub time: Vec<f64>,
    pub thrust: Vec<f64>,
}

impl ThrustCurve {
    pub fn new() -> Self {
        ThrustCurve {
            time: Vec::new(),
            thrust: Vec::new(),
        }
    }

    pub fn load_from_eng<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
        let mut curve = ThrustCurve::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let Ok(time) = parts[0].parse() {
                    if let Ok(thrust) = parts[1].parse() {
                        curve.time.push(time);
                        curve.thrust.push(thrust);
                    }
                }
            }
        }

        Ok(curve)
    }

    pub fn thrust_at(&self, time: f64) -> f64 {
        match self
            .time
            .binary_search_by(|&probe| probe.partial_cmp(&time).unwrap())
        {
            Ok(index) => self.thrust[index],
            Err(index) => {
                if index == 0 {
                    self.thrust[0]
                } else if index == self.time.len() {
                    0.0
                } else {
                    let t1 = self.time[index - 1];
                    let t2 = self.time[index];
                    let f1 = self.thrust[index - 1];
                    let f2 = self.thrust[index];
                    ((time - t1) / (t2 - t1)) * (f2 - f1) + f1
                }
            }
        }
    }
}


