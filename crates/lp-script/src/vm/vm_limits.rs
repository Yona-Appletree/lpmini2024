/// Configuration limits for the VM
#[derive(Debug, Clone, Copy)]
pub struct VmLimits {
    pub max_call_stack_depth: usize,
    pub max_stack_size: usize,
    pub max_instructions: usize,
}

impl Default for VmLimits {
    fn default() -> Self {
        VmLimits {
            max_call_stack_depth: 64,
            max_stack_size: 256,
            max_instructions: 10_000,
        }
    }
}

#[cfg(test)]
mod tests {
    use lp_pool::allow_global_alloc;

    use super::*;
    use crate::vm::LpsVm;

    #[test]
    fn test_vm_limits() {
        use crate::parse_expr;
        let program = allow_global_alloc(|| parse_expr("1.0"));

        let custom_limits = VmLimits {
            max_call_stack_depth: 32,
            max_stack_size: 128,
            max_instructions: 5000,
        };

        let vm = LpsVm::new(&program, custom_limits).unwrap();
        assert_eq!(vm.limits.max_call_stack_depth, 32);
        assert_eq!(vm.limits.max_stack_size, 128);
        assert_eq!(vm.limits.max_instructions, 5000);
    }
}
