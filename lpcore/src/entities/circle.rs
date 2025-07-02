use crate::data::size_int::SizeInt;
use crate::data::texture_ref::TextureRef;
use schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct Input {
    image_size: SizeInt,
}

struct Output {
    image: TextureRef,
}

pub fn schema() -> Schema {
    schema_for!(Input)
}
