pub struct NameParser;

impl NameParser {
    ///
    /// Expects filename without extension
    ///
    pub fn parse(filename: impl AsRef<str>) -> ParsedName {
        let title = String::from(filename.as_ref());
        let mut title = title.replace('.', " ");

        let year = Self::guess_year(&title);

        if let Some(index) = year.map(|(i, _)| i).or(Self::find_known_separator(&title)) {
            // year or resolution is usually last relevant info
            title = title
                .split_whitespace()
                .take(index)
                .collect::<Vec<_>>()
                .join(" ");
        }

        let year = year.map(|(_, year)| year);

        ParsedName { title, year }
    }

    fn guess_year(title: &str) -> Option<(usize, usize)> {
        for (i, mut word) in title.split_whitespace().enumerate() {
            let pattern: &[_] = &['(', ')'];
            word = word.trim_matches(pattern);

            if let Ok(num) = word.parse::<usize>() {
                // is probably year
                if num > 1850 {
                    return Some((i, num));
                }
            }
        }
        None
    }

    fn find_known_separator(title: &str) -> Option<usize> {
        for (i, word) in title.split_whitespace().enumerate() {
            match word {
                "720p" | "1080p" | "2160p" => return Some(i),
                _ => continue,
            }
        }
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedName {
    pub title: String,
    pub year: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_parses_names() {
        let names = [
            "Movie.Name.1929.DVDRip.600MB.Some[Thing]",
            "Another Movie 5 (2015) Bluray 1080p.h264-Some[Thing]",
            "Awesome.Movie.720p",
        ];

        let expected = [
            ParsedName {
                title: "Movie Name".into(),
                year: Some(1929),
            },
            ParsedName {
                title: "Another Movie 5".into(),
                year: Some(2015),
            },
            ParsedName {
                title: "Awesome Movie".into(),
                year: None,
            },
        ];

        for (name, expected) in names.iter().zip(expected) {
            assert_eq!(NameParser::parse(name), expected);
        }
    }
}
