use std::{collections::{HashMap, HashSet}, ops::Range};

type Register = u8;
type Immediate = u8;
type Address = u16;
type Offset = i8;

pub struct EncodedInstruction(u16);

impl EncodedInstruction {
    pub fn read_bits(&self, range: Range<usize>) -> u16 {
        let shifted = self.0 >> (u16::BITS as usize - range.end);
        let mask = ((1 << (range.end - range.start)) - 1) as u16;
        shifted & mask
    }

    pub fn opcode(&self) -> u8 {
        self.read_bits(0..4) as u8
    }

    pub fn reg_a(&self) -> u8 {
        self.read_bits(4..8) as u8
    }

    pub fn reg_b(&self) -> u8 {
        self.read_bits(8..12) as u8
    }

    pub fn reg_c(&self) -> u8 {
        self.read_bits(12..16) as u8
    }

    pub fn immediate(&self) -> u8 {
        self.read_bits(8..16) as u8
    }

    pub fn address(&self) -> u16 {
        self.read_bits(6..16)
    }

    pub fn offset(&self) -> i8 {
        // TODO: I'm pretty sure there's a cleaner way of doing this
        let a = self.read_bits(13..16) as u8;
        if self.read_bits(12..13) != 0 {
            -8 + a as i8
        } else {
            a as i8
        }
    }

    fn condition(&self) -> Condition {
        Condition::from_bits(self.read_bits(4..6))
    }
}

#[derive(Debug)]
pub enum Instruction {
    Nop,
    Hlt,
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Nor(Register, Register, Register),
    And(Register, Register, Register),
    Xor(Register, Register, Register),
    Rsh(Register, Register),
    Ldi(Register, Immediate),
    Adi(Register, Immediate),
    Jmp(Address),
    Brh(Condition, Address),
    Cal(Address),
    Ret,
    Lod(Register, Register, Offset),
    Str(Register, Register, Offset)
}

impl Instruction {
    pub fn from_instruction(instruction: EncodedInstruction) -> Self {
        static DESERIALISERS: [fn(EncodedInstruction) -> Instruction; 16] = [
            |_| Instruction::Nop,
            |_| Instruction::Hlt,
            |i| Instruction::Add(i.reg_a(), i.reg_b(), i.reg_c()),
            |i| Instruction::Sub(i.reg_a(), i.reg_b(), i.reg_c()),
            |i| Instruction::Nor(i.reg_a(), i.reg_b(), i.reg_c()),
            |i| Instruction::And(i.reg_a(), i.reg_b(), i.reg_c()),
            |i| Instruction::Xor(i.reg_a(), i.reg_b(), i.reg_c()),
            |i| Instruction::Rsh(i.reg_a(), i.reg_c()),
            |i| Instruction::Ldi(i.reg_a(), i.immediate()),
            |i| Instruction::Adi(i.reg_a(), i.immediate()),
            |i| Instruction::Jmp(i.address()),
            |i| Instruction::Brh(i.condition(), i.address()),
            |i| Instruction::Cal(i.address()),
            |_| Instruction::Ret,
            |i| Instruction::Lod(i.reg_a(), i.reg_b(), i.offset()),
            |i| Instruction::Str(i.reg_a(), i.reg_b(), i.offset()),
        ];

        DESERIALISERS[instruction.opcode() as usize](instruction)
    }

    pub fn to_nasm(&self, label_map: &HashMap<u16, String>) -> String {
        fn get_dest_str(reg: u8) -> String {
            if reg == 0 {
                "al".into()
            } else {
                format!("[reg + {reg}]")
            }
        }

        match self {
            Instruction::Nop => include_str!("intrinsics/nop.asm").into(),
            Instruction::Hlt => include_str!("intrinsics/hlt.asm").into(),
            Instruction::Add(a, b, c) => {
                let dest = get_dest_str(*c);

                format!(include_str!("intrinsics/add.asm"), a = a, b = b, dest = dest)
                // format!("\tmov cl, [reg + {a}]\n\tmov dl, [reg + {b}]\n\tmov {dest}, cl\n\tadd {dest}, dl")
            },
            Instruction::Sub(a, b, c) => {
                let dest = get_dest_str(*c);
                
                format!(include_str!("intrinsics/sub.asm"), a = a, b = b, dest = dest)
                // format!("\tmov cl, [reg + {a}]\n\tmov dl, [reg + {b}]\n\tmov {dest}, cl\n\tsub {dest}, dl")
            },
            Instruction::Nor(a, b, c) => {
                let dest = get_dest_str(*c);
                
                format!(include_str!("intrinsics/nor.asm"), a = a, b = b, dest = dest)
                // format!("\tmov cl, [reg + {a}]\n\tmov dl, [reg + {b}]\n\tmov {dest}, cl\n\tor {dest}, dl\n\tnot byte {dest}")
            },
            Instruction::And(a, b, c) => {
                let dest = get_dest_str(*c);
                
                format!(include_str!("intrinsics/and.asm"), a = a, b = b, dest = dest)
            },
            Instruction::Xor(a, b, c) => {
                let dest = get_dest_str(*c);
                format!(include_str!("intrinsics/xor.asm"), a = a, b = b, dest = dest)
                // format!("\tmov cl, [reg + {a}]\n\tmov dl, [reg + {b}]\n\tmov {dest}, cl\n\txor {dest}, dl")
            },
            Instruction::Rsh(a, c) => {
                let dest = get_dest_str(*c);
                
                format!(include_str!("intrinsics/rsh.asm"), a = a, dest = dest)
                // format!("\tmov cl, [reg + {a}]\n\tshr cl, 1\n\tmov {dest}, cl")
            },
            Instruction::Ldi(a, i) => {
                let dest = get_dest_str(*a);
                format!(include_str!("intrinsics/ldi.asm"), i = i, dest = dest)
            },
            Instruction::Adi(a, i) => {
                let dest = get_dest_str(*a);
                format!(include_str!("intrinsics/adi.asm"), i = i, dest = dest)
            },
            Instruction::Jmp(a) => format!(include_str!("intrinsics/jmp.asm"), l = label_map[a]),
            Instruction::Brh(c, a) => match c {
                Condition::Equal => format!(include_str!("intrinsics/brh/eq.asm"), l = label_map[a]),
                Condition::NotEqual => format!(include_str!("intrinsics/brh/ne.asm"), l = label_map[a]),
                Condition::GreaterThanOrEqual =>  format!(include_str!("intrinsics/brh/ge.asm"), l = label_map[a]),
                Condition::LessThan => format!(include_str!("intrinsics/brh/lt.asm"), l = label_map[a]),
            },
            Instruction::Cal(a) => format!(include_str!("intrinsics/cal.asm"), l = label_map[a]),
            Instruction::Ret => include_str!("intrinsics/ret.asm").into(),
            Instruction::Lod(a, b, o) => {
                let dest = get_dest_str(*b);
                format!(include_str!("intrinsics/lod.asm"), a = a, o = o, dest = dest)
                // format!("\tmov r8, reg\n\tmov cl, [reg + {b}]\n\tadd cl, {o}\n\tmovzx rcx, cl\n\tmov dl, [r8 + rcx]\n\tmov {dest}, dl") "\tmov cl, [reg + {a}]\n\tadd cl, {o}\n\tmovzx rcx, cl\n\tmov dl, [r8 + rcx]\n\tmov {dest}, dl")
            },
            Instruction::Str(a, b, o) => {
                format!(include_str!("intrinsics/str.asm"), a = a, b = b, o = o)
            },
        }
    }
}

#[derive(Debug)]
pub enum Condition {
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThan,
}

impl Condition {
    fn from_bits(bits: u16) -> Self {
        match bits {
            0b00 => Condition::Equal,
            0b01 => Condition::NotEqual,
            0b10 => Condition::GreaterThanOrEqual,
            0b11 => Condition::LessThan,
            _ => unreachable!()
        }
    }
}

pub fn disassemble(bin: &[u16]) -> Vec<Instruction> {
    bin.iter()
        .map(|&value| EncodedInstruction(value))
        .map(Instruction::from_instruction)
        .collect()
}

pub fn parse_mc_file(src: &str) -> Vec<u16> {
    src.lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect()
}

pub fn transpile(src: &str) -> String {
    let parsed = parse_mc_file(src);
    let instructions = disassemble(&parsed);
    let labels = find_labels(&instructions);

    let label_map = labels.iter()
        .enumerate()
        .map(|(i, &addr)| (addr, format!("label_{i}")))
        .collect::<HashMap<_, _>>();


    let mut output = String::new();
    for (i, instruction) in instructions.iter().enumerate() {
        if let Some(label) = label_map.get(&(i as u16)) {
            output += &format!("{label}:\n");
        }
        output += &format!("{}\n", instruction.to_nasm(&label_map));
    }

    format!("{}\n{output}\n", include_str!("asm_header.s"))
}

fn find_labels(instructions: &[Instruction]) -> Vec<u16> {
    let mut labels = HashSet::new();

    for instruction in instructions.iter() {
        match instruction {
            Instruction::Jmp(addr) => {
                labels.insert(addr);
            },
            Instruction::Brh(_, addr) => {
                labels.insert(addr);
            },
            Instruction::Cal(addr) => {
                labels.insert(addr);
            },
            _ => {}
        }
    }

    let mut r = labels.into_iter()
        .copied()
        .collect::<Vec<_>>();

    r.sort();
    r
}
