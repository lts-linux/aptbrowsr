use diesel::prelude::*;
use crate::schema::distros;
use libapt::{Distro, Key};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::distros)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Repo {
    pub id: i32,
    pub url: String,
    pub name: Option<String>,
    pub path: Option<String>,
    pub key: Option<String>,
    pub armored_key: bool,
}

impl Repo {
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

#[derive(Insertable)]
#[diesel(table_name = distros)]
pub struct NewRepo<'a> {
    pub url: &'a str,
    pub name: Option<&'a str>,
    pub path: Option<&'a str>,
    pub key: Option<&'a str>,
    pub armored_key: bool,
}
