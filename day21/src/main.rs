use std::collections::HashMap;

fn main() {
    let cars = vec![
        "volvo", "toyota", "volvo", "ferrari", "bmw", "bmw", "ferrari", "ferrari", "toyota",
    ];
    let counted = count_in_loop(&cars);
    println!("{:#?}", counted);
    let vowel_consonant = count_vowel_and_consonant(&cars);
    println!("{:#?}", vowel_consonant);
}

fn count_in_loop<'a>(words: &'a [&'a str]) -> HashMap<&'a str, u32> {
    let mut counter: HashMap<&'a str, u32> = HashMap::new();
    for word in words {
        *counter.entry(word).or_insert(0) += 1;
    }
    counter
}

fn count_vowel_and_consonant<'a>(words: &'a [&'a str]) -> HashMap<&'a str, HashMap<&'a str, i32>> {
    let mut vowel_consonant: HashMap<&'a str, HashMap<&'a str, i32>> = HashMap::new();
    let mut counted: HashMap<&'a str, i32> = HashMap::new();

    for word in words {
        let mut count_vowel = 0;
        let mut count_consonant = 0;
        for c in word.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => count_vowel += 1,
                'a'..='z' | 'A'..='Z' => count_consonant += 1,
                _ => {}
            }
        }
        *counted.entry("vowel").or_insert(0) = count_vowel;
        *counted.entry("consonant").or_insert(0) = count_consonant;
        *vowel_consonant.entry(word).or_insert(HashMap::new()) = counted.clone();
    }
    vowel_consonant
}
