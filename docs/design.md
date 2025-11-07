# LightPlayer Architecture Design

## Core Concepts

### Nodes

The system is built from **nodes** - composable units that produce outputs and consume inputs.

Nodes have:

- **Type**: Categorized by namespace (e.g., `ui:slider`, `effect:pipeline`, `math:lfo`)
- **Config**: Static values that define the node's behavior
- **Inputs**: Dynamic values that flow from other nodes
- **Outputs**: Values produced by the node (e.g., `value`, `rgb`)
- **Children**: Scoped sub-nodes owned by the parent

### Node Hierarchy

Nodes can have children using dot notation:

```toml
[perlin.children.zoom]
type = "math:lfo"
period_ms = 5000
```

When a parent node is deleted, its children are automatically removed.

### Inputs vs Config

All node parameters can be changed at runtime (for live editing). The distinction:

- **Input**: A reference to another node's output (e.g., `uniforms = { zoom = "children.zoom.output" }`)
- **Config**: A literal value (e.g., `period_ms = 5000`)

When a **config** value changes, the node is torn down and rebuilt. When an **input** value changes, it just flows through.

## Node Lifecycle

Each node type defines which parameters are "config" (expensive to change).

### Lifecycle Methods

```rust
trait Node {
    fn setup(&mut self, config: &NodeConfig) -> Result<()>;
    fn teardown(&mut self);
    fn update(&mut self, time: Fixed, inputs: &NodeInputs);
    fn get_output(&self, name: &str) -> Option<Value>;

    fn save_state(&self) -> State;
    fn restore_state(&mut self, state: State);
}
```

### Config Change Flow

When a config parameter changes:

1. `state = node.save_state()` - Capture runtime state
2. `node.teardown()` - Clean up resources
3. `node.setup(new_config)` - Reinitialize with new config
4. `node.restore_state(state)` - Restore runtime state

This allows smooth transitions even when structure changes.

## Data Flow

The system is **pull-based** from the renderer's perspective:

```
Render Loop:
  1. Update all nodes (bottom-up: children before parents)
     - Resolve input references
     - Call node.update()
  2. Renderer pulls from active effect
  3. Effect steps pull from their inputs (uniforms)
  4. Map 2D buffer to fixtures
  5. Send to output hardware
```

## Node Types

### Core Categories

- `ui:*` - User interface controls (buttons, sliders, palettes)
- `math:*` - Mathematical functions (lfo, oscillators)
- `effect:*` - Visual effects (pipeline, select, blend)
- `fixture:*` - LED geometry definitions (concentric, grid, strip)
- `output:*` - Hardware interfaces (local-pixels, network)
- `renderer:*` - Orchestrators (2d, 3d)

### Key Node Types

**`ui:slider`** - Continuous value control

- Config: `min`, `max`, `step`, `default`
- Inputs: None (or optional `value` for external control)
- Outputs: `value`

**`math:lfo`** - Low-frequency oscillator

- Config: `period_ms`, `range`, `waveform`
- Inputs: None (or optional `speed` multiplier)
- Outputs: `value`, `phase`

**`effect:pipeline`** - Sequential rendering pipeline

- Config: `steps` (array of step definitions)
- Inputs: Per-step uniforms (defined in step config)
- Outputs: Rendered buffer (implicit)

**`effect:select`** - Effect switcher

- Config: `effects` (array of node references or "children")
- Inputs: `index` (optional, for external control)
- Outputs: Active effect's output

**`fixture:concentric`** - Concentric ring mapping

- Config: `rings`, `wrapping`, `offset`, `format`
- Inputs: `output` (reference to output node)
- Outputs: Mapped LED data

**`renderer:2d`** - 2D canvas renderer

- Config: `resolution`, `fixtures`
- Inputs: `effect` (active effect to render)
- Outputs: None (writes to hardware)

## Pipeline Steps

Effect pipelines contain sequential steps. Each step type defines its behavior:

**`expr` step** - Execute lpscript expression

```toml
{ type = "expr", uniforms = { zoom = "children.zoom.output" }, glsl = "perlin3(uv * zoom, time)" }
```

- Uniforms are resolved and passed to lpscript VM
- Can reference `input` for the previous step's output
- Returns scalar, vec2, vec3, or vec4

**`palette` step** - Apply color mapping

```toml
{ type = "palette", palette = "builtin:rainbow" }
```

- Converts greyscale to RGB using palette lookup

**`color` step** - Fill with solid color

```toml
{ type = "color", color = "FF0000" }
```

- Outputs solid color to buffer

## Reference Resolution

Node references use dot notation with `::` separator for outputs:

```toml
uniforms = { zoom = "children.zoom.output" }
```

Resolution rules:

- `children.zoom.output` - Relative: looks in parent's children
- `zoom.output` - Searches up the tree (current scope, then parent scope, etc.)
- `/perlin/zoom.output` - Absolute: from root

This provides both convenience and precision.

## Scene Structure

A scene is a graph of nodes with a single renderer node as the entry point:

```toml
[canvas]
type = "renderer:2d"
effect = "chooser"
fixtures = ["circle"]
```

The renderer determines what gets evaluated each frame.

## Implementation Phases

**Phase 1: Core Infrastructure**

- Node trait and basic types
- Entity ID and hierarchy system
- Reference resolution
- Simple value types (Fixed)

**Phase 2: Basic Nodes**

- LFO value source
- Simple expr step with uniforms
- Effect pipeline

**Phase 3: UI Integration**

- Effect selector
- UI slider controls
- Live editing

**Phase 4: Hardware**

- Fixture mapping
- Output nodes
- Full render loop

## Design Principles

1. **User simplicity over implementation simplicity** - Hide complexity from users
2. **Everything is live-editable** - No distinction between design-time and runtime
3. **Pull-based evaluation** - Only compute what's needed
4. **Composable by default** - Small nodes combine into complex behaviors
5. **Fail gracefully** - Invalid references or errors don't crash the system

