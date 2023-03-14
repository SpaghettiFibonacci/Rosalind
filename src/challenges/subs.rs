use super::Runnable;

// Finding a Motif in DNA
pub struct Subs;

impl Runnable for Subs {
    fn execute(&self, input: &str) -> String {
        Self::find_motif_dna(input)
    }
}

impl Subs {
    /// Finding a Motif in DNA
    fn find_motif_dna(input: &str) -> String {
        let mut index_vec = Vec::new();
        let s = input.lines().collect::<Vec<&str>>()[0];
        let t = input.lines().collect::<Vec<&str>>()[1];
        for i in 0..s.len() {
            if s[i..].starts_with(t) {
                index_vec.push(i + 1);
            }
        }
        index_vec
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
