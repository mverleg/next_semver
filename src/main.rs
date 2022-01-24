use ::rocket::Build;
use ::rocket::get;
use ::rocket::launch;
use ::rocket::request::FromParam;
use ::rocket::Rocket;
use ::rocket::routes;
use ::rocket::catch;
use ::semver::Version;

use ::next_semver::Part;

#[derive(Debug, Clone, Copy)]
pub enum BumpPart {
    Major,
    Minor,
    Patch,
}

impl From<Part> for BumpPart {
    fn from(part: Part) -> Self {
        match part {
            Part::Major => BumpPart::Major,
            Part::Minor => BumpPart::Minor,
            Part::Patch => BumpPart::Patch,
        }
    }
}

impl<'a> FromParam<'a> for BumpPart {
    type Error = ();

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(match param {
             "ma" | "major" | "breaking" => BumpPart::Major,
             "mi" | "minor" | "feature" => BumpPart::Minor,
             "pa" | "patch" | "fix" => BumpPart::Patch,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BumpVersion {
    version: Version,
}

impl From<Version> for BumpVersion {
    fn from(version: Version) -> Self {
        BumpVersion { version }
    }
}

impl<'a> FromParam<'a> for BumpVersion {
    type Error = ();

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(BumpVersion {
            version: Version::parse(param).map_err(|_| ())?,
        })
    }
}

#[get("/<part>/<version>")]
fn next(part: BumpPart, version: BumpVersion) -> String {
    version.to_string()
}

#[get("/<part>/<version>")]
fn cannot_parse(part: &str, version: &str) -> String {
    "not found".to_owned()
}

#[catch(default)]
fn fallback() -> String {
    "not found".to_owned()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        next,
        cannot_parse,
        fallback,
    ])
}
