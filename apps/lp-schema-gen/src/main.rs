mod registry;
mod types;
mod zod_gen;

use registry::SchemaRegistry;
use types::lfo_config::{LfoConfig, LfoShape};
use types::ui_slider::UiSliderConfig;

fn main() {
    // Create registry and register all types using LpSchema types
    let mut registry = SchemaRegistry::new();
    registry.register::<UiSliderConfig>();
    registry.register::<LfoShape>();
    registry.register::<LfoConfig>();

    // Generate Zod schemas from lp-data types
    let zod_schemas = zod_gen::generate_zod_schemas(registry.registry());

    // Print to stdout (can be redirected to a file)
    print!("{}", zod_schemas);
}
