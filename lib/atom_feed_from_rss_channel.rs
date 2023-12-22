use std::io;
use atom_syndication as atom;
use rss;
use chrono::{self, offset::FixedOffset};
use std::cmp;
use std::convert;

#[derive(Clone, cmp::PartialEq)]
pub struct AtomFeedFromRSSChannel(pub atom::Feed);
impl AtomFeedFromRSSChannel {
    pub fn unwrap(AtomFeedFromRSSChannel(feed): Self) -> atom::Feed {
        feed
    }
}
impl convert::TryFrom<rss::Channel> for AtomFeedFromRSSChannel {
    type Error = io::Error;

    fn try_from(value: rss::Channel) -> io::Result<AtomFeedFromRSSChannel > {
        Ok(AtomFeedFromRSSChannel(
            atom::FeedBuilder::default()
                .title(value.title.clone())
                .link(
                    atom::LinkBuilder::default()
                        .href(value.link.clone())
                        .build()
                )
                .subtitle(match value.description.as_str() {
                    "" => None,
                    description => Some(
                        atom::TextBuilder::default()
                            .value(description.to_string())
                            .build()
                    ),
                })
                .entries(value.items.clone().into_iter().map_while(|i| atom_entry_from_rss_item(i).ok()).collect::<Vec<atom::Entry>>())
                .build()
        ))
    }
}
impl convert::Into<atom::Feed> for AtomFeedFromRSSChannel {
    fn into(self) -> atom::Feed {
        let AtomFeedFromRSSChannel(feed) = self;
        feed
    }
}

fn atom_entry_from_rss_item(item: rss::Item) -> io::Result<atom::Entry> {
    Ok(
        atom::EntryBuilder::default()
            .title(item.title.unwrap_or_default())  
            .id(item.guid.unwrap_or_default().value)
            .updated(match item.pub_date {
                Some(s) => chrono::DateTime::<FixedOffset>::parse_from_rfc2822(s.as_str()).unwrap_or_default(),
                None => chrono::DateTime::<FixedOffset>::default(),
            })
            .content(item.content.map(|s| atom::ContentBuilder::default().value(Some(s)).build()))
            .links(match item.link {
                Some(l) => vec![atom::LinkBuilder::default().href(l).build()],
                None => vec![],
            })
            .build()
    )
}

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
