use std::{fs::File, io::Read};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Operator {
  INCERMENT= b'+' as isize,
  DECERMENT= b'-' as isize,
  MOVELEFT= b'<' as isize,
  MOVERIGHT= b'>' as isize,
  PERIOD= b'.' as isize,
  COMMA= b',' as isize,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum LoopChar {
  LEFTSQAUREBRACKET=91,
  RIGHTSQAUREBRACKET=93,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Symbol {
  Operator(Operator),
  LoopChar(LoopChar),
  Comment,
}

impl From<u8> for Symbol {
  fn from(x: u8) -> Self {
    match x {
      b'+' => Symbol::Operator(Operator::INCERMENT),
      b'-' => Symbol::Operator(Operator::DECERMENT),
      b'>' => Symbol::Operator(Operator::MOVERIGHT),
      b'<' => Symbol::Operator(Operator::MOVELEFT),
      b'.' => Symbol::Operator(Operator::PERIOD),
      b',' => Symbol::Operator(Operator::COMMA),
      b'[' => Symbol::LoopChar(LoopChar::LEFTSQAUREBRACKET),
      b']' => Symbol::LoopChar(LoopChar::RIGHTSQAUREBRACKET),
      _ => Symbol::Comment,
    }
  }
}


fn execute_operator(operator: &Operator, program_mem: &mut (Vec<u8>, usize)) {
  match *operator {
    Operator::INCERMENT => (*program_mem).0[(*program_mem).1] = (*program_mem).0[(*program_mem).1].wrapping_add(1),
    Operator::DECERMENT => (*program_mem).0[(*program_mem).1] = (*program_mem).0[(*program_mem).1].wrapping_sub(1),
    Operator::MOVERIGHT => (*program_mem).1 += 1,
    Operator::MOVELEFT => (*program_mem).1 -= 1,
    Operator::PERIOD => print!("{}", (*program_mem).0[((*program_mem).1)] as char),
    Operator::COMMA => {
      let mut buffer = String::new();
      std::io::stdin()
        .read_line(&mut buffer)
        .unwrap();
      buffer.pop();
      (*program_mem).0[(*program_mem).1] = buffer.parse::<u8>()
        .unwrap();
    }
  }
}

fn main() {  
  #![allow(unused)]

  let mut args: Vec<String> = std::env::args().collect();
  if args.len() == 1 {panic!("No file supplied")}
  let mut file =  File::open(&mut args[1]
    .as_mut_str())
    .expect("failed to open file");
  let mut bytes = vec![0; 100000];  
  file.read(&mut bytes).unwrap();

  let mut program_mem = (vec![0u8; 10000], 0usize);

  let mut instructions = Vec::new();
  for elem in bytes {
    instructions.push(Symbol::from(elem));
  };

  let mut index = 0usize;
  let mut loop_jumps = Vec::new();
  while index < instructions.len() {
    match instructions[index] {
      Symbol::Operator(x) => execute_operator(&x, &mut program_mem),
      Symbol::LoopChar(LoopChar::LEFTSQAUREBRACKET) => loop_jumps.push(index + 1),
      Symbol::LoopChar(LoopChar::RIGHTSQAUREBRACKET) => {
        if program_mem.0[program_mem.1] == 0 {
          loop_jumps.pop();
        }else {
          index = loop_jumps[loop_jumps.len() -1] - 1;
        }
      }
      _ => {}
    }
    index += 1;
  }
}

