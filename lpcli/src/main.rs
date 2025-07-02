use lpcore::entities::lfo;
use serde_json;

fn main() {
    let schema = lfo::schema();
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
