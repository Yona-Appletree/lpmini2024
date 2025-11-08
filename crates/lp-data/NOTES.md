# Design Questions and Decisions

## Open Questions

### 1. Primitive Shape Modules
**Question**: Should primitives (Fixed, Int32, Bool, String, Vec2, Vec3, Vec4) have shape modules in `src/shape/` (like `shape/fixed/`, `shape/int32/`), or are the `ShapeRef` variants sufficient since they have no metadata?

**Context**: Currently primitives are just variants in `ShapeRef` enum with no associated metadata. But metadata might be needed (e.g., "this Vec3 should be a color").

**Related**: Where does metadata live? On the value itself or on the field that contains the value? Example: if a Vec3 should be a color, where does that metadata live?

**Decision**: **Option D - Hybrid approach**
- Primitives have shape modules with optional metadata
- Default shapes have no metadata (just the kind)
- Can attach metadata when needed (e.g., `Vec3Shape::color()` vs `Vec3Shape::position()`)
- Metadata lives on the value itself (via the shape)
- All primitives get shape modules for consistency, even if they start with no metadata

**Status**: Decided

---

### 2. Value Struct Location
**Question**: Should value structs (ArrayValue, OptionValue, etc.) move to `src/shape/{array,option}/` and implement the corresponding value traits, or stay in `src/types/`?

**Context**: RecordValue and RecordShape live in `src/shape/record/mod.rs`. Should other value types follow the same pattern?

**Decision**: **Move to `src/shape/` modules**
- Value structs move to `shape/{array,option,enum,tuple}/` modules
- They implement the corresponding value traits
- Consistent with `RecordValueDyn` in `shape/record/record_dynamic.rs`
- Keeps shape and value together

**Status**: Decided

---

### 3. Static Records Implementing RecordValue
**Question**: Should codegen generate methods that access Rust struct fields directly, or should there be a wrapper that converts the struct to/from `LpValue`?

**Context**: Static records like `LfoConfig` are Rust structs. They need to implement `RecordValue` trait. Do we:
- Generate methods that directly access struct fields?
- Wrap the struct in something that converts to/from `LpValue`?

**Decision**: **Newtype wrappers with trait objects (Option F2 + Option 1)**
- Struct fields use newtype wrappers: `frequency: LpFixed` where `LpFixed` wraps `Fixed`
- Newtype wrappers implement `LpValueTrait` directly
- `RecordValue::get_field()` returns `&dyn LpValueTrait` (trait objects)
- This allows direct reference to fields: `Ok(&self.frequency as &dyn LpValueTrait)`
- No conversion, no caching, no temporary values needed
- Type-safe at compile time (can't assign wrong type to field)
- Codegen generates `impl RecordValue for LfoConfig` with direct field access

**Status**: Decided

---

### 4. Static Enums/Tuples
**Question**: Should static enums/tuples also implement `EnumValue`/`TupleValue` directly, or do they need wrappers like `RecordValueDyn`?

**Context**: Rust enums can implement traits. But do we need wrappers for them like we do for dynamic records?

**Decision**: **Same pattern as static records**
- Use newtype wrappers: `LpWaveform` wraps `Waveform` enum
- Wrappers implement `EnumValue`/`TupleValue` traits
- Traits return `&dyn LpValueTrait` for consistency
- Same benefits: type safety, direct references, no conversion overhead

**Status**: Decided

---

### 5. LpValue Enum Storage
**Question**: Should `LpValue` enum variants continue to store concrete structs like `ArrayValue`, `StructValue`, or should they store trait objects (`Box<dyn RecordValue>`, etc.)?

**Context**: 
- Dynamic values likely need trait objects (e.g., `Box<dyn RecordValue>`)
- Static values could be concrete structs
- Current design stores concrete structs like `ArrayValue`, `StructValue`, `MapValue`, etc.

**Decision**: **Hybrid approach**
- Static primitives: `LpValue::Fixed(LpFixed)` - stores newtype wrapper directly
- Static records: Don't use `LpValue` enum - they implement `RecordValue` directly and return `&dyn LpValueTrait`
- Dynamic values: Use trait objects in `LpValue` enum (e.g., `LpValue::Record(Box<dyn RecordValue>)`)
- Arrays/Options/etc: Can use either pattern depending on static vs dynamic

**Status**: Decided (partial - needs refinement for composite types)

---

## Design Decisions

### RecordValue and RecordShape Location
**Decision**: `RecordValue` and `RecordShape` traits live in `src/shape/record/mod.rs` and are re-exported at the root for easy access.

**Status**: Implemented

---

### Dynamic Records
**Decision**: `RecordValueDyn` in `src/shape/record/record_dynamic.rs` implements `RecordValue` trait. This is how dynamic records have their values stored.

**Status**: Partially implemented (needs `RecordValue` implementation)

---

### Static Records
**Decision**: Static records don't have a Value per se - they're just Rust structs. Codegen would generate `impl RecordValue for LfoConfig` and static code for accessing its values. Codegen would also implement some kind of trait with a const shape (or similar) for the LfoConfig shape.

**Status**: Design phase

---

## Implementation Notes

### Current State
- Old `src/types/` directory still exists with old metadata system
- New `src/shape/` directory has shape system partially implemented
- `LpValue` enum still uses old value structs from `src/types/`
- Need to clean up old code and migrate to new system

### Next Steps
1. Clean up old `src/types/` code (mostly delete)
2. Flesh out primitive/literal types in `src/shape/`
3. Move value structs to appropriate `src/shape/` modules
4. Implement `RecordValue` for `RecordValueDyn`
5. Resolve open questions above

