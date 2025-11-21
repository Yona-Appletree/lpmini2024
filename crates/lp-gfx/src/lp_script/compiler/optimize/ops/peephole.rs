/// Peephole optimization for opcodes
///
/// Pattern-matches local sequences of opcodes and replaces them with more efficient sequences.
///
/// Patterns:
/// - Push x; Drop1 → (delete)
/// - LoadLocal(x); StoreLocal(x) → (delete) if same index
/// - Remove unreachable opcodes after unconditional Jump
extern crate alloc;
use alloc::vec::Vec;

use crate::lp_script::vm::opcodes::LpsOpCode;

/// Optimize opcodes using peephole patterns
pub fn optimize(opcodes: Vec<LpsOpCode>) -> Vec<LpsOpCode> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < opcodes.len() {
        // Pattern: Push followed by Drop1
        if i + 1 < opcodes.len()
            && matches!(opcodes[i], LpsOpCode::Push(_) | LpsOpCode::PushInt32(_))
            && matches!(opcodes[i + 1], LpsOpCode::Drop1)
        {
            // Skip both instructions
            i += 2;
            continue;
        }

        // Pattern: LoadLocal(x) followed by StoreLocal(x) with same index
        if i + 1 < opcodes.len() {
            match (&opcodes[i], &opcodes[i + 1]) {
                (LpsOpCode::LoadLocalDec32(idx1), LpsOpCode::StoreLocalDec32(idx2))
                | (LpsOpCode::LoadLocalInt32(idx1), LpsOpCode::StoreLocalInt32(idx2))
                | (LpsOpCode::LoadLocalVec2(idx1), LpsOpCode::StoreLocalVec2(idx2))
                | (LpsOpCode::LoadLocalVec3(idx1), LpsOpCode::StoreLocalVec3(idx2))
                | (LpsOpCode::LoadLocalVec4(idx1), LpsOpCode::StoreLocalVec4(idx2))
                    if idx1 == idx2 =>
                {
                    // Skip both instructions (loading and storing to same place is a no-op)
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        // Pattern: Dup followed by Drop (various sizes)
        if i + 1 < opcodes.len() {
            match (&opcodes[i], &opcodes[i + 1]) {
                (LpsOpCode::Dup1, LpsOpCode::Drop1)
                | (LpsOpCode::Dup2, LpsOpCode::Drop2)
                | (LpsOpCode::Dup3, LpsOpCode::Drop3)
                | (LpsOpCode::Dup4, LpsOpCode::Drop4) => {
                    // Skip both (dup then drop is a no-op)
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }

        // No pattern matched, keep the instruction
        result.push(opcodes[i]);
        i += 1;
    }

    // Second pass: remove unreachable code after unconditional jumps
    remove_unreachable_after_jumps(result)
}

/// Remove unreachable opcodes after unconditional jumps
fn remove_unreachable_after_jumps(opcodes: Vec<LpsOpCode>) -> Vec<LpsOpCode> {
    let mut result = Vec::new();
    let jump_targets = collect_jump_targets(&opcodes);

    // Track mapping from old index to new index
    let mut index_mapping = alloc::vec![None; opcodes.len()];

    let mut i = 0;
    while i < opcodes.len() {
        index_mapping[i] = Some(result.len());
        result.push(opcodes[i]);

        // If this is an unconditional jump or return, skip instructions until next jump target
        if matches!(opcodes[i], LpsOpCode::Jump(_) | LpsOpCode::Return) {
            i += 1;

            // Skip instructions until we hit a jump target
            while i < opcodes.len() && !jump_targets.contains(&i) {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    // Patch all jump offsets to account for removed instructions
    patch_jump_offsets(result, &index_mapping)
}

/// Patch jump offsets after instructions have been removed
fn patch_jump_offsets(
    mut opcodes: Vec<LpsOpCode>,
    index_mapping: &[Option<usize>],
) -> Vec<LpsOpCode> {
    for (i, opcode) in opcodes.iter_mut().enumerate() {
        match opcode {
            LpsOpCode::Jump(offset)
            | LpsOpCode::JumpIfZero(offset)
            | LpsOpCode::JumpIfNonZero(offset) => {
                // Find the original index this instruction was at
                let original_i = index_mapping
                    .iter()
                    .position(|&mapped| mapped == Some(i))
                    .expect("Should find original index");

                // Calculate the original target
                let original_target = (original_i as i32 + 1 + *offset) as usize;

                // Find the new target index
                if let Some(Some(new_target)) = index_mapping.get(original_target) {
                    // Calculate new offset
                    let new_offset = (*new_target as i32) - (i as i32) - 1;
                    *offset = new_offset;
                }
            }
            _ => {}
        }
    }
    opcodes
}

/// Collect all jump targets (instruction indices that are jumped to)
fn collect_jump_targets(opcodes: &[LpsOpCode]) -> alloc::collections::BTreeSet<usize> {
    let mut targets = alloc::collections::BTreeSet::new();

    for (i, op) in opcodes.iter().enumerate() {
        match op {
            LpsOpCode::Jump(offset)
            | LpsOpCode::JumpIfZero(offset)
            | LpsOpCode::JumpIfNonZero(offset) => {
                // Calculate target index (current index + 1 + offset)
                let target = (i as i32 + 1 + offset) as usize;
                if target < opcodes.len() {
                    targets.insert(target);
                }
            }
            _ => {}
        }
    }

    targets
}
