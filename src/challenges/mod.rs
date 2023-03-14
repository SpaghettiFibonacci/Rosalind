use crate::io::manage_dataset::{load_file, save_file};

pub mod dna;
pub mod gc;
pub mod hamm;
pub mod iprb;
pub mod perm;
pub mod revc;
pub mod rna;
pub trait Runnable {
    fn execute(&self, input: &str) -> String;
    fn run(&self) {
        let input = load_file("data/input.txt").unwrap();
        save_file("data/output.txt", &self.execute(&input)).unwrap();
    }
}

pub fn get_result<T: Runnable>(assignment: T) {
    assignment.run();
}
