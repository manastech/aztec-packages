use acvm::acir::brillig::Opcode as BrilligOpcode;
use acvm::acir::circuit::brillig::Brillig;

use acvm::brillig_vm::brillig::{BinaryFieldOp, BinaryIntOp};

use crate::instructions::{
    AvmInstruction, AvmOperand, AvmTypeTag, FIRST_OPERAND_INDIRECT, ZEROTH_OPERAND_INDIRECT,
};
use crate::opcodes::AvmOpcode;
use crate::utils::{dbg_print_avm_program, dbg_print_brillig_program};

/// Transpile a Brillig program to AVM bytecode
pub fn brillig_to_avm(brillig: &Brillig) -> Vec<u8> {
    dbg_print_brillig_program(&brillig);

    let mut avm_instrs: Vec<AvmInstruction> = Vec::new();

    // Map Brillig pcs to AVM pcs
    // (some Brillig instructions map to >1 AVM instruction)
    let brillig_pcs_to_avm_pcs = map_brillig_pcs_to_avm_pcs(avm_instrs.len(), brillig);

    // Transpile a Brillig instruction to one or more AVM instructions
    for brillig_instr in &brillig.bytecode {
        match brillig_instr {
            BrilligOpcode::BinaryFieldOp {
                destination,
                op,
                lhs,
                rhs,
            } => {
                let avm_opcode = match op {
                    BinaryFieldOp::Add => AvmOpcode::ADD,
                    BinaryFieldOp::Sub => AvmOpcode::SUB,
                    BinaryFieldOp::Mul => AvmOpcode::MUL,
                    BinaryFieldOp::Div => AvmOpcode::DIV,
                    BinaryFieldOp::Equals => AvmOpcode::EQ,
                };
                // TODO(4268): set in_tag to `field`
                avm_instrs.push(AvmInstruction {
                    opcode: avm_opcode,
                    operands: vec![
                        AvmOperand::U32 {
                            value: lhs.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: rhs.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: destination.to_usize() as u32,
                        },
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::BinaryIntOp {
                destination,
                op,
                bit_size: _, // TODO(4268): support u8..u128 and use in_tag
                lhs,
                rhs,
            } => {
                let avm_opcode = match op {
                    BinaryIntOp::Add => AvmOpcode::ADD,
                    BinaryIntOp::Sub => AvmOpcode::SUB,
                    BinaryIntOp::Mul => AvmOpcode::MUL,
                    BinaryIntOp::UnsignedDiv => AvmOpcode::DIV,
                    BinaryIntOp::Equals => AvmOpcode::EQ,
                    BinaryIntOp::LessThan => AvmOpcode::LT,
                    BinaryIntOp::LessThanEquals => AvmOpcode::LTE,
                    BinaryIntOp::And => AvmOpcode::AND,
                    BinaryIntOp::Or => AvmOpcode::OR,
                    BinaryIntOp::Xor => AvmOpcode::XOR,
                    BinaryIntOp::Shl => AvmOpcode::SHL,
                    BinaryIntOp::Shr => AvmOpcode::SHR,
                    _ => panic!(
                        "Transpiler doesn't know how to process BinaryIntOp {:?}",
                        brillig_instr
                    ),
                };
                // TODO(4268): support u8..u128 and use in_tag
                avm_instrs.push(AvmInstruction {
                    opcode: avm_opcode,
                    operands: vec![
                        AvmOperand::U32 {
                            value: lhs.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: rhs.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: destination.to_usize() as u32,
                        },
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::CalldataCopy { destination_address, size, offset } => {
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::CALLDATACOPY,
                    operands: vec![
                        AvmOperand::U32 {
                            value: *offset as u32, // cdOffset (calldata offset)
                        }, AvmOperand::U32 {
                            value: *size as u32,
                        }, AvmOperand::U32 {
                            value: destination_address.to_usize() as u32, // dstOffset
                        }],
                        ..Default::default()
                    });
            }
            BrilligOpcode::Jump { location } => {
                let avm_loc = brillig_pcs_to_avm_pcs[*location];
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::JUMP,
                    operands: vec![AvmOperand::U32 {
                        value: avm_loc as u32,
                    }],
                    ..Default::default()
                });
            }
            BrilligOpcode::JumpIf {
                condition,
                location,
            } => {
                let avm_loc = brillig_pcs_to_avm_pcs[*location];
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::JUMPI,
                    operands: vec![
                        AvmOperand::U32 {
                            value: avm_loc as u32,
                        },
                        AvmOperand::U32 {
                            value: condition.to_usize() as u32,
                        },
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::Const { destination, value } => {
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::SET,
                    dst_tag: Some(AvmTypeTag::UINT32),
                    operands: vec![
                        // TODO(4267): support u8..u128 and use dst_tag
                        AvmOperand::U32 {
                            value: value.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: destination.to_usize() as u32,
                        },
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::Mov {
                destination,
                source,
            } => {
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::MOV,
                    operands: vec![
                        AvmOperand::U32 {
                            value: source.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: destination.to_usize() as u32,
                        },
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::Load {
                destination,
                source_pointer,
            } => {
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::MOV,
                    indirect: Some(ZEROTH_OPERAND_INDIRECT), // indirect srcOffset operand
                    operands: vec![
                        AvmOperand::U32 {
                            value: source_pointer.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: destination.to_usize() as u32,
                        },
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::Store {
                destination_pointer,
                source,
            } => {
                // INDIRECT dstOffset operand (bit 1 set high)
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::MOV,
                    indirect: Some(FIRST_OPERAND_INDIRECT), // indirect dstOffset operand
                    operands: vec![
                        AvmOperand::U32 {
                            value: source.to_usize() as u32,
                        },
                        AvmOperand::U32 {
                            value: destination_pointer.to_usize() as u32,
                        },
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::Call { location } => {
                let avm_loc = brillig_pcs_to_avm_pcs[*location];
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::INTERNALCALL,
                    operands: vec![AvmOperand::U32 {
                        value: avm_loc as u32,
                    }],
                    ..Default::default()
                });
            }
            BrilligOpcode::Return {} => avm_instrs.push(AvmInstruction {
                opcode: AvmOpcode::INTERNALRETURN,
                ..Default::default()
            }),
            BrilligOpcode::Stop { return_data_offset, return_data_size } => {
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::RETURN,
                    operands: vec![
                        AvmOperand::U32 { value: *return_data_offset as u32},
                        AvmOperand::U32 { value: *return_data_size as u32},
                    ],
                    ..Default::default()
                });
            }
            BrilligOpcode::Trap { /*return_data_offset, return_data_size*/ } => {
                // TODO(https://github.com/noir-lang/noir/issues/3113): Trap should support return data
                avm_instrs.push(AvmInstruction {
                    opcode: AvmOpcode::REVERT,
                    operands: vec![
                        //AvmOperand::U32 { value: *return_data_offset as u32},
                        //AvmOperand::U32 { value: *return_data_size as u32},
                        AvmOperand::U32 { value: 0},
                        AvmOperand::U32 { value: 0},
                    ],
                    ..Default::default()
                });
            }
            _ => panic!(
                "Transpiler doesn't know how to process {:?} brillig instruction",
                brillig_instr
            ),
        }
    }

    dbg_print_avm_program(&avm_instrs);

    // Constructing bytecode from instructions
    let mut bytecode = Vec::new();
    for i in 0..avm_instrs.len() {
        let instr_bytes = avm_instrs[i].to_bytes();
        bytecode.extend_from_slice(&instr_bytes);
    }
    bytecode
}

/// Compute an array that maps each Brillig pc to an AVM pc.
/// This must be done before transpiling to properly transpile jump destinations.
/// This is necessary for two reasons:
/// 1. The transpiler injects `initial_offset` instructions at the beginning of the program.
/// 2. Some brillig instructions (_e.g._ Stop, or certain ForeignCalls) map to multiple AVM instructions
/// args:
///     initial_offset: how many AVM instructions were inserted at the start of the program
///     brillig: the Brillig program
/// returns: an array where each index is a Brillig pc,
///     and each value is the corresponding AVM pc.
fn map_brillig_pcs_to_avm_pcs(initial_offset: usize, brillig: &Brillig) -> Vec<usize> {
    let mut pc_map = Vec::with_capacity(brillig.bytecode.len());
    pc_map.resize(brillig.bytecode.len(), 0);
    pc_map[0] = initial_offset;
    for i in 0..brillig.bytecode.len() - 1 {
        let num_avm_instrs_for_this_brillig_instr = match &brillig.bytecode[i] {
            BrilligOpcode::Load { .. } => 2,
            BrilligOpcode::Store { .. } => 2,
            _ => 1,
        };
        // next Brillig pc will map to an AVM pc offset by the
        // number of AVM instructions generated for this Brillig one
        pc_map[i + 1] = pc_map[i] + num_avm_instrs_for_this_brillig_instr;
    }
    pc_map
}
