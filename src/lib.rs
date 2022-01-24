use ::std::borrow::Borrow;

use ::semver::{BuildMetadata, Version};

#[derive(Clone, Copy)]
pub enum Part {
    Major,
    Minor,
    Patch,
}

pub fn bump(version: impl Borrow<Version>, part: Part) -> Version {
    let version = version.borrow();
    match part {
        Part::Major => Version {
            major: version.major + 1,
            minor: 0,
            patch: 0,
            pre: version.pre.clone(),
            build: BuildMetadata::EMPTY,
        },
        Part::Minor => Version {
            major: version.major,
            minor: version.minor + 1,
            patch: 0,
            pre: version.pre.clone(),
            build: BuildMetadata::EMPTY,
        },
        Part::Patch => Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch + 1,
            pre: version.pre.clone(),
            build: BuildMetadata::EMPTY,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_pre() {
        assert_eq!(v("1.3.0-alpha").to_string(), next_minor("1.2.5-alpha"))
    }

    #[test]
    fn strip_build() {
        assert_eq!(v("1.3.0").to_string(), next_minor("1.2.5+567"))
    }

    #[test]
    fn pre_and_build() {
        assert_eq!(v("1.3.0-alpha").to_string(), next_minor("1.2.5-alpha+567"))
    }

    fn v(version: &str) -> Version {
        Version::parse(version).unwrap()
    }

    fn next_minor(version: &str) -> String {
        bump(v(version), Part::Minor).to_string()
    }
}
