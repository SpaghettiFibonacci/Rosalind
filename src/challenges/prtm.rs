use crate::utils::get_ammino_acid_residue_weight;

use super::Runnable;

// Calculating Protein Mass
pub struct Prtm;

impl Runnable for Prtm {
    fn execute(&self, input: &str) -> String {
        Self::find_motif_dna(input)
    }
}

impl Prtm {
    /// Calculating Protein Mass
    fn find_motif_dna(input: &str) -> String {
        let mut total_weight = 0f64;
        for i in input.chars() {
            total_weight += get_ammino_acid_residue_weight(i.to_string())
        }
        format!("{:.3}", total_weight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_motif_dna() {
        let input = "SKADYEK";
        let expected = "821.392";

        assert_eq!(expected, Prtm::find_motif_dna(input));
    }
}
