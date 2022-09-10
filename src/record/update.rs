use std::{fmt::Display, mem::{Discriminant, discriminant}, str::FromStr};

use crate::ParseError;

use super::Property;

use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct Update {
    pub id: u64,
    pub props: FxHashMap<Discriminant<Property>, Property>,
}

impl FromStr for Update {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, mut rest) = line.split_once(',').ok_or(ParseError::Eol)?;
        let id = u64::from_str_radix(id, 16)?;
        let mut props = FxHashMap::default();

        let mut prev = None;
        let mut offset = 0;
        for (i, ch) in rest.char_indices() {
            if ch == ',' && prev != Some('\\') {
                let (kv, r) = rest.split_at(i - offset);
                rest = r.strip_prefix(',').unwrap_or(rest);
                offset = i + 1;

                let p = Property::from_str(kv)?;
                props.insert(discriminant(&p), p);
            }

            prev = Some(ch);
        }

        if !rest.is_empty() {
                let p = Property::from_str(rest)?;
                props.insert(discriminant(&p), p);
        }

        Ok(Update { id, props })
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.id)?;
        for p in self.props.values() {
            write!(f, ",{}", p)?;
        }
        Ok(())
    }
}
