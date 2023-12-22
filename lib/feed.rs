use std::io;
use std::convert::From;
use atom_syndication as atom;
use rss;
use chrono::{self, offset::FixedOffset};

pub(crate) fn atom_feed_from_rss_channel(channel: &rss::Channel) -> io::Result<atom::Feed> {
    Ok(atom::FeedBuilder::default()
        .title(channel.title.clone())
        .link(atom::LinkBuilder::default()
            .href(channel.link.clone())
            .build())
        .subtitle(match channel.description.as_str() {
            "" => None,
            description => Some(atom::TextBuilder::default()
                .value(description.to_string())
                .build()),
        })
        .entries(channel.items.clone().into_iter().map_while(|i| atom_entry_from_rss_item(i).ok()).collect::<Vec<atom::Entry>>())
        .build())
}

fn atom_entry_from_rss_item(item: rss::Item) -> io::Result<atom::Entry> {
    Ok(atom::EntryBuilder::default()
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
        .build())
}
