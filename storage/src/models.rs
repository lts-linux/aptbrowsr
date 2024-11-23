use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use common::distro::Distro;

use crate::schema::distros;


#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::distros)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbDistro {
    pub id: i32,
    pub repo_url: String,
    pub name_or_path: String,
    pub repo_key: Option<String>,
    pub armored_key: bool,
    pub flat_repo: bool,
}

impl DbDistro {
    pub fn to_distro(self) -> Distro {
        Distro {
            id: self.id,
            url: self.repo_url,
            name_or_path: self.name_or_path,
            key: self.repo_key,
            armored_key: self.armored_key,
            flat_repo: self.flat_repo,
        }   
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = distros)]
pub struct NewDistro {
    pub repo_url: String,
    pub name_or_path: String,
    pub repo_key: Option<String>,
    pub armored_key: bool,
    pub flat_repo: bool,
}

impl NewDistro {
    pub fn from_distro(distro: Distro) -> NewDistro {
        NewDistro {
            repo_url: distro.url,
            name_or_path: distro.name_or_path,
            repo_key: distro.key,
            armored_key: distro.armored_key,
            flat_repo: distro.flat_repo,
        }
    }
}
