# Lightplayer Scene

`lp-scene` contains the scene graph implementation. This is the core of the engine and contains
the busisness logic for creating, updating, and running scenes.

A scene is defined in a config object, often deserialized from JSON or TOML.

Scenes are made up of nodes, which are connected together to form a graph.

Data is pulled through the scene graph.

## Time Representation

Time is tracked in two ways:

- **Absolute time**: Integer milliseconds (`i64`) - sufficient precision for absolute time tracking
- **Relative time**: Fixed-point milliseconds (`Fixed`) - fractional precision for sub-millisecond accuracy in delta calculations
