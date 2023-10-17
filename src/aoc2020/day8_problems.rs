mod day8 {
  use std::ops::Add;

  #[derive(PartialEq, Debug, Copy, Clone)]
  pub enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
  }

  impl Instruction {}

  #[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
  pub(crate) struct PcCounter {
    v: i32,
  }

  impl PcCounter {
    pub fn zero() -> Self {
      Self { v: 0 }
    }
  }
  impl From<i32> for PcCounter {
    fn from(v: i32) -> Self {
      PcCounter { v }
    }
  }
  impl From<PcCounter> for usize {
    fn from(v: PcCounter) -> usize {
      v.v as usize
    }
  }

  impl Add<i32> for PcCounter {
    type Output = Self;
    fn add(self, other: i32) -> Self {
      Self { v: self.v + other }
    }
  }
  #[derive(PartialEq, Debug, Copy, Clone)]
  pub struct AccValue {
    v: i32,
  }

  impl AccValue {
    pub fn zero() -> Self {
      Self { v: 0 }
    }
  }

  impl Add<i32> for AccValue {
    type Output = Self;

    fn add(self, other: i32) -> Self {
      Self { v: self.v + other }
    }
  }

  impl From<AccValue> for i32 {
    fn from(v: AccValue) -> i32 {
      v.v
    }
  }
  impl From<i32> for AccValue {
    fn from(v: i32) -> AccValue {
      AccValue { v }
    }
  }

  pub(crate) struct CpuState {
    pub(crate) pc: PcCounter,
    pub(crate) acc_value: AccValue,
  }
}

mod day8parsing {
  use crate::aoc2020::day8_problems::day8::*;

  fn parse_instruction_op(operand: i32, s: &str) -> Instruction {
    match s {
      "jmp" => Instruction::Jmp(operand),
      "nop" => Instruction::Nop(operand),
      "acc" => Instruction::Acc(operand),
      _ => panic!("unknown instruction bang!"),
    }
  }

  pub fn parse_instruction(s: &str) -> Instruction {
    let parts = s.split_whitespace().collect::<Vec<&str>>();
    let sign_multiplier = if parts[1].starts_with('-') { -1 } else { 1 };
    let operand = parts[1].parse().unwrap();
    parse_instruction_op(operand, parts[0])
  }

  pub fn parse_program(s: &str) -> Vec<Instruction> {
    s.lines().map(|x| parse_instruction(x)).collect()
  }
}

mod part1 {
  use super::day8::{AccValue, CpuState, Instruction, PcCounter};
  use super::day8parsing::parse_program;
  use std::collections::HashSet;

  pub(crate) fn execute_instruction(i: Instruction, state: CpuState) -> CpuState {
    match i {
      Instruction::Nop(_i) => CpuState {
        pc: state.pc + 1,
        ..state
      },
      Instruction::Acc(v) => CpuState {
        pc: state.pc + 1,
        acc_value: state.acc_value + v,
      },
      Instruction::Jmp(v) => CpuState {
        pc: state.pc + v,
        ..state
      },
    }
  }

  pub fn prob1(s: &str) -> AccValue {
    fn run_cpu_until_repeat(
      mut address_history: HashSet<PcCounter>,
      cpu_state: CpuState,
      program: Vec<Instruction>,
    ) -> AccValue {
      if address_history.contains(&cpu_state.pc) {
        cpu_state.acc_value
      } else {
        address_history.insert(cpu_state.pc);
        let program_index: usize = cpu_state.pc.into();
        let cpu_state_dash = execute_instruction(program[program_index], cpu_state);
        run_cpu_until_repeat(address_history, cpu_state_dash, program)
      }
    }
    let prog = parse_program(s);
    let cpu_state = CpuState {
      pc: PcCounter::zero(),
      acc_value: AccValue::zero(),
    };
    run_cpu_until_repeat(HashSet::new(), cpu_state, prog)
  }
}

#[cfg(test)]
mod day8_tests {
  use super::day8::*;
  use super::day8parsing::parse_instruction;
  use super::part1::*;

  #[test]
  fn test_char_with_sample_line2() {
    let input = "nop +0";
    let result = parse_instruction(input);
    assert_eq!(result, Instruction::Nop(0));
  }

  #[test]
  fn test_char_with_sample_line2_negative() {
    let input = "nop -1";
    let result = parse_instruction(input);
    assert_eq!(result, Instruction::Nop(-1));
  }

  #[test]
  fn pc_addition_test() {
    let input = PcCounter::zero();
    let input2 = input + 1;
    assert_eq!(input2, 1.into());
  }

  #[test]
  fn test_sample_program() {
    let input = "nop +0\n\
    acc +1\n\
    jmp +4\n\
    acc +3\n\
    jmp -3\n\
    acc -99\n\
    acc +1\n\
    jmp -4\n\
    acc +6\n";
    let result = prob1(input);
    assert_eq!(result, 5.into());
  }
}
