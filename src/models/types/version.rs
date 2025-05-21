use std::cmp::Ordering;

pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    pub fn from(version_str: &str) -> Self {
        let parts: Vec<u32> = version_str
            .trim_start_matches('v')
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
    pub fn cmp(&self, other: &Version) -> Ordering {
        if self.major > other.major {
            return Ordering::Greater;
        }
        if self.major < other.major {
            return Ordering::Less;
        }

        if self.minor > other.minor {
            return Ordering::Greater;
        }
        if self.minor < other.minor {
            return Ordering::Less;
        }

        if self.patch > other.patch {
            return Ordering::Greater;
        }
        if self.patch < other.patch {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
