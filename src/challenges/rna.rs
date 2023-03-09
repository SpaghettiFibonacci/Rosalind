use super::Runnable;

// Transcribing DNA into RNA
pub struct Rna;

impl Runnable for Rna {
    fn execute(&self, input: &str) -> String {
        Self::transcribing_dna_into_rna(input)
    }
}

impl Rna {
    /// Transcribing DNA into RNA
    fn transcribing_dna_into_rna(dna: &str) -> String {
        dna.replace('T', "U")
    }
}
