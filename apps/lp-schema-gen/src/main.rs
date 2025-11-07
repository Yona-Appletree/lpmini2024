mod registry;
mod types;
mod zod_gen;

use registry::SchemaRegistry;
use types::lfo_config::{LfoConfig, LfoShape};
use types::ui_slider::UiSliderConfig;

fn main() {
    // Create registry and register all types
    let mut registry = SchemaRegistry::new();
    registry.register::<UiSliderConfig>();
    registry.register::<LfoShape>();
    registry.register::<LfoConfig>();

    // Generate Zod schemas
    let zod_schemas = zod_gen::generate_zod_schemas(&registry);
    
    // Print to stdout (can be redirected to a file)
    print!("{}", zod_schemas);
}
