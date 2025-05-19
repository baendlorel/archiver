pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    pub fn from(version_str: &str) -> Self {
        let parts: Vec<u32> = version_str
            .split('.')
            .map(|x| {
                x.parse::<u32>()
                    .expect(&format!("Parse version error! {}", version_str))
            })
            .collect();
        if parts.len() != 3 {
            panic!("Version string must be in the format x.y.z");
        }

        Version {
            major: parts[0],
            minor: parts[1],
            patch: parts[2],
        }
    }

    /// 比较版本，如果大于则返回1，等于返回0，小于则返回-1
    pub fn compare(&self, other: &Version) -> i8 {
        if self.major > other.major {
            return 1;
        }
        if self.major < other.major {
            return -1;
        }

        if self.minor > other.minor {
            return 1;
        }
        if self.minor < other.minor {
            return -1;
        }

        if self.patch > other.patch {
            return 1;
        }
        if self.patch < other.patch {
            return -1;
        }
        0
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
