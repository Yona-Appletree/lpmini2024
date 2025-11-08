use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Fixed, Vec2, Vec3, Vec4};

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

impl Serialize for Vec2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = [self.x.to_f32(), self.y.to_f32()];
        arr.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: [f32; 2] = Deserialize::deserialize(deserializer)?;
        Ok(Vec2::from_f32(arr[0], arr[1]))
    }
}

impl Serialize for Vec3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = [self.x.to_f32(), self.y.to_f32(), self.z.to_f32()];
        arr.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Vec3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: [f32; 3] = Deserialize::deserialize(deserializer)?;
        Ok(Vec3::from_f32(arr[0], arr[1], arr[2]))
    }
}

impl Serialize for Vec4 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = [
            self.x.to_f32(),
            self.y.to_f32(),
            self.z.to_f32(),
            self.w.to_f32(),
        ];
        arr.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Vec4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: [f32; 4] = Deserialize::deserialize(deserializer)?;
        Ok(Vec4::from_f32(arr[0], arr[1], arr[2], arr[3]))
    }
}
