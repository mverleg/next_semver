use ::std::fmt;

use ::rocket::Build;
use ::rocket::get;
use ::rocket::http::Status;
use ::rocket::launch;
use ::rocket::Request;
use ::rocket::request::FromParam;
use ::rocket::Rocket;
use ::rocket::routes;
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

impl fmt::Display for BumpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)
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
fn part_err(part: &str, version: BumpVersion) -> String {
    format!("cannot parse part (first part of path): '{}' should be one of 'major', 'minor' or 'patch'", part)
}

#[get("/<part>/<version>")]
fn version_err(part: &str, version: &str) -> String {
    format!("cannot parse version (second part of path): '{}' should be a semver, e.g. '1.2.4'", version)
}

#[get("/<first>")]
fn missing_part(first: &str) -> String {
    format!("found only one path part ('{}'), expected two parts, e.g. /major/1.2.4 or /patch/0.2.0", version)
}

#[get("/")]
fn fallback(status: Status, request: &Request) -> String {
    format!("did not find bump and version in path, expected e.g. /major/1.2.4 or /patch/0.2.0")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        next,
        part_err,
        version_err,
        missing_part,
        fallback,
    ])
}
