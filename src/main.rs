use nlp::alignment_strings;
use nlp::graphemes_struct::Graphemes;
fn main() {

    let strings = alignment_strings(&intention, &execution, 1, " ");
    println!("{}\n{}", strings[0], strings[1])
}
