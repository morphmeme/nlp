use nlp::alignment_strings;
use nlp::graphemes_struct::Graphemes;
use nlp::metrics::levenshtein_distance;

fn main() {
    let intention = vec![String::from("intention")];
    let execution = vec![String::from("execution")];
    let strings = levenshtein_distance(&intention, &execution, 1);
    println!("{}", strings)
}
