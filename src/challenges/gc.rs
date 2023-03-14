use std::collections::HashMap;

use super::Runnable;

// Computing GC Content
// TODO: make it work for 1,2
pub struct Gc;

impl Runnable for Gc {
    fn execute(&self, input: &str) -> String {
        Self::compute_gc_content(input)
    }
}

impl Gc {
    /// Computing GC Content
    fn compute_gc_content(input: &str) -> String {
        let labels = input
            .lines()
            .filter(|x| x.starts_with('>'))
            .map(|x| x.replace('>', ""))
            .collect::<Vec<String>>();
        let dna_vec_from_multiple_fasta_entries = input
            .split('>')
            .map(|x| x.lines().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
            .iter()
            .filter(|x| x.len() > 1)
            .map(|x| x[1..].join(""))
            .collect::<Vec<String>>();

        let map = labels
            .iter()
            .zip(dna_vec_from_multiple_fasta_entries.iter())
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>();

        let mut gc_content_map = HashMap::new();

        for (k, v) in map {
            let gc_content = v.chars().filter(|x| x == &'G' || x == &'C').count() as f64
                / v.len() as f64
                * 100.0;
            gc_content_map.insert(k, gc_content);
        }

        let mut highest_gc_content = 0.0;
        let mut highest_gc_content_label = "".to_string();
        for (k, v) in gc_content_map {
            if v > highest_gc_content {
                highest_gc_content = v;
                highest_gc_content_label = k;
            }
        }

        format!("{}\r\n{:.6}", highest_gc_content_label, highest_gc_content)
    }
}
