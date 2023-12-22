use std::cmp;
use atom_syndication as atom;

#[derive(cmp::PartialEq)]
pub struct OrdAtomEntry(pub atom::Entry);
impl cmp::PartialOrd for OrdAtomEntry {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let OrdAtomEntry(lhs) = self;
        let OrdAtomEntry(rhs) = other;

        lhs.updated.partial_cmp(&rhs.updated)
    }
}
impl cmp::Eq for OrdAtomEntry {}
impl cmp::Ord for OrdAtomEntry {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.partial_cmp(other) {
            Some(c) => c,
            None => cmp::Ordering::Equal,
        }
    }
}
