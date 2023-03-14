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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complement_a_strand_of_dna() {
        let dna = "GTCAATG";
        let expected = "CATTGAC".to_string();
        assert_eq!(Revc::complement_a_strand_of_dna(dna), expected);

        let dna = "AGTC";
        let expected = "GACT".to_string();
        assert_eq!(Revc::complement_a_strand_of_dna(dna), expected);

        // Test with invalid DNA characters
        let dna_with_invalid_chars = "AGTXCAN";
        assert_ne!(
            Revc::complement_a_strand_of_dna(dna_with_invalid_chars),
            dna_with_invalid_chars.to_string()
        );
    }
}
