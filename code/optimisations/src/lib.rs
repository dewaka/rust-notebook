use crossbeam_utils::CachePadded;

struct Counters {

}

fn foo() {
    let array = [CachePadded::new(1i8), CachePadded::new(2i8)];
    let addr1 = &*array[0] as *const i8 as usize;
    let addr2 = &*array[1] as *const i8 as usize;

    assert!(addr2 - addr1 >= 64);
    assert_eq!(addr1 % 64, 0);
    assert_eq!(addr2 % 64, 0);
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn it_works() {
        foo();
    }
}
