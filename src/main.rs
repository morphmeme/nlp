use nlp::Graphemes;

fn main() {
    let word : Graphemes = Graphemes::new("§uper");
    println!("{:?}", word);
    //dbg!(levenshtein_distance("kitten", "sitting"));
}
