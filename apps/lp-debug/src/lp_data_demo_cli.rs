use lp_debug::lp_data_demo::CircleMappingConfig;

fn main() {
    let schema = CircleMappingConfig::schema_json();
    println!(
        "Schema JSON:\n{}",
        serde_json::to_string_pretty(&schema).expect("schema serialization")
    );

    let sample = CircleMappingConfig::sample();
    println!(
        "\nSample Config:\n{}",
        serde_json::to_string_pretty(&sample).expect("sample serialization")
    );

    let lp_value = CircleMappingConfig::sample_lp_value();
    println!(
        "\nSample LpValue:\n{}",
        serde_json::to_string_pretty(&lp_value).expect("lp value serialization")
    );
}

