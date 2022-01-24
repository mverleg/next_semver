use semver::{BuildMetadata, Version};

#[derive(Clone, Copy)]
pub enum Part {
    Major,
    Minor,
    Patch,
}

pub fn bump(version: &Version, part: Part) -> Version {
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
