use super::Runnable;

// Counting Point Mutations
pub struct Hamm;

impl Runnable for Hamm {
    fn execute(&self, input: &str) -> String {
        Self::counting_point_mutations(input)
    }
}

impl Hamm {
    /// counting point mutations
    fn counting_point_mutations(input: &str) -> String {
        let split = input.split_whitespace().collect::<Vec<&str>>();
        let mut count = 0;
        for (i, c) in split[0].chars().enumerate() {
            if c != split[1].chars().nth(i).unwrap() {
                count += 1;
            }
        }
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_point_mutations() {
        let input = "GAGCCTACTAACGGGAT\nCATCGTAATGACGGCCT";
        let result = Hamm::counting_point_mutations(input);
        assert_eq!(result, "7");
    }
}
