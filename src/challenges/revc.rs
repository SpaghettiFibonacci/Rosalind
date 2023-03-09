use super::Runnable;

// Complementing a Strand of DNA
pub struct Revc;

impl Runnable for Revc {
    fn execute(&self, input: &str) -> String {
        Self::transcribing_dna_into_rna(input)
    }
}

impl Revc {
    fn transcribing_dna_into_rna(dna: &str) -> String {
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
