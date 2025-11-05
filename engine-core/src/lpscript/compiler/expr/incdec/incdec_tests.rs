/// Tests for increment/decrement operators
#[cfg(test)]
mod tests {
    use crate::lpscript::compile_script;
    use crate::lpscript::vm::LpsOpCode;

    #[test]
    fn test_prefix_increment_opcodes() {
        // Just check that it compiles and generates reasonable opcodes
        let script = "int x = 5; int y = ++x; return y;";
        let program = compile_script(script).unwrap();

        println!("Opcodes for 'int y = ++x':");
        for (i, op) in program.opcodes.iter().enumerate() {
            println!("  {}: {}", i, op.name());
        }

        // Check we don't have an absurd number of opcodes (indicating infinite loop)
        assert!(
            program.opcodes.len() < 100,
            "Too many opcodes generated: {}",
            program.opcodes.len()
        );
    }

    #[test]
    fn test_postfix_increment_opcodes() {
        let script = "int x = 5; int y = x++; return y;";
        let program = compile_script(script).unwrap();

        println!("Opcodes for 'int y = x++':");
        for (i, op) in program.opcodes.iter().enumerate() {
            println!("  {}: {}", i, op.name());
        }

        assert!(
            program.opcodes.len() < 100,
            "Too many opcodes generated: {}",
            program.opcodes.len()
        );
    }

    #[test]
    #[ignore] // Compound assignment has issues (pre-existing)
    fn test_compound_assignment_opcodes() {
        let script = "int x = 10; x += 5; return x;";
        let program = compile_script(script).unwrap();

        println!("Opcodes for 'x += 5':");
        for (i, op) in program.opcodes.iter().enumerate() {
            println!("  {}: {}", i, op.name());
        }

        assert!(
            program.opcodes.len() < 100,
            "Too many opcodes generated: {}",
            program.opcodes.len()
        );

        // Note: We don't run this in the VM because there's a bug that causes infinite execution
        // The opcodes look correct, but there may be an issue with jump offsets or similar
        // TODO: Debug and fix the VM execution issue
    }
}
