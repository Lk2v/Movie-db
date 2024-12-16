use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchFilter {
    Alphabetical,
    Popular,
    Latest,
    TopRated,
    None,
    Unknow
}

impl SearchFilter {
    pub fn to_view_name(&self) -> &'static str {
        match self {
            SearchFilter::Alphabetical => "MovieShortView_Alphabetical",
            SearchFilter::Popular => "MovieShortView_ByPopularity",
            SearchFilter::Latest => "MovieShortView_ByReleaseDate",
            SearchFilter::TopRated => "MovieShortView_ByRating",
            _ => "MovieShortView",
        }
    }
    /*fn from(&self, str: &str ) -> Self {
        match str {
            "alphabetical" => SearchFilter::Alphabetical,
            "popular" => SearchFilter::Popular,
            "latest" => SearchFilter::Latest,
            "upcoming" => SearchFilter::Upcoming,
            "toprated" => SearchFilter::TopRated,
            _ => SearchFilter::Unknow,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            SearchFilter::Alphabetical => "alphabetical",
            SearchFilter::Popular => "popular",
            SearchFilter::Latest => "latest",
            SearchFilter::Upcoming => "upcoming",
            SearchFilter::TopRated => "toprated",
            _ => "unknown"
        }
    }*/
}
