use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Dec32, Mat3, Vec2, Vec3, Vec4};

impl Serialize for Dec32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.to_f32())
    }
}

impl<'de> Deserialize<'de> for Dec32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let f = f32::deserialize(deserializer)?;
        Ok(Dec32::from_f32(f))
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

impl Serialize for Mat3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = [
            self.m[0].to_f32(),
            self.m[1].to_f32(),
            self.m[2].to_f32(),
            self.m[3].to_f32(),
            self.m[4].to_f32(),
            self.m[5].to_f32(),
            self.m[6].to_f32(),
            self.m[7].to_f32(),
            self.m[8].to_f32(),
        ];
        arr.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Mat3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: [f32; 9] = Deserialize::deserialize(deserializer)?;
        Ok(Mat3::from_f32(
            arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7], arr[8],
        ))
    }
}
