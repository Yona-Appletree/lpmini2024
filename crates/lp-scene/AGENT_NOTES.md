# lp-scene Agent Notes

## Open Questions

### Node Structure

- **Will nodes have config and input or just one value?**
  - Current decision: Just input for now. Nodes have `input`, `state`, and `output` fields.
  - Future: May need to separate config (static, expensive to change) from input (dynamic, flows through).

### Runtime Structure

- **How will the runtime structure for nodes work?**
  - Current: Using `NodeInstance` enum to wrap node types. This works but doesn't scale well.
  - Future: Need a better approach for trait objects or dynamic dispatch that works with `RecordValue`.

### Value Updates

- **How will values get updated?**
  - Current: `update()` method mutates internal `output` field.
  - Future: May need more sophisticated update mechanisms for nodes that depend on other nodes.

### Change Notification

- **How do we notify nodes of changes?**
  - Current: Not implemented. Nodes are updated every frame.
  - Future: Need dependency tracking and change propagation system.

### Frame Tracking

- **How do we keep track of which nodes have been computed for a frame?**
  - Current: Frame counter in scene, but not per-node tracking yet.
  - Future: Per-node frame tracking to avoid recomputing unchanged nodes.

### Container Nodes

- **How will container nodes work? (effects are containers, for example)**
  - Current: Not implemented.
  - Future: Need to support nodes with children, hierarchical structure, and scoped name resolution.

## Design Decisions Made

1. **Node Structure**: Nodes are `lp-data` RecordValues with three parts: `input`, `state`, `output`
2. **Node Inputs**: Only input for now (no separate config)
3. **Frame Tracking**: Use frame counter per scene (per-node tracking to be added)
4. **Node Outputs**: Stored in node instance as a field, mutated by `update` method
5. **Graph Traversal**: Nodes are RecordValues, allowing property access via `lp-data` APIs
6. **Node Storage**: Using `NodeInstance` enum for now (will need better solution for scaling)

## Known Issues

1. **NodeInstance Enum**: Using an enum to wrap node types works but doesn't scale. Need a better solution for trait objects that works with `RecordValue`.

2. **BTreeMap Not Supported**: Scene config uses `BTreeMap` which isn't supported by RecordValue derive. For now, `LpSceneConfig` doesn't derive RecordValue. This may need to change in the future.

## Implementation Status

- ✅ Basic crate structure
- ✅ LpNode trait definition
- ✅ LFO node implementation (complete with all waveforms)
- ✅ Scene config structure
- ✅ Scene runtime with update_frame
- ✅ Test suite (all tests passing)
- ✅ Proc macro compatibility (resolved by re-exporting `lp_data::kind` as `crate::kind`)
- ✅ Node output access via RecordValue

## Next Steps

1. Fix proc macro import errors or work around them
2. Implement per-node frame tracking
3. Add support for node dependencies and input resolution
4. Design container node system
5. Improve node storage/dispatch mechanism
