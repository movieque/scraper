use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use super::Error;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dataset {
    Movies,
    TvShows,
    People,
    Seasons,
    Episodes,
    Companies,
    Networks,
}


impl Dataset {
    pub fn url_path(&self) -> &'static str {
        match self {
            Dataset::Movies => "movie_ids_",
            Dataset::TvShows => "tv_series_ids_",
            Dataset::Seasons => "seasons_ids_",
            Dataset::Episodes => "episodes_ids_",
            Dataset::People => "person_ids_",
            Dataset::Networks => "tv_network_ids_",
            Dataset::Companies => "production_company_ids_"
        }
    }
}


impl Display for Dataset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Dataset::Movies => write!(f, "movies"),
            Dataset::TvShows => write!(f, "tv_shows"),
            Dataset::People => write!(f, "people"),
            Dataset::Seasons => write!(f, "seasons"),
            Dataset::Episodes => write!(f, "episodes"),
            Dataset::Companies => write!(f, "companies"),
            Dataset::Networks => write!(f, "networks")
        }
    }
}


impl FromStr for Dataset {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "movies" => Ok(Dataset::Movies),
            "tv_shows" => Ok(Dataset::TvShows),
            "people" => Ok(Dataset::People),
            "seasons" => Ok(Dataset::Seasons),
            "episodes" => Ok(Dataset::Episodes),
            "companies" => Ok(Dataset::Companies),
            "networks" => Ok(Dataset::Networks),
            _ => Err(Error::UnknownDataset),
        }
    }
}