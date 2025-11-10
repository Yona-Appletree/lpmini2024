mod registry;
mod types;
mod zod_gen;

use registry::SchemaRegistry;
// use types::lfo_config::{LfoConfig, LfoShape};
// use types::ui_slider::UiSliderConfig;

fn main() {
    // Create registry and register all types using LpSchema types
    // TODO: These types need to implement LpValue and LpDescribe first
    // Once LpSchema derive is fixed or types implement LpValue manually, uncomment:
    let registry = SchemaRegistry::new();
    // registry.register::<UiSliderConfig>();
    // registry.register::<LfoShape>();
    // registry.register::<LfoConfig>();

    // Generate Zod schemas from lp-data types
    let zod_schemas = zod_gen::generate_zod_schemas(&registry.all_types());

    // Print to stdout (can be redirected to a file)
    print!("{}", zod_schemas);
}
