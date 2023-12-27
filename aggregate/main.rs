use std::io::{self, prelude::*};
use std::net;
use std::cmp;
use std::collections::BinaryHeap;
use std::fs;
use atom_syndication as atom;
use rss;
use chrono;
use lib::{OrdAtomEntry, AtomFeedFromRSSChannel};

/// creates atom feed, or converts rss channel to an atom feed
fn read_feed(reader: impl io::BufRead) -> io::Result<atom::Feed> {
    let mut cursor = io::Cursor::new(reader);

    match atom::Feed::read_from(cursor.get_mut()) {
        Ok(feed) => Ok(feed),
        Err(_) => {
            cursor.set_position(0);

            let channel = rss::Channel::read_from(cursor.get_mut()).map_err(io::Error::other)?;
            
            Ok(AtomFeedFromRSSChannel::try_from(channel)?.into())
        },
    }
}

fn main() -> io::Result<()> {

    // create a binary heap, a priority queue of rss items ordered by date
    let mut entries: Vec<atom::Entry> = Vec::new();

    // read rss feed contents from stdin
    while let Ok(feed) = read_feed(io::stdin().lock()) {
        entries.extend(feed.entries.into_iter());
    }

    // create rss xml feed
    let mut builder = atom::FeedBuilder::default();
    builder.title("a feed".to_string());
    builder.namespace(("media".to_string(), "http://search.yahoo.com/mrss/".to_string()));
    builder.namespace(("yt".to_string(), "http://www.youtube.com/xml/schemas/2015/".to_string()));

    entries.sort_by_key(|e| e.updated);

    for entry in entries {
        builder.entry(entry); 
    }
    
    let feed = builder.build();

    // write to stdout
    feed.write_to(io::stdout().lock()).map_err(io::Error::other)?.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_feed_atom() -> io::Result<()> {
        let input = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
                    <feed xmlns=\"http://www.w3.org/2005/Atom\">
                        <title>an atom feed</title>
                        <link href=\"https://example.org/\"/>
                        <entry>
                            <title>Atom-Powered Robots Run Amok</title>
                            <link href=\"https://example.org/2003/12/13/atom03\"/>
                            <summary>a summary</summary>
                            <content>the actual content</content>
                        </entry>
                    </feed>";

        let result = read_feed(io::BufReader::new(input.as_bytes()))?;

        assert_eq!(result.title.value, "an atom feed");
        assert_eq!(result.links.len(), 1);
        assert_eq!(result.links[0].href, "https://example.org/");
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].title.value, "Atom-Powered Robots Run Amok");
        assert_eq!(result.entries[0].links.len(), 1);
        assert_eq!(result.entries[0].links[0].href, "https://example.org/2003/12/13/atom03");
        assert_eq!(result.entries[0].clone().summary.unwrap().value, "a summary");
        assert_eq!(result.entries[0].clone().content.unwrap().value.unwrap(), "the actual content");

        Ok(())
    }

    #[test]
    fn test_read_feed_rss() -> io::Result<()> {
        let input: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
                            <rss version=\"2.0\"></rss>
                                <channel>
                                    <title>dummy feed</title>
                                    <link>https://www.w3schools.com</link>
                                    <description>a description</description>
                                       <item>
                                        <title>an entry</title>
                                        <link>https://www.w3schools.com/xml/xml_rss.asp</link>
                                        <description>entry description</description>
                                    </item>
                                </channel>
                            </rss>";

        let result = read_feed(io::BufReader::new(input.as_bytes()))?;
        assert_eq!(result.title.value, "dummy feed");
        assert_eq!(result.links.len(), 1);
        assert_eq!(result.links[0].href, "https://www.w3schools.com");
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].title.value, "an entry");
        assert_eq!(result.entries[0].links.len(), 1);
        assert_eq!(result.entries[0].links[0].href, "https://www.w3schools.com/xml/xml_rss.asp");
        assert_eq!(result.entries[0].clone().content.unwrap().value.unwrap(), "entry description");

        Ok(())
    }

    #[test]
    fn test_read_feed_atom_complex() -> io::Result<()> {
        let input = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
                    <feed xmlns=\"http://www.w3.org/2005/Atom\">
                        <title>an atom feed</title>
                        <link href=\"https://example.org/\"/>
                        <updated>2003-12-13T18:30:02Z</updated>
                        <author>
                            <name>John Doe</name>
                        </author>
                        <id>urn:uuid:60a76c80-d399-11d9-b93C-0003939e0af6</id>

                        <entry>
                            <title>Atom-Powered Robots Run Amok</title>
                            <link href=\"https://example.org/2003/12/13/atom03\"/>
                            <id>urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a</id>
                            <updated>2003-12-13T18:30:02Z</updated>
                            <summary>a summary</summary>
                            <content>the actual content</content>
                        </entry>
                    </feed>";

        let result = read_feed(io::BufReader::new(input.as_bytes()))?;

        assert_eq!(result.title.value, "an atom feed");
        assert_eq!(result.id.as_str(), "urn:uuid:60a76c80-d399-11d9-b93C-0003939e0af6");
        assert_eq!(result.links.len(), 1);
        assert_eq!(result.links[0].href, "https://example.org/");
        assert_eq!(result.entries.len(), 1);
        assert_eq!(result.entries[0].title.value, "Atom-Powered Robots Run Amok");
        assert_eq!(result.entries[0].links.len(), 1);
        assert_eq!(result.entries[0].links[0].href, "https://example.org/2003/12/13/atom03");
        assert_eq!(result.entries[0].clone().summary.unwrap().value, "a summary");
        assert_eq!(result.entries[0].clone().content.unwrap().value.unwrap(), "the actual content");
        assert_eq!(result.entries[0].id, "urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a");

        Ok(())


    }
}
