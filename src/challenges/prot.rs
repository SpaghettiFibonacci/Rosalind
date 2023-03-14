use crate::utils::rna_codon_to_ammino_acid;

use super::Runnable;

// Translating RNA into Protein
pub struct Prot;

impl Runnable for Prot {
    fn execute(&self, input: &str) -> String {
        Self::translating_rna_to_protein(input)
    }
}

impl Prot {
    /// Translating RNA into Protein
    fn translating_rna_to_protein(input: &str) -> String {
        let rna_codons = input
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut protein = String::new();

        for codon in rna_codons {
            protein = format!("{}{}", protein, rna_codon_to_ammino_acid(&codon));
        }

        protein
    }
}
