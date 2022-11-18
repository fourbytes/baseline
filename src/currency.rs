use std::fmt::{Display, Debug};

use smol_str::SmolStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pair {
    base: SmolStr,
    quote: SmolStr
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}/{}", self.base, self.quote))
    }
}

impl Pair {
    pub fn new<S>(base: S, quote: S) -> Self 
    where S: Into<SmolStr> {
        Self {
            base: base.into(),
            quote: quote.into(),
        }
    }

    pub fn base(&self) -> &SmolStr {
        &self.base
    }

    pub fn quote(&self) -> &SmolStr {
        &self.quote
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashSet, BTreeSet};

    use super::Pair;

    #[test]
    fn test_display() {
        let id = Pair::new("GBP", "USD");
        assert_eq!(id.to_string(), "GBP/USD");
    }

    #[test]
    fn test_hash() {
        let mut set = HashSet::<Pair>::new();
        set.insert(Pair::new("GBP", "USD"));
    }

    #[test]
    fn test_btree() {
        let mut set = BTreeSet::<Pair>::new();
        set.insert(Pair::new("GBP", "USD"));
    }
}
