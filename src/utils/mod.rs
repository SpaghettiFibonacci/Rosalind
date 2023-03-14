use std::collections::HashMap;

pub fn rna_codon_to_ammino_acid(codon: &str) -> String {
    match codon {
        "UUU" | "UUC" => "F",
        "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => "L",
        "AUU" | "AUC" | "AUA" => "I",
        "AUG" => "M",
        "GUU" | "GUC" | "GUA" | "GUG" => "V",
        "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => "S",
        "CCU" | "CCC" | "CCA" | "CCG" => "P",
        "ACU" | "ACC" | "ACA" | "ACG" => "T",
        "GCU" | "GCC" | "GCA" | "GCG" => "A",
        "UAU" | "UAC" => "Y",
        "UAA" | "UAG" | "UGA" => "Stop",
        "CAU" | "CAC" => "H",
        "CAA" | "CAG" => "Q",
        "AAU" | "AAC" => "N",
        "AAA" | "AAG" => "K",
        "GAU" | "GAC" => "D",
        "GAA" | "GAG" => "E",
        "UGU" | "UGC" => "C",
        "UGG" => "W",
        "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => "R",
        "GGU" | "GGC" | "GGA" | "GGG" => "G",
        _ => "Error",
    }
    .to_string()
}

pub fn get_ammino_acid_residue_weight(ammino_acid: String) -> f64 {
    match ammino_acid.as_str() {
        "A" => 71.03711,
        "C" => 103.00919,
        "D" => 115.02694,
        "E" => 129.04259,
        "F" => 147.06841,
        "G" => 57.02146,
        "H" => 137.05891,
        "I" => 113.08406,
        "K" => 128.09496,
        "L" => 113.08406,
        "M" => 131.04049,
        "N" => 114.04293,
        "P" => 97.05276,
        "Q" => 128.05858,
        "R" => 156.10111,
        "S" => 87.03203,
        "T" => 101.04768,
        "V" => 99.06841,
        "W" => 186.07931,
        "Y" => 163.06333,
        _ => 0.0,
    }
}

pub fn fasta_to_hashmap(fasta_string: String) -> HashMap<String, String> {
    let labels = fasta_string
        .lines()
        .filter(|x| x.starts_with('>'))
        .map(|x| x.replace('>', ""))
        .collect::<Vec<String>>();
    let dna_vec_from_multiple_fasta_entries = fasta_string
        .split('>')
        .map(|x| x.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .filter(|x| x.len() > 1)
        .map(|x| x[1..].join(""))
        .collect::<Vec<String>>();

    labels
        .iter()
        .zip(dna_vec_from_multiple_fasta_entries.iter())
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<String, String>>()
}
