fn main() {
    // This doesn't work very well, but it is funny.
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
            if l.len() % 2 == 0 {
                "functional"
            } else if l.eq("haskell") {
                "functional"
            } else {
                "imperative"
            });
    }
}
