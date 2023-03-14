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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcribing_dna_into_rna() {
        let rna = Rna {};
        assert_eq!(
            rna.execute("GATGGAACTTGACTACGTAAATT"),
            "GAUGGAACUUGACUACGUAAAUU"
        );
        assert_eq!(rna.execute("TTTT"), "UUUU");
        assert_eq!(rna.execute(""), "");
    }
}
