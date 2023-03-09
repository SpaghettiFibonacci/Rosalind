use super::Runnable;

// Counting DNA Nucleotides
pub struct Dna;

impl Runnable for Dna {
    fn execute(&self, input: &str) -> String {
        let (a, c, g, t) = Self::count_dna_nucleotides(input);
        println!("{} {} {} {}", a, c, g, t);
        String::from("Done")
    }
}

impl Dna {
    /// Count DNA nucleotides in a string
    fn count_dna_nucleotides(dna: &str) -> (u32, u32, u32, u32) {
        let mut a = 0;
        let mut c = 0;
        let mut g = 0;
        let mut t = 0;
        for n in dna.chars() {
            match n {
                'A' => a += 1,
                'C' => c += 1,
                'G' => g += 1,
                'T' => t += 1,
                _ => (),
            }
        }
        (a, c, g, t)
    }
}
