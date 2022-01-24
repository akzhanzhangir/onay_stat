use std::{collections::HashMap, usize};

#[derive(Default, Debug)]
pub struct Calc {
    data: Vec<f64>,
    sorted: Vec<f64>,
    len: usize,
    even: bool,
    middle_index: usize,
    calc_total: bool,
    calc_mean: bool,
    calc_median: bool,
    calc_range: bool,
    calc_variance: bool,
    calc_standart_deviation: bool,
    calc_sorted_median: bool,
    calc_max: bool,
    calc_max_indices: bool,
    calc_min: bool,
    calc_min_indices: bool,
    calc_mode: bool,
    register: Register,
}
#[derive(Default, Debug)]
struct Register {
    total: f64,
    mean: f64,
    median: f64,
    range: f64,
    variance: f64,
    standart_deviation: f64,
    sorted_median: f64,
    max_indices: Vec<usize>,
    max_value: f64,
    min_indices: Vec<usize>,
    min_value: f64,
    mode: f64,
}
impl Calc {
    pub fn length(&self) -> usize {
        self.len
    }

    pub fn even(&self) -> bool {
        self.even
    }

    pub fn total(&mut self) -> f64 {
        if !self.calc_total {
            self.register.total = self.data.iter().sum();
            self.calc_total = true;
        }
        self.register.total
    }

    pub fn mean(&mut self) -> f64 {
        if !self.calc_mean {
            self.register.mean = self.total() / (self.len as f64);
            self.calc_mean = true;
        }
        self.register.mean
    }

    pub fn median(&mut self) -> f64 {
        if self.len == 1 {
            self.register.sorted_median = self.data[0];
            self.register.median = self.data[0];
            self.calc_sorted_median = true;
            self.calc_median = true;
        }

        if !self.calc_median {
            if self.even {
                self.register.median =
                    (self.data[self.middle_index] + self.data[self.middle_index - 1]) / 2.0;
            } else {
                self.register.median = self.data[self.middle_index];
            }
        }
        self.calc_median = true;
        self.register.median
    }

    pub fn sorted_median(&mut self) -> f64 {
        if self.len == 1 {
            self.register.sorted_median = self.data[0];
            self.register.median = self.data[0];
            self.calc_sorted_median = true;
            self.calc_median = true;
        }
        if !self.calc_sorted_median {
            if self.even {
                self.register.sorted_median =
                    (self.sorted[self.middle_index] + self.sorted[self.middle_index - 1]) / 2.0;
            } else {
                self.register.sorted_median = self.sorted[self.middle_index];
            }
        }
        self.calc_sorted_median = true;
        self.register.sorted_median
    }

    pub fn range(&mut self) -> f64 {
        if !self.calc_range {
            self.register.range = self.max_value() - self.min_value();
            self.calc_range = true;
        }
        self.register.range
    }

    pub fn variance(&mut self) -> f64 {
        if !self.calc_variance {
            let mean = self.mean();
            for i in self.data.iter() {
                let temp = i - mean;
                self.register.variance += temp * temp;
            }
            self.register.variance /= self.len as f64;
            self.calc_variance = true;
        }
        self.register.variance
    }

    pub fn standart_deviation(&mut self) -> f64 {
        if !self.calc_standart_deviation {
            self.register.standart_deviation = self.variance().sqrt();
            self.calc_standart_deviation = true;
        }
        self.register.standart_deviation
    }

    pub fn max_indices(&mut self) -> Vec<usize> {
        if !self.calc_max_indices {
            let max = self.max_value();
            for i in 0..self.len {
                if self.data[i] == max {
                    self.register.max_indices.push(i);
                }
            }
            self.calc_max_indices = true;
        }
        self.register.max_indices.clone()
    }

    pub fn max_value(&mut self) -> f64 {
        if !self.calc_max {
            self.register.max_value = self.sorted[self.len - 1];
            self.calc_max = true
        }
        self.register.max_value
    }

    pub fn min_indices(&mut self) -> Vec<usize> {
        if !self.calc_min_indices {
            let min = self.min_value();
            for i in 0..self.len {
                if self.data[i] == min {
                    self.register.min_indices.push(i);
                }
            }
            self.calc_min_indices = true;
        }
        self.register.min_indices.clone()
    }

    pub fn min_value(&mut self) -> f64 {
        if !self.calc_min {
            self.register.min_value = self.sorted[0];
            self.calc_min = true
        }
        self.register.min_value
    }

    pub fn mode(&mut self) -> f64 {
        if !self.calc_mode {
            let mut counts = HashMap::new();
            self.register.mode = self
                .data
                .iter()
                .copied()
                .max_by_key(|&n| {
                    let count = counts.entry(n.to_string()).or_insert(0);
                    *count += 1;
                    *count
                })
                .unwrap();
        }
        self.register.mode
    }

    pub fn run_all(&mut self) {
        self.sorted_median();
        self.median();
        self.standart_deviation();
        self.max_indices();
        self.min_indices();
        self.range();
        self.mode();
    }

    pub fn display(self) {
        println!("Data {:?}", self.data);
        println!("Count {}", self.len);
        println!("Total {}", self.register.total);
        println!("Mean {}", self.register.mean);
        println!("Median {}", self.register.median);
        println!("Sorted median {}", self.register.sorted_median);
        println!("Mode {}", self.register.mode);
        println!("Range {}", self.register.range);
        println!("Variance {}", self.register.variance);
        println!("Standart Deviation {}", self.register.standart_deviation);
        println!("Max {}", self.register.max_value);
        println!("Max indecies {:?}", self.register.max_indices);
        println!("Min {}", self.register.min_value);
        println!("Min indecies {:?}", self.register.min_indices);
    }

    /// Making a new Calc struct from f64 vector
    pub fn new(data: Vec<f64>) -> Option<Calc> {
        if data.is_empty() {
            return None;
        }

        let mut sorted = data.clone();
        let mut calc = Calc {
            data: data.clone(),
            len: data.len(),
            ..Default::default()
        };

        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        calc.sorted = sorted;

        if data.len() % 2 != 0 {
            calc.even = false;
        } else {
            calc.even = true;
        }

        calc.middle_index = calc.len / 2;
        Some(calc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_mean() {
        let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
        ])
        .unwrap();
        assert!(approx_eq!(f64, calc.mean(), 0.24905562997351_f64, epsilon = 0.00000003, ulps = 2));
    }

    #[test]
    fn test_median() {
        let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
        ])
        .unwrap();
        assert!(approx_eq!(f64, calc.median(), 0.2382140377175458_f64, epsilon = 0.00000003, ulps = 2));
    }
    #[test]
    fn test_sorted_median() {
        let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
        ])
        .unwrap();
        assert!(approx_eq!(f64, calc.sorted_median(), 0.2382140377175458_f64, epsilon = 0.00000003, ulps = 2));
    }
    #[test]
    fn test_mode() {
        let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
        ])
        .unwrap();
        assert!(approx_eq!(f64, calc.mode(), 0.2041683180134608_f64, epsilon = 0.00000003, ulps = 2 ));
    }
    #[test]
    fn test_range() {
        let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
        ])
        .unwrap();
        assert!(approx_eq!(f64, calc.range(), 0.44750014793171_f64,epsilon = 0.00000003, ulps = 2));
    }
    #[test]
    fn test_variance() {
        let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
        ])
        .unwrap();
        assert!(approx_eq!(f64, calc.variance(), 0.020777812166056_f64, epsilon = 0.00000003, ulps = 2));
    }
    #[test]
    fn test_standart_deviation() {
        let mut calc = Calc::new(vec![
            0.4814386506837457,
            0.0339385027520397,
            0.2382140377175458,
            0.2875186407007349,
            0.2041683180134608,
        ])
        .unwrap();                           
        assert!(approx_eq!(f64, calc.standart_deviation(), 0.14414510801985_f64,epsilon = 0.00000003, ulps = 2));
    }
}
