use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::distros;
use libapt::{Distro, Key};


#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::distros)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DbDistro {
    pub id: i32,
    pub url: String,
    pub name: Option<String>,
    pub path: Option<String>,
    pub key: Option<String>,
    pub armored_key: bool,
}

impl DbDistro {
    pub fn to_distro(self) -> Option<Distro> {
        let distro_key = match &self.key {
            Some(k) => {
                if self.armored_key {
                    Key::armored_key(k)
                } else {
                    Key::key(k)
                }
            },
            None => Key::NoSignatureCheck,
        };

        if let Some(n) = self.name {
            Some(Distro::repo(&self.url, &n, distro_key))
        } else if let Some(p) = self.path {
            Some(Distro::flat_repo(&self.url, &p, distro_key))
        } else {
            None
        }
    }
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = distros)]
pub struct NewDistro {
    pub url: String,
    pub name: Option<String>,
    pub path: Option<String>,
    pub key: Option<String>,
    pub armored_key: bool,
}

impl NewDistro {
    pub fn from_distro(distro: Distro) -> Option<NewDistro> {
        let (armored_key, key) = match distro.key {
            Key::NoSignatureCheck => (false, None),
            Key::ArmoredKey(k) => (true, Some(k)),
            Key::Key(k) => (false, Some(k)),
        };

        if let Some(n) = distro.name {
            Some(NewDistro {
                url: distro.url,
                name: Some(n),
                path: None,
                key: key,
                armored_key: armored_key,
            })
        } else if  let Some(n) = distro.path {
            Some(NewDistro {
                url: distro.url,
                name: Some(n),
                path: None,
                key: key,
                armored_key: armored_key,
            })
        } else {
            None
        }
    }
}

