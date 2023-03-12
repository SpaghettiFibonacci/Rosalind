use super::Runnable;

// Mendel's First Law
pub struct Iprb;

impl Runnable for Iprb {
    fn execute(&self, input: &str) -> String {
        Self::mendels_first_law(input)
    }
}

impl Iprb {
    /// Mendel's First Law
    fn mendels_first_law(input: &str) -> String {
        let mut input = input.split_whitespace();
        let k = input.next().unwrap().parse::<f64>().unwrap();
        let m = input.next().unwrap().parse::<f64>().unwrap();
        let n = input.next().unwrap().parse::<f64>().unwrap();
        let total = k + m + n;
        let mut result = 0.0;
        // k
        result += k / total;
        // m
        result += m / total * (k / (total - 1.0));
        result += m / total * ((m - 1.0) / (total - 1.0)) * 0.75;
        result += m / total * (n / (total - 1.0)) * 0.5;
        // n
        result += n / total * (k / (total - 1.0));
        result += n / total * (m / (total - 1.0)) * 0.5;
        format!("{:.5}", result)
    }
}
