use ::std::fmt;

use ::rocket::get;
use ::rocket::launch;
use ::rocket::request::FromParam;
use ::rocket::response::status;
use ::rocket::routes;
use ::rocket::Build;
use ::rocket::Rocket;
use ::semver::Version;

use ::next_semver::bump;
use ::next_semver::Part;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
        if !param.starts_with('v') {
            return Err(());
        };
        Ok(PrefixBumpVersion {
            version: Version::parse(&param[1..]).map_err(|_| ())?,
        })
    }
}

#[get("/json/<version>", rank = 1)]
fn next_json(version: BumpVersion) -> String {
    let version = version.into();
    let major = bump(&version, BumpPart::Major.into()).to_string();
    let minor = bump(&version, BumpPart::Minor.into()).to_string();
    let patch = bump(&version, BumpPart::Patch.into()).to_string();
    format!("{{\"current\":\"{}\",\"major\":\"{}\",\"minor\":\"{}\",\"patch\":\"{}\"}}\n", version, major, minor, patch)
}

#[get("/<part>/<version>", rank = 2)]
#[allow(clippy::needless_borrows_for_generic_args)]
fn next(part: BumpPart, version: BumpVersion) -> String {
    bump(&version.into(), part.into()).to_string()
}

#[get("/<part>/<version>", rank = 3)]
#[allow(clippy::needless_borrows_for_generic_args)]
fn next_prefix(part: BumpPart, version: PrefixBumpVersion) -> String {
    bump(&version.into(), part.into()).to_string()
}

#[get("/<part>/<_>", rank = 4)]
fn part_err(part: &str) -> status::BadRequest<String> {
    status::BadRequest(format!(
        "cannot parse part (first part of path): '{}' \
        should be one of 'major', 'minor' or 'patch'",
        part
    ))
}

#[get("/<_>/<version>", rank = 5)]
fn version_err(version: &str) -> status::BadRequest<String> {
    status::BadRequest(format!(
        "cannot parse version (second part of path): '{}' \
        should be a semver, e.g. '1.2.4'",
        version
    ))
}

#[get("/<_>/<_>/<_>")]
fn three_parts() -> status::BadRequest<String> {
    status::BadRequest(
        "path too long, expected two parts, e.g. /major/1.2.4 or /patch/0.2.0".to_owned(),
    )
}

//TODO: there's probably a better way for this?
#[get("/<_>/<_>/<_>/<_>")]
fn four_parts() -> status::BadRequest<String> {
    status::BadRequest(
        "path too long, expected two parts, e.g. /major/1.2.4 or /patch/0.2.0".to_owned(),
    )
}

#[get("/<param>")]
fn missing_part(param: &str) -> status::BadRequest<String> {
    status::BadRequest(format!(
        "found only one path part ('{}'), expected two \
        parts, e.g. /major/1.2.4 or /patch/0.2.0",
        param
    ))
}

#[get("/")]
fn fallback() -> status::BadRequest<String> {
    status::BadRequest(
        ("Welcome to next_semver! This service gives you \
    bumped version numbers. Are you on version 1.2.5 and have a new feature? Request \
    /minor/1.2.5 and you get your next version: 1.3.0. It is extremely simple. First path \
    part is major, minor or patch, second part is the current semantic version. \
    There is /json/2.4.1 to get all 3 bumped versions as json.")
            .to_owned(),
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    //TODO: maybe catch all other request methods
    rocket::build().mount(
        "/",
        routes![
            next_json,
            next,
            next_prefix,
            part_err,
            version_err,
            three_parts,
            four_parts,
            missing_part,
            fallback,
        ],
    )
}
