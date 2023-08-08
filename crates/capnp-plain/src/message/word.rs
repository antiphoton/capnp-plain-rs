pub mod word_ref;
pub mod word_slice;

use anyhow::Error;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Word(pub [u8; 8]);

impl Word {
    pub fn is_zero(&self) -> bool {
        self.0 == [0; 8]
    }
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
