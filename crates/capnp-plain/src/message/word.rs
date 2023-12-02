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
