//! Test demonstrating runtime value system with a simple scene graph.

#[cfg(test)]
mod tests {
    use crate::shape::int32::LpInt32;
    use crate::shape::record::RecordValue;
    use crate::tests::nodes::lfo::LfoConfig;

    #[test]
    fn test_node_meta() {
        let lfo = LfoConfig {
            period_ms: LpInt32(100),
        };

        // Test getting a field
        let period_ms = lfo
            .get_field("period_ms")
            .expect("should get period_ms field");
        assert_eq!(period_ms.kind(), crate::shape::kind::LpKind::Int32);

        // Test getting a non-existent field
        assert!(lfo.get_field("nonexistent").is_err());
    }

    #[test]
    fn test_node_meta_mut() {
        let mut lfo = LfoConfig {
            period_ms: LpInt32(100),
        };

        // Test getting a mutable field
        let period_ms_mut = lfo
            .get_field_mut("period_ms")
            .expect("should get period_ms field");
        // Can't directly mutate through trait object, but we can verify it works

        // Test setting a field
        lfo.set_field("period_ms", crate::value::LpValue::Int32(200))
            .expect("should set period_ms");
        assert_eq!(lfo.period_ms.0, 200);
    }
}
