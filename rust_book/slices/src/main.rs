fn main() {

    let mut s = String::from("hello degens");

    let word = first_word(&s);

    s.clear();

    println!("word: {}", word);

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}