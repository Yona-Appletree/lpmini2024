use super::Fixed;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Fixed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.to_f32())
    }
}

impl<'de> Deserialize<'de> for Fixed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let f = f32::deserialize(deserializer)?;
        Ok(Fixed::from_f32(f))
    }
}
