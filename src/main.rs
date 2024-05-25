use std::fs::File;
use std::io::{self, BufRead};

fn load_dictionary(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn wagner_fischer(s1: &str, s2: &str) -> usize {
    let (len_s1, len_s2) = (s1.len(), s2.len());
    let (s1, s2) = if len_s1 > len_s2 { (s2, s1) } else { (s1, s2) };
    let (len_s1, len_s2) = (s1.len(), s2.len());

    let mut current_row: Vec<usize> = (0..=len_s1).collect();
    for i in 1..=len_s2 {
        let previous_row = current_row.clone();
        current_row = vec![i; len_s1 + 1];
        for j in 1..=len_s1 {
            let add = previous_row[j] + 1;
            let delete = current_row[j - 1] + 1;
            let mut change = previous_row[j - 1];
            if s1.as_bytes()[j - 1] != s2.as_bytes()[i - 1] {
                change += 1;
            }
            current_row[j] = add.min(delete).min(change);
        }
    }

    current_row[len_s1]
}

fn spell_check(word: &str, dictionary: &[String]) -> Vec<(String, usize)> {
    let mut suggestions = Vec::new();

    if dictionary.contains(&word.to_string()) {
        return suggestions;
    }

    for correct_word in dictionary {
        let distance = wagner_fischer(word, correct_word);
        suggestions.push((correct_word.clone(), distance));
    }

    suggestions.sort_by_key(|&(_, distance)| distance);
    suggestions.truncate(10);
    suggestions
}

fn main() -> io::Result<()> {
    let dictionary = load_dictionary("words.txt")?;
    let mut input_word = String::new();
    println!("Enter your word:");
    io::stdin().read_line(&mut input_word)?;

    let input_word = input_word.trim();

    let suggestions = spell_check(&input_word, &dictionary);

    if suggestions.is_empty() {
        println!("Word entered is correct");
    } else {
        println!("Top 10 suggestions for '{}':", input_word);
        for (word, distance) in suggestions {
            println!("{} (Distance: {})", word, distance);
        }
    }

    Ok(())
}