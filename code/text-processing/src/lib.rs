use regex;
use twoway;

fn split_stuff() {
    let message = "Hello there | how are you ; doing?";
    for s in message.split(|c| (c == '|') || (c == ';')) {
        dbg!(s);
    }
}

fn split_stuff_with_patterns() {
    let message = "Hello there | how are you ; doing?";
    for s in message.split(&['|', ';'][..]) {
        dbg!(s);
    }
}

fn split_by_regex() {
    let message = "hello there; how|    are|you, Chathura";

    let re = regex::Regex::new(r"[;,|\s]\s*").unwrap();

    for s in re.split(message) {
        dbg!(s);
    }
}

fn search_by_twoway() {
    let loc = twoway::find_str("hello there how are you?", "there");
    dbg!(loc);
}

fn string_starts_or_ends_with() {
    let s = "Hello there";

    dbg!(s.starts_with("Hello"));
    dbg!(s.ends_with("there"));
}

fn string_to_vecs_and_back() {
    let s = "Hello there";

    // Convert a string to a Vec<char>
    let cvec: Vec<char> = s.chars().collect();

    // Convert a Vec<char> into a String
    let back: String = cvec.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_split_stuff() {
        split_stuff();
    }

    #[test]
    fn test_split_stuff_with_patterns() {
        split_stuff_with_patterns();
    }

    #[test]
    fn test_split_by_regex() {
        split_by_regex();
    }

    #[test]
    fn test_search_by_twoway() {
        search_by_twoway();
    }

    #[test]
    fn test_string_starts_or_ends_with() {
        string_starts_or_ends_with();
    }
}
