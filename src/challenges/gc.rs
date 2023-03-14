use std::collections::HashMap;

use crate::utils::fasta_to_hashmap;

use super::Runnable;

// Computing GC Content
pub struct Gc;

impl Runnable for Gc {
    fn execute(&self, input: &str) -> String {
        Self::compute_gc_content(input)
    }
}

impl Gc {
    /// Computing GC Content
    fn compute_gc_content(input: &str) -> String {
        let mut highest_gc_content = 0.0;
        let mut highest_gc_content_label = "".to_string();
        let map = fasta_to_hashmap(input.to_string());
        let mut gc_content_map = HashMap::new();

        for (k, v) in map {
            let gc_content = v.chars().filter(|x| x == &'G' || x == &'C').count() as f64
                / v.len() as f64
                * 100.0;
            gc_content_map.insert(k, gc_content);
        }

        for (k, v) in gc_content_map {
            if v > highest_gc_content {
                highest_gc_content = v;
                highest_gc_content_label = k;
            }
        }

        format!("{}\r\n{:.6}", highest_gc_content_label, highest_gc_content)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_compute_gc_content() {
        let input = ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT";
        assert_eq!(
            Gc::compute_gc_content(input),
            "Rosalind_0808\r\n60.919540".to_string()
        );
    }
}
