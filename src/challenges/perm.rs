use super::Runnable;

// Enumerating gene orders
// TODO: make it work for 1,2
pub struct Perm;

impl Runnable for Perm {
    fn execute(&self, input: &str) -> String {
        Self::enumerating_gene_orders(input)
    }
}

impl Perm {
    /// Enumerate gene orders
    fn enumerating_gene_orders(input: &str) -> String {
        let n = input.parse::<usize>().unwrap();
        let mut v = Vec::new();
        for i in 1..=n {
            v.push(i);
        }
        let mut perm_string = String::from("");

        // create permutations from v
        let perms = Self::create_permutations_from_loop(&v);
        perm_string = format!("{}{}\r\n", perm_string, perms.len());
        // rebuild for string
        for perm in perms {
            let mut item_string = String::from("");
            for i in perm {
                item_string = format!("{}{} ", item_string, i);
            }
            perm_string = format!("{}{}\r\n", perm_string, item_string.trim());
        }
        perm_string.trim().to_string()
    }

    fn generate_permutations(data: &mut [usize], start: usize, result: &mut Vec<Vec<usize>>) {
        if start == data.len() {
            result.push(data.to_vec());
        } else {
            for i in start..data.len() {
                data.swap(start, i);
                Self::generate_permutations(data, start + 1, result);
                data.swap(start, i);
            }
        }
    }

    fn create_permutations_from_loop(data: &Vec<usize>) -> Vec<Vec<usize>> {
        let mut permutations = Vec::new();
        let mut data = data.clone();
        Self::generate_permutations(&mut data, 0, &mut permutations);
        permutations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerating_gene_orders() {
        let input = "3";
        let result = Perm::enumerating_gene_orders(input);
        assert_eq!(
            result,
            "6\r\n1 2 3\r\n1 3 2\r\n2 1 3\r\n2 3 1\r\n3 2 1\r\n3 1 2"
        );
    }
}
