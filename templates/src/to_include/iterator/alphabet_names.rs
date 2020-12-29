// This iterator yeilds all names in lexicografical order: 'a', 'b', ..., 'z', 'aa', 'ab', ... . This is infinite iterator
struct AlphabeticalNamesIterator {
    letters: Vec<u8>,
}
 
impl AlphabeticalNamesIterator {
    fn new() -> Self {
        Self {
            letters: vec![0],
        }
    }
}
 
impl Iterator for AlphabeticalNamesIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        const SIZE: u8 = b'z' - b'a';
        let result = self.letters.iter().rev().map(|x| char::from(b'a' + x)).collect::<String>();
 
        let expand = self.letters
            .iter_mut()
            .scan((), |(), x| {
                *x += 1;
                if *x > SIZE {
                    *x = 0;
                    Some(())
                } else {
                    None
                }
            })
            .count() == self.letters.len();
 
        if expand {
            self.letters.push(0);
        }
 
        Some(result)
    }
}
