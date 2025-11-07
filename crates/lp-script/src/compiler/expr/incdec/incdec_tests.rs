/// Tests for increment/decrement operators
#[cfg(test)]
mod tests {
    use crate::compile_script;

    #[test]
    fn test_prefix_increment_opcodes() {
        // Just check that it compiles and generates reasonable opcodes
        let script = "int x = 5; int y = ++x; return y;";
        let program = compile_script(script).unwrap();

        println!("Opcodes for 'int y = ++x':");
        let main_fn = program.main_function().unwrap();
        for (i, op) in main_fn.opcodes.iter().enumerate() {
            println!("  {}: {}", i, op.name());
        }

        // Check we don't have an absurd number of opcodes (indicating infinite loop)
        assert!(
            main_fn.opcodes.len() < 100,
            "Too many opcodes generated: {}",
            main_fn.opcodes.len()
        );
    }

    #[test]
    fn test_postfix_increment_opcodes() {
        let script = "int x = 5; int y = x++; return y;";
        let program = compile_script(script).unwrap();

        println!("Opcodes for 'int y = x++':");
        let main_fn = program.main_function().unwrap();
        for (i, op) in main_fn.opcodes.iter().enumerate() {
            println!("  {}: {}", i, op.name());
        }

        assert!(
            main_fn.opcodes.len() < 100,
            "Too many opcodes generated: {}",
            main_fn.opcodes.len()
        );
    }

    #[test]
    fn test_compound_assignment_opcodes() {
        let script = "int x = 10; x += 5; return x;";
        let program = compile_script(script).unwrap();

        println!("Opcodes for 'x += 5':");
        let main_fn = program.main_function().unwrap();
        for (i, op) in main_fn.opcodes.iter().enumerate() {
            println!("  {}: {}", i, op.name());
        }

        assert!(
            main_fn.opcodes.len() < 100,
            "Too many opcodes generated: {}",
            main_fn.opcodes.len()
        );

        // Note: We don't run this in the VM because there's a bug that causes infinite execution
        // The opcodes look correct, but there may be an issue with jump offsets or similar
        // TODO: Debug and fix the VM execution issue
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::compile_script;
    use crate::fixed::ToFixed;
    use crate::vm::lps_vm::LpsVm;
    use crate::vm::vm_limits::VmLimits;

    #[test]
    fn test_prefix_increment_integration() {
        let script = "
            int x = 5;
            int y = ++x;
            return y;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 6); // ++5 = 6
    }

    #[test]
    fn test_postfix_increment_integration() {
        let script = "
            int x = 5;
            int y = x++;
            return y;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 5); // Returns original value
    }

    #[test]
    fn test_compound_addition_integration() {
        let script = "
            int x = 10;
            x += 5;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 15); // 10 + 5 = 15
    }

    #[test]
    fn test_compound_bitwise_and_integration() {
        let script = "
            int x = 15;
            x &= 7;
            return x;
        ";
        let program = compile_script(script).unwrap();
        let mut vm = LpsVm::new(&program, VmLimits::default()).unwrap();
        let result = vm
            .run_scalar(0.0.to_fixed(), 0.0.to_fixed(), 0.0.to_fixed())
            .unwrap();
        assert_eq!(result.0, 7); // 15 & 7 = 7
    }
}
