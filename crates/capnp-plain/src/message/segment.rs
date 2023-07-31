use super::word::Word;

pub struct Segment {
    pub words: Vec<Word>,
}

impl Segment {
    pub fn from_bytes(input: &[u8]) -> Self {
        let a = input.chunks_exact(8);
        let words = a.map(|x| x.try_into().unwrap()).collect();
        Segment { words }
    }
}
