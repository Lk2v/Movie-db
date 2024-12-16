use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum SearchGenre {
    All = -1,
    Action = 1,
    Adventure = 2,
    Animation = 3,
    Comedy = 4,
    Crime = 5,
    Documentary = 6,
    Drama = 7,
    Family = 8,
    Fantasy = 9,
    History = 10,
    Horror = 11,
    Music = 12,
    Mystery = 13,
    Romance = 14,
    ScienceFiction = 15,
    Thriller = 16,
    TVMovie = 17,
    War = 18,
    Western = 19
}

impl SearchGenre {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            SearchGenre::All => None,

            SearchGenre::Action => Some("Action"),
            SearchGenre::Adventure => Some("Adventure"),
            SearchGenre::Animation => Some("Animation"),
            SearchGenre::Comedy => Some("Comedy"),
            SearchGenre::Crime => Some("Crime"),
            SearchGenre::Documentary => Some("Documentary"),
            SearchGenre::Drama => Some("Drama"),
            SearchGenre::Family => Some("Family"),
            SearchGenre::Fantasy => Some("Fantasy"),
            SearchGenre::History => Some("History"),
            SearchGenre::Horror => Some("Horror"),
            SearchGenre::Music => Some("Music"),
            SearchGenre::Mystery => Some("Mystery"),
            SearchGenre::Romance => Some("Romance"),
            SearchGenre::ScienceFiction => Some("Science Fiction"),
            SearchGenre::Thriller => Some("Thriller"),
            SearchGenre::TVMovie => Some("TV Movie"),
            SearchGenre::War => Some("War"),
            SearchGenre::Western => Some("Western"),
        }
    }
}