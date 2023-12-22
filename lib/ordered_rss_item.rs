use std::cmp;
use rss;
use chrono;

#[derive(cmp::PartialEq)]
pub struct OrdRSSItem(pub rss::Item);
impl cmp::PartialOrd for OrdRSSItem {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let OrdRSSItem(lhs) = self;
        let OrdRSSItem(rhs) = other;

        let lhs_pub_date = chrono::DateTime::parse_from_rfc2822(lhs.pub_date.clone()?.as_str()).ok()?;
        let rhs_pub_date = chrono::DateTime::parse_from_rfc2822(rhs.pub_date.clone()?.as_str()).ok()?;

        Some(lhs_pub_date.cmp(&rhs_pub_date))    
    }
}
impl cmp::Eq for OrdRSSItem {}
impl cmp::Ord for OrdRSSItem {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.partial_cmp(other) {
            Some(c) => c,
            None => cmp::Ordering::Equal,
        }
    }
}
