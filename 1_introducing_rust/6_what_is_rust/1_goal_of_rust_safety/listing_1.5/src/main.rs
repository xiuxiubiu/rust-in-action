fn main() {
    let fruit = vec!['ð', 'ð', 'ð'];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, 'ðsh');
}
