fn main() {
    let input = "first apple zebra exercise";
    let pig_latin = convert_to_pig_latin(input);
    println!("{}", pig_latin);
}

fn convert_to_pig_latin(input: &str) -> String {
    input.split_whitespace()
         .map(|word| convert_word_to_pig_latin(word))
         .collect::<Vec<_>>()
         .join(" ")
}

fn convert_word_to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let first_char = word.chars().next().unwrap();

    if vowels.contains(&first_char) {
        format!("{}-hay", word)
    } else {
        let mut chars = word.chars();
        let first_consonant = chars.next().unwrap();
        let remaining_chars: String = chars.collect();
        format!("{}-{}ay", remaining_chars, first_consonant)
    }
}
