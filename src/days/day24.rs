use std::collections::HashSet;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::parse::parse_signed;

type SolverInput = Vec<Instruction>;

const INPUT_LENGTH: usize = 14;

#[derive(Clone, Copy, PartialEq)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
pub enum Operand {
    Variable(Register),
    Value(i64),
}

#[derive(Clone, Copy)]
pub enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
    Clear(Register),
    Set(Register, Operand),
    Neql(Register, Operand),
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Alu {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "x"),
            Self::Y => write!(f, "y"),
            Self::Z => write!(f, "z"),
            Self::W => write!(f, "w"),
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Variable(reg) => write!(f, "{}", reg),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inp(reg) => write!(f, "inp {}", reg),
            Self::Add(reg, op) => write!(f, "add {} {}", reg, op),
            Self::Mul(reg, op) => write!(f, "mul {} {}", reg, op),
            Self::Div(reg, op) => write!(f, "div {} {}", reg, op),
            Self::Mod(reg, op) => write!(f, "mod {} {}", reg, op),
            Self::Eql(reg, op) => write!(f, "eql {} {}", reg, op),
            Self::Clear(reg) => write!(f, "clear {}", reg),
            Self::Set(reg, op) => write!(f, "set {} {}", reg, op),
            Self::Neql(reg, op) => write!(f, "neql {} {}", reg, op),
        }
    }
}

impl Instruction {
    fn discards_register(&self) -> Option<Register> {
        match self {
            Self::Inp(reg) => Some(*reg),
            Self::Clear(reg) => Some(*reg),
            Self::Set(reg, Operand::Value(_)) => Some(*reg),
            Self::Set(reg1, Operand::Variable(reg2)) if reg1 != reg2 => Some(*reg1),
            _ => None,
        }
    }

    fn uses_register_val(&self, reg: Register) -> bool {
        match self {
            &Self::Add(reg1, Operand::Variable(reg2))
            | &Self::Mul(reg1, Operand::Variable(reg2))
            | &Self::Div(reg1, Operand::Variable(reg2))
            | &Self::Mod(reg1, Operand::Variable(reg2))
            | &Self::Eql(reg1, Operand::Variable(reg2))
            | &Self::Neql(reg1, Operand::Variable(reg2))
                if reg1 == reg || reg2 == reg =>
            {
                true
            }
            &Self::Add(rego, _)
            | &Self::Mul(rego, _)
            | &Self::Div(rego, _)
            | &Self::Mod(rego, _)
            | &Self::Eql(rego, _)
            | &Self::Set(_, Operand::Variable(rego))
            | &Self::Neql(rego, _)
                if rego == reg =>
            {
                true
            }
            _ => false,
        }
    }
}

impl Alu {
    fn reg(&self, register: Register) -> i64 {
        match register {
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
            Register::W => self.w,
        }
    }

    fn reg_mut(&mut self, register: Register) -> &mut i64 {
        match register {
            Register::X => &mut self.x,
            Register::Y => &mut self.y,
            Register::Z => &mut self.z,
            Register::W => &mut self.w,
        }
    }

    fn op_to_val(&self, op: Operand) -> i64 {
        match op {
            Operand::Value(v) => v,
            Operand::Variable(r) => self.reg(r),
        }
    }

    // requires Some input
    fn run_instruction(&mut self, instruction: Instruction, input: &mut Option<i64>) -> Result<()> {
        use Instruction::*;
        match instruction {
            Inp(reg) => {
                *self.reg_mut(reg) = input
                    .take()
                    .ok_or(anyhow!("Inp received in ALU with no input"))?;
            }
            Add(reg, op) => {
                *self.reg_mut(reg) = self.reg(reg) + self.op_to_val(op);
            }
            Mul(reg, op) => {
                *self.reg_mut(reg) = self.reg(reg) * self.op_to_val(op);
            }
            Div(reg, op) => {
                *self.reg_mut(reg) = self.reg(reg) / self.op_to_val(op);
            }
            Mod(reg, op) => {
                *self.reg_mut(reg) = self.reg(reg) % self.op_to_val(op);
            }
            Eql(reg, op) => {
                *self.reg_mut(reg) = (self.reg(reg) == self.op_to_val(op)) as i64;
            }
            Clear(reg) => {
                *self.reg_mut(reg) = 0;
            }
            Set(reg, op) => *self.reg_mut(reg) = self.op_to_val(op),
            Neql(reg, op) => {
                *self.reg_mut(reg) = (self.reg(reg) != self.op_to_val(op)) as i64;
            }
        }
        Ok(())
    }

    // Returns remaining instructions
    fn run_for_input<'a>(
        &mut self,
        instructions: &'a [Instruction],
        input: i64,
    ) -> &'a [Instruction] {
        let mut input = Some(input);
        for (i, ins) in instructions.iter().enumerate() {
            if input.is_none() {
                if let Instruction::Inp(_) = ins {
                    return &instructions[i..];
                }
            }
            let _ = self.run_instruction(*ins, &mut input);
        }
        &[]
    }
}

fn optimize_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
    use Instruction::*;
    use Operand::*;
    type VI = Vec<Instruction>;

    fn remove_noops(ins: &mut VI) -> bool {
        let old_len = ins.len();
        ins.retain(|i| match i {
            Mul(_, Value(1)) => false,
            Div(_, Value(1)) => false,
            _ => true,
        });
        old_len != ins.len()
    }
    fn simplify_instructions(ins: &mut VI) -> bool {
        let mut ret = false;
        for i in ins.iter_mut() {
            match i {
                Mul(reg, Value(0)) => {
                    *i = Clear(*reg);
                    ret = true;
                }
                _ => (),
            }
        }
        let mut i = 0;
        while i + 1 < ins.len() {
            match (ins[i], ins[i + 1]) {
                (Clear(reg1), Add(reg2, op)) if reg1 == reg2 => {
                    ins[i] = Set(reg1, op);
                    ins.remove(i + 1);
                    ret = true;
                }
                (Eql(reg1, op), Eql(reg2, Value(0))) if reg1 == reg2 => {
                    ins[i] = Neql(reg1, op);
                    ins.remove(i + 1);
                    ret = true;
                }
                _ => (),
            }
            i += 1;
        }
        ret
    }
    fn insert_implicit_clears(ins: &mut VI) -> bool {
        let mut pos = vec![];
        for i in 0..ins.len() {
            let discarded_reg = match ins[i] {
                Instruction::Clear(_) => continue, // Skip clears
                inst @ _ => inst.discards_register(),
            };
            if discarded_reg.is_none() {
                continue;
            }
            let reg = discarded_reg.unwrap();
            for j in (0..i).rev() {
                if discarded_reg == ins[j].discards_register() {
                    break;
                }
                if ins[j].uses_register_val(reg) {
                    pos.push((j + 1, reg));
                    break;
                }
            }
        }
        for (posi, &(i, reg)) in pos.iter().enumerate() {
            ins.insert(i + posi, Instruction::Clear(reg));
        }
        !pos.is_empty()
    }

    let mut instructions = instructions;
    loop {
        let something_changed = remove_noops(&mut instructions)
            || simplify_instructions(&mut instructions)
            || insert_implicit_clears(&mut instructions);
        if !something_changed {
            // Insert the clears on end for variables we do not check
            instructions.push(Clear(Register::X));
            instructions.push(Clear(Register::Y));
            instructions.push(Clear(Register::W));
            break instructions;
        }
    }
}

fn find_first_z_zero<'a, I>(
    instructions: &[Instruction],
    alu: Alu,
    depth: Option<usize>,
    inputs: I,
    known_alus: &mut HashSet<(Alu, usize)>,
) -> Option<i64>
where
    I: IntoIterator<Item = &'a i64> + Clone,
{
    let depth = depth.unwrap_or(1);
    let inputs_copy = inputs.clone();

    if known_alus.contains(&(alu, depth)) {
        return None;
    }

    for digit in inputs {
        let mut new_alu = alu;
        let remaining = new_alu.run_for_input(instructions, *digit);
        if depth == INPUT_LENGTH {
            if new_alu.z == 0 {
                return Some(*digit);
            }
        } else if let Some(val) = find_first_z_zero(
            remaining,
            new_alu,
            Some(depth + 1),
            inputs_copy.clone(),
            known_alus,
        ) {
            // found solution
            let pos_from_left = INPUT_LENGTH - depth;
            let mul = 10i64.pow(pos_from_left as u32);
            return Some((digit * mul) + val);
        }
    }

    known_alus.insert((alu, depth));
    None
}

fn par_find_first_z_zero<'a, 'b, I: 'a>(instructions: &[Instruction], inputs: I) -> Option<i64>
where
    I: IntoIterator<Item = &'b i64>
        + IntoParallelIterator<Item = &'b i64>
        + Clone
        + std::marker::Sync,
{
    let inputs_clone = inputs.clone();
    inputs.into_par_iter().find_map_first(|digit| {
        let mut alu = Alu::default();
        let instructions = alu.run_for_input(instructions, *digit);
        find_first_z_zero(
            instructions,
            alu,
            Some(2),
            inputs_clone.clone(),
            &mut HashSet::new(),
        )
        .map(|v| v + (digit * 10i64.pow((INPUT_LENGTH - 1) as u32)))
    })
}

pub fn parse_input(file: &[u8]) -> Result<SolverInput> {
    fn parse_register(input: &[u8]) -> IResult<&[u8], Register> {
        let (rest, sign) = alt((tag(b"x"), tag(b"y"), tag(b"z"), tag(b"w")))(input)?;
        let reg = match sign[0] {
            b'x' => Register::X,
            b'y' => Register::Y,
            b'z' => Register::Z,
            b'w' => Register::W,
            _ => unreachable!(),
        };
        Ok((rest, reg))
    }
    fn parse_operand(input: &[u8]) -> IResult<&[u8], Operand> {
        if let Ok((rest, num)) = parse_signed(input) {
            Ok((rest, Operand::Value(num)))
        } else {
            parse_register(input).map(|t| (t.0, Operand::Variable(t.1)))
        }
    }
    let inp = |text| -> IResult<&[u8], Instruction> {
        let (rest, reg) = preceded(tag(b"inp "), parse_register)(text)?;
        Ok((rest, Instruction::Inp(reg)))
    };
    let dual_operand = |text| -> IResult<&[u8], Instruction> {
        let opcode = alt((
            tag(b"add"),
            tag(b"mul"),
            tag(b"div"),
            tag(b"mod"),
            tag(b"eql"),
        ));
        let operands = separated_pair(parse_register, tag(b" "), parse_operand);
        let (rest, (op, arg)) = separated_pair(opcode, tag(b" "), operands)(text)?;
        match op {
            b"add" => Ok((rest, Instruction::Add(arg.0, arg.1))),
            b"mul" => Ok((rest, Instruction::Mul(arg.0, arg.1))),
            b"div" => Ok((rest, Instruction::Div(arg.0, arg.1))),
            b"mod" => Ok((rest, Instruction::Mod(arg.0, arg.1))),
            b"eql" => Ok((rest, Instruction::Eql(arg.0, arg.1))),
            _ => unreachable!(),
        }
    };
    let parse_instruction = alt((inp, dual_operand));

    separated_list1(tag(b"\n"), parse_instruction)(file)
        .map_err(|_| anyhow!("Failed parsing instructions"))
        .map(|t| optimize_instructions(t.1))
}

pub fn solve_part1(input: &SolverInput) -> i64 {
    let digits = (1..10).rev().collect::<Vec<_>>();
    par_find_first_z_zero(input, &digits[..]).unwrap_or(0)
}

pub fn solve_part2(input: &SolverInput) -> i64 {
    let digits = (1..10).collect::<Vec<_>>();
    par_find_first_z_zero(input, &digits[..]).unwrap_or(0)
}
