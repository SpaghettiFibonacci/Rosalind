// Counting DNA Nucleotides
pub struct Dna;
/// Count DNA nucleotides in a string
pub fn count_dna_nucleotides(dna: &str) -> (u32, u32, u32, u32) {
    let mut a = 0;
    let mut c = 0;
    let mut g = 0;
    let mut t = 0;
    for n in dna.chars() {
        match n {
            'A' => a += 1,
            'C' => c += 1,
            'G' => g += 1,
            'T' => t += 1,
            _ => (),
        }
    }
    (a, c, g, t)
}
