use super::{I, IBuilder};

pub enum Disassembly {
    Data {
        data: Vec<u8>, // Note: Vectors are not optimized for this use case.
        len: u8,
        offset: u8,
    },
    Instruction {
        instruction: I,
        len: u8,
        offset: u8,
    },
}

impl Disassembly {
    pub fn offset(&self) -> u8 {
        match self {
            Disassembly::Data { offset, .. } => *offset,
            Disassembly::Instruction { offset, .. } => *offset,
        }
    }

    pub fn len(&self) -> u8 {
        match self {
            Disassembly::Data { len, .. } => *len,
            Disassembly::Instruction { len, .. } => *len,
        }
    }
}

pub fn disassemble(bytes: &[u8]) -> Vec<Disassembly> {
    let mut disassembly = vec![
        Disassembly::Data { data: bytes.to_vec(), len: bytes.len() as u8, offset: 0 }
    ];

    let mut stack = vec![0_u8];
    'block: while let Some(offset) = stack.pop() {
        let mut index = disassembly.iter().position(|d| match d {
            Disassembly::Data { offset: o, len, .. } => offset >= *o && offset < o + len,
            Disassembly::Instruction { offset: o, len, .. } => offset >= *o && offset < o + len,
        }).unwrap();

        loop {
            if let Disassembly::Data { ref mut data, ref mut len, ref mut offset } = disassembly[index] {
                let instruction = match I::from_opcode(data[0]) {
                    IBuilder::Complete(instruction) => instruction,
                    IBuilder::NeedsData(incomplete) if  *len > 1 => incomplete.with_data(data[1]),
                    IBuilder::NeedsData(_) => continue 'block,
                    IBuilder::Invalid => continue 'block,
                };

                let instruction_len = match instruction {
                    I::Nop(..) | I::Hlt(..) | I::Out(..) => 1,
                    I::Ldi(..) | I::Lda(..) | I::Sta(..) | I::Add(..) | I::Sub(..) => 2,
                    I::Jmp(..) | I::Jpz(..) | I::Jpc(..) => {
                        if (data[1] as usize) < bytes.len() {
                            stack.push(data[1]);
                        }
                        2
                    },
                };

                let instruction = Disassembly::Instruction {
                    instruction,
                    len: instruction_len,
                    offset: *offset,
                };

                *offset = *offset + instruction_len;
                *data = data.split_off(instruction_len as usize);
                *len = *len - instruction_len;

                disassembly.insert(index, instruction);
                index += 1;
            } else {
                continue 'block;
            }
        }
    }

    disassembly
}
