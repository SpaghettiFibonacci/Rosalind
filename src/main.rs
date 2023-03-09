use challenges::get_result;

pub mod challenges;
pub mod io;
fn main() {
    // get_result(challenges::dna::Dna);
    get_result(challenges::rna::Rna);
}
