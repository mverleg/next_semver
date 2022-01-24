use ::std::fmt;

use ::rocket::Build;
use ::rocket::get;
use ::rocket::launch;
use ::rocket::request::FromParam;
use ::rocket::Rocket;
use ::rocket::routes;
use ::semver::Version;
use rocket::response::status;

use ::next_semver::Part;
use next_semver::bump;

#[derive(Debug, Clone, Copy)]
pub enum BumpPart {
    Major,
    Minor,
    Patch,
}

impl From<BumpPart> for Part {
    fn from(part: BumpPart) -> Self {
        match part {
            BumpPart::Major => Part::Major,
            BumpPart::Minor => Part::Minor,
            BumpPart::Patch => Part::Patch,
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

impl From<BumpVersion> for Version {
    fn from(version: BumpVersion) -> Self {
        version.version
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

#[derive(Debug, Clone)]
pub struct PrefixBumpVersion {
    version: Version,
}

impl From<PrefixBumpVersion> for Version {
    fn from(version: PrefixBumpVersion) -> Self {
        version.version
    }
}

impl<'a> FromParam<'a> for PrefixBumpVersion {
    type Error = ();

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        if ! param.starts_with('v') {
            return Err(())
        };
        Ok(PrefixBumpVersion {
            version: Version::parse(&param[1..]).map_err(|_| ())?,
        })
    }
}

#[get("/<part>/<version>", rank = 1)]
fn next(part: BumpPart, version: BumpVersion) -> String {
    bump(&version.into(), part.into()).to_string()
}

#[get("/<part>/<version>", rank = 2)]
fn next_prefix(part: BumpPart, version: PrefixBumpVersion) -> String {
    bump(&version.into(), part.into()).to_string()
}

#[get("/<part>/<version>", rank = 3)]
fn part_err(part: &str, version: BumpVersion) -> status::BadRequest<String> {
    status::BadRequest(Some(format!("cannot parse part (first part of path): '{}' \
        should be one of 'major', 'minor' or 'patch'", part)))
}

#[get("/<part>/<version>", rank = 4)]
fn version_err(part: &str, version: &str) -> status::BadRequest<String> {
    status::BadRequest(Some(format!("cannot parse version (second part of path): '{}' \
        should be a semver, e.g. '1.2.4'", version)))
}

#[get("/<param>")]
fn missing_part(param: &str) -> status::BadRequest<String> {
    status::BadRequest(Some(format!("found only one path part ('{}'), expected two \
        parts, e.g. /major/1.2.4 or /patch/0.2.0", param)))
}

#[get("/")]
fn fallback() -> status::BadRequest<String> {
    status::BadRequest(Some(format!("did not find bump and version in path, expected \
        e.g. /major/1.2.4 or /patch/0.2.0")))
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![
        next,
        next_prefix,
        part_err,
        version_err,
        missing_part,
        fallback,
    ])
}
