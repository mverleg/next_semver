use semver::Version;

#[derive(Clone, Copy)]
pub enum Part {
    Major,
    Minor,
    Patch,
}

pub fn bump(version: &Version, part: Part) -> Version {
    unimplemented!()
}
