use std::fmt::{Display, Debug};

use smol_str::SmolStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
    /// Market Identification Code
    mic: SmolStr,

    /// Ticker Symbol
    symbol: SmolStr
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}:{}", self.mic, self.symbol))
    }
}

impl Identifier {
    pub fn new<S>(mic: S, symbol: S) -> Self
    where S: Into<SmolStr> {
        Self {
            mic: mic.into(),
            symbol: symbol.into(),
        }
    }

    #[must_use] pub fn mic(&self) -> &SmolStr {
        &self.mic
    }

    #[must_use] pub fn symbol(&self) -> &SmolStr {
        &self.symbol
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashSet, BTreeSet};

    use super::Identifier;

    #[test]
    fn test_display() {
        let id = Identifier::new("XASX", "ANZ");
        assert_eq!(id.to_string(), "XASX:ANZ");
    }

    #[test]
    fn test_hash() {
        let mut set = HashSet::<Identifier>::new();
        set.insert(Identifier::new("XASX", "ANZ"));
    }

    #[test]
    fn test_btree() {
        let mut set = BTreeSet::<Identifier>::new();
        set.insert(Identifier::new("XASX", "ANZ"));
    }
}
