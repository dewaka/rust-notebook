use regex;

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
}
