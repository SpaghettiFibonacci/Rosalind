use super::Runnable;

// Complementing a Strand of DNA
pub struct Revc;

impl Runnable for Revc {
    fn execute(&self, input: &str) -> String {
        Self::complement_a_strand_of_dna(input)
    }
}

impl Revc {
    fn complement_a_strand_of_dna(dna: &str) -> String {
        dna.chars()
            .map(|c| match c {
                'A' => 'T',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => c,
            })
            .rev()
            .collect::<String>()
    }
}
