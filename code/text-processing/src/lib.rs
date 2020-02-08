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

#[cfg(test)]
mod tests {
    use crate::{split_stuff, split_stuff_with_patterns};

    #[test]
    fn test_split_stuff() {
        split_stuff();
    }

    #[test]
    fn test_split_stuff_with_patterns() {
        split_stuff_with_patterns();
    }
}
