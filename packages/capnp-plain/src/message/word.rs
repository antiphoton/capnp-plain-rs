pub mod word_ref;
pub mod word_slice;

use anyhow::Error;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Word(pub [u8; 8]);

impl Word {
    pub fn is_zero(&self) -> bool {
        self.0 == [0; 8]
    }
    pub fn from_bytes(bytes: &[u8]) -> Vec<Self> {
        bytes
            .chunks_exact(8)
            .map(|x| {
                let x: [u8; 8] = x.try_into().unwrap();
                Word(x)
            })
            .collect()
    }
    pub fn from_packed_bytes(input: &[u8]) -> Vec<Self> {
        let mut output = vec![];
        let mut input = input.iter().cloned();
        while let Some(tag) = input.next() {
            match tag {
                0 => {
                    let n = input.next().unwrap() + 1;
                    for _ in 0..n {
                        output.push(Word([0; 8]));
                    }
                }
                0xff => {
                    output.push(take_word(&mut input));
                    let n = input.next().unwrap();
                    for _ in 0..n {
                        output.push(take_word(&mut input));
                    }
                }
                _ => {
                    let y: Vec<_> = (0..8)
                        .map(|i| {
                            if (tag & (1 << i)) > 0 {
                                input.next().unwrap()
                            } else {
                                0
                            }
                        })
                        .collect();
                    output.push(Word(y.try_into().unwrap()));
                }
            }
        }
        output
    }
    pub fn to_packed_bytes(input: &[u8]) -> Vec<u8> {
        let mut output = vec![];
        for word in input.chunks_exact(8) {
            let mut tag = 0u8;
            let mut y = Vec::with_capacity(8);
            for (i, x) in word.iter().enumerate() {
                if *x > 0 {
                    tag |= 1 << i;
                    y.push(*x)
                }
            }
            output.push(tag);
            if tag == 0 {
                output.push(0);
            }
            output.append(&mut y);
            if tag == 0xff {
                output.push(0);
            }
        }
        output
    }
}

fn take_word(input: &mut impl Iterator<Item = u8>) -> Word {
    let bytes: Vec<u8> = input.by_ref().take(8).collect();
    Word(bytes.try_into().unwrap())
}

impl TryFrom<&[u8]> for Word {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let a: &[u8; 8] = value.try_into()?;
        Ok(Word(*a))
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a: Vec<_> = self.0.iter().map(|x| format!("{:02x}", x)).collect();
        write!(f, "[{}]", a.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packing() {
        assert_eq!(
            Word::to_packed_bytes(&[8, 0, 0, 0, 3, 0, 2, 0, 0x19, 0, 0, 0, 0xaa, 1, 0, 0]),
            vec![0x51, 8, 3, 2, 0x31, 0x19, 0xaa, 1]
        );
        assert_eq!(Word::to_packed_bytes(&[0; 16]), vec![0, 0, 0, 0]);
        assert_eq!(
            Word::to_packed_bytes(&[0x8a; 8]),
            vec![0xff, 0x8a, 0x8a, 0x8a, 0x8a, 0x8a, 0x8a, 0x8a, 0x8a, 0]
        );
    }
}
