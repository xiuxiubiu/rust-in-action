fn main() {
    let mut letters = vec!["a", "b", "c"];
    let p = &mut letters;

    for letter in p.iter().enumerate() {
        // println!("{}", letter);
        letters.push(*letter.1);
    }
}
