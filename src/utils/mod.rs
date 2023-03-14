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
        "UAA" | "UAG" | "UGA" => "",
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
        _ => "",
    }
    .to_string()
}
