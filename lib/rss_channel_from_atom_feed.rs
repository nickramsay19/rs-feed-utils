use std::io;
use std::cmp;
use std::convert;
use atom_syndication as atom;
use rss;

#[derive(Clone, cmp::PartialEq)]
pub struct RSSChannelFromAtomFeed(pub rss::Channel);
impl RSSChannelFromAtomFeed {
    pub fn unwrap(RSSChannelFromAtomFeed(channel): Self) -> rss::Channel {
        channel
    }
}
impl convert::TryFrom<atom::Feed> for RSSChannelFromAtomFeed {
    type Error = io::Error;

    fn try_from(value: atom::Feed) -> io::Result<RSSChannelFromAtomFeed> {
        Err(io::Error::from(io::ErrorKind::Other))
    }
}
