use serde::{Deserialize, Serialize};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Serializer {
    fn serialize<T: Serialize + 'static>(&self, data: &T) -> Result<String, serde_yaml::Error>;
    fn deserialize<T: for<'de> Deserialize<'de> + 'static>(
        &self,
        content: &str,
    ) -> Result<T, serde_yaml::Error>;
}

pub struct YamlSerializer;
impl YamlSerializer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Serializer for YamlSerializer {
    fn serialize<T: Serialize>(&self, data: &T) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(data)
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(
        &self,
        content: &str,
    ) -> Result<T, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }
}
