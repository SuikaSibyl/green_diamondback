use std::env;
use std::fs::File;
use std::io::prelude::*;

use sexp::Atom::*;
use sexp::*;

use im::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum Val {
    Reg(Reg),
    Imm(i64),
    Const(i64),
    RegOffset(Reg, i64),
}

#[derive(Debug, Clone, Copy)]
enum Reg {
    RAX,
    RBX,
    RCX,
    RSP,
    RDI,
}

#[derive(Debug)]
enum Instr {
    IMov(Val, Val),
    IAdd(Val, Val),
    ISub(Val, Val),
    IMul(Val, Val),
    Cmp(Val, Val),
    Jmp(String),
    Jne(String),
    Je(String),
    Jge(String),
    Jle(String),
    And(Val, Val),
    CMOV(Val, Val),
    Label(String),
    Sar(Val, Val),
    Jo(String),
    Call(String),
    Return(),
    ICMovo(Val, Val),
    ICMovne(Val, Val),
}

#[derive(Debug)]
enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
}

#[derive(Debug)]
enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug)]
struct Program {
  defs: Vec<Definition>,
  main: Expr,
}

#[derive(Debug)]
enum Definition {
    Func(String, Vec<String>, Expr),
}

#[derive(Debug)]
enum Expr {
    Number(i64),
    Boolean(bool),
    Input(),
    Id(String),
    Let(Vec<(String, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    Set(String, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Call(String, Vec<Expr>),
    Print(Box<Expr>),
}

const TRUE_CONST: i64 = 3;
const FALSE_CONST: i64 = 1;

const ERRCODE_INVALID_ARG: i64 = 1;
const ERRCODE_OVERFLOW: i64 = 2;

const RESERVED_WORDS: [&'static str; 23] = [
  "true", 
  "false", 
  "input", 
  "let", 
  "set!", 
  "if", 
  "block", 
  "loop", 
  "break", 
  "add1", 
  "sub1", 
  "isnum",
  "isbool",
  "print",
  "fun",
  "+",
  "-",
  "*",
  "<",
  ">",
  ">=",
  "<=",
  "=",
];

const RESERVED_LABELS: [&'static str; 4] = [
  "throw_error",
  "snek_print",
  "snek_error",
  "our_code_starts_here",
];

fn new_label(l: &mut i64, s: &str) -> String {
    let current = *l;
    *l += 1;
    format!("{}{}", s, current)
}

fn parse_bind(s: &Sexp) -> (String, Expr) {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(s)), e] => {
                if RESERVED_WORDS.contains(&s.as_str()) {
                    panic!("parse error: Invalid keyword \"{:?}\" matches reserved word", s);
                }
                (s.clone(), parse_expr(e))
            }
            _ => panic!("parse error: Invalid bind \"{:?}\"", s),
        },
        _ => panic!("parse error: Invalid bind \"{:?}\"", s),
    }
}

fn is_func_define(s: &Sexp) -> bool {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(s)), Sexp::List(_), _] if s == "fun" => true,
            _ => false,
        },
        _ => false,
    }
}

fn parse_program(s: &Sexp) -> Program {
  match s {
    Sexp::List(vec) => {
      let mut defs: Vec<Definition> = vec![];
      for (i, sub_expr) in vec.iter().enumerate() {
        if is_func_define(sub_expr) {
          defs.push(parse_definition(sub_expr));
        }
        else {
          if i != vec.len() - 1 {
            panic!("parse error: Invalid program, main is not the last element")
          }
          let main = parse_expr(sub_expr);
          return Program { defs, main };
        }
      }
      panic!("parse error: Invalid program, find not main: \"{:?}\"", s)
    }
    _ => panic!("parse error: Invalid program, program is not a list: \"{:?}\"", s),
  }
}

fn parse_definition(s: &Sexp) -> Definition {
    match s {
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(name)), Sexp::List(arg_names), e] => {
                let mut args = vec![];
                for arg_name in arg_names {
                    match arg_name {
                        Sexp::Atom(S(s)) => {
                            if RESERVED_WORDS.contains(&s.as_str()) {
                                panic!("parse error: Invalid keyword \"{:?}\" matches reserved word", s);
                            }
                            args.push(s.clone());
                        }
                        _ => panic!("parse error: Invalid arg \"{:?}\"", arg_name),
                    }
                }
                if args.len() == 0 {
                    panic!("parse error: Invalid function definition without function name");
                }
                if RESERVED_LABELS.contains(&args[0].as_str()) {
                    panic!("parse error: Invalid function definition with reserved label function name");
                }
                println!("Fn name: {name}");
                Definition::Func(name.clone(), args, parse_expr(e))
            }
            _ => panic!("parse error: Invalid definition \"{:?}\"", s),
        },
        _ => panic!("parse error: Invalid definition \"{:?}\"", s),
    }
}

fn depth(e: &Expr) -> i32 {
  match e {
    Expr::Number(_) => 0,
    Expr::Boolean(_) => 0,
    Expr::Input() => 0,
    Expr::Id(_) => 0,
    Expr::Let(bindings, body) => {
      let mut d = bindings.len() as i32;  // bindings depth
      for (i, (_, expr)) in bindings.iter().enumerate() {
        d = d.max(depth(expr) + i as i32); // binding expr depth
      }
      d + depth(body)
    },
    Expr::UnOp(_, expr) => depth(expr),
    Expr::BinOp(_, lhs, rhs) => depth(rhs).max(1 + depth(lhs)),
    Expr::Set(_, expr) => depth(expr),
    Expr::If(cond, thn, els) => depth(cond).max(depth(thn)).max(depth(els)),
    Expr::Block(exprs) => {
      let mut d = 0;
      for expr in exprs {
        d = d.max(depth(expr))
      }
      d
    },
    Expr::Loop(expr) => depth(expr),
    Expr::Break(expr) => depth(expr),
    Expr::Call(_, exprs) => {
      let mut d = exprs.len() as i32;
      for (i, expr) in exprs.iter().enumerate() {
        d = d.max(depth(expr) + i as i32); // binding expr depth
      }
      d
    },
    Expr::Print(expr) => depth(expr) + 2,
  }
}

fn parse_expr(s: &Sexp) -> Expr {
  match s {
    // number
    Sexp::Atom(I(n)) => {
        let i = i64::try_from(*n);
        match i {
            Err(e) => panic!("Invalid operand {s}, error {e}"),
            Ok(f) => Expr::Number(f),
        }
    }
    // boolean
    Sexp::Atom(S(s)) if s == "true" => Expr::Boolean(true),
    Sexp::Atom(S(s)) if s == "false" => Expr::Boolean(false),
    Sexp::Atom(S(s)) if s == "input" => Expr::Input(),
    // identifier
    Sexp::Atom(S(s)) => Expr::Id(s.clone()),
    // let
    Sexp::List(vec) => {
      match &vec[..] {
        // let
        [Sexp::Atom(S(op)), Sexp::List(vec), e] if op == "let" => {
          let mut binds = vec![];
          for bind in vec {
            binds.push(parse_bind(&bind));
          }
          Expr::Let(binds, Box::new(parse_expr(&e)))
        },
        // op1
        [Sexp::Atom(S(op)), e] if op == "add1" => Expr::UnOp(Op1::Add1, Box::new(parse_expr(&e))),
        [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::UnOp(Op1::Sub1, Box::new(parse_expr(&e))),
        [Sexp::Atom(S(op)), e] if op == "isnum" => Expr::UnOp(Op1::IsNum, Box::new(parse_expr(&e))),
        [Sexp::Atom(S(op)), e] if op == "isbool" => Expr::UnOp(Op1::IsBool, Box::new(parse_expr(&e))),
        // op2
        [Sexp::Atom(S(op)), e1, e2] if op == "+" => Expr::BinOp(Op2::Plus, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        [Sexp::Atom(S(op)), e1, e2] if op == "-" => Expr::BinOp(Op2::Minus, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        [Sexp::Atom(S(op)), e1, e2] if op == "*" => Expr::BinOp(Op2::Times, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        [Sexp::Atom(S(op)), e1, e2] if op == "<" => Expr::BinOp(Op2::Less, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        [Sexp::Atom(S(op)), e1, e2] if op == ">" => Expr::BinOp(Op2::Greater, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        [Sexp::Atom(S(op)), e1, e2] if op == ">=" => Expr::BinOp(Op2::GreaterEqual, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        [Sexp::Atom(S(op)), e1, e2] if op == "<=" => Expr::BinOp(Op2::LessEqual, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        [Sexp::Atom(S(op)), e1, e2] if op == "=" => Expr::BinOp(Op2::Equal, Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2))),
        // print
        [Sexp::Atom(S(op)), e] if op == "print" => Expr::Print(Box::new(parse_expr(&e))),
        // if
        [Sexp::Atom(S(op)), e1, e2, e3] if op == "if" => Expr::If(Box::new(parse_expr(&e1)), Box::new(parse_expr(&e2)), Box::new(parse_expr(&e3))),
        // loop / break
        [Sexp::Atom(S(op)), e] if op == "loop" => Expr::Loop(Box::new(parse_expr(&e))),
        [Sexp::Atom(S(op)), e] if op == "break" => Expr::Break(Box::new(parse_expr(&e))),
        // set
        [Sexp::Atom(S(op)), Sexp::Atom(S(s)), e] if op == "set!" => Expr::Set(s.clone(), Box::new(parse_expr(&e))),
        // block
        [Sexp::Atom(S(op)), ..] if op == "block" => {
          let subexpr = &vec[1..];
          if subexpr.len() == 0 {
            panic!("parse error: Invalid Block with 0 subexpr");
          }
          else {
            let mut binds = vec![];
            for bind in subexpr {
              binds.push(parse_expr(&bind));
            }
            Expr::Block(binds)  
          }
        },
        [Sexp::Atom(S(fname)), ..] if ! RESERVED_WORDS.contains(&fname.as_str()) => {
          let subexpr = &vec[1..];
          let mut params = vec![];
          for param in subexpr {
            params.push(parse_expr(&param));
          }
          Expr::Call(fname.clone(), params)
        },
        _ => panic!("parse error: Invalid op {:?}", vec),
      }
    }
    _ => panic!("parse error: Invalid Sexp \"{:?}\"", s),
  }
}

fn compile_program(p: &Program) -> String {
  let mut instr: Vec<Instr> = vec![];
  let mut label_id: i64 = 0;
  let mut loop_stack: Vec<String> = vec![];
  let mut func_dic: im::HashMap<String, i32> = im::HashMap::new();
  // register the function definitions
  for def in &p.defs {
    register_definition(def, &mut func_dic);
  }
  // compile the function definitions
  for def in &p.defs {
    instr.extend(compile_definition(def, &mut label_id, &mut func_dic));
  }
  // compile the main function
  let main_depth = (depth(&p.main).max(0) / 2) * 2 + 1;
  instr.push(Instr::Label("our_code_starts_here".to_string()));
  instr.push(Instr::ISub(Val::Reg(Reg::RSP), Val::Const((main_depth * 8) as i64)));
  instr.extend(compile_to_instrs(&p.main, 0, &HashMap::new(), &mut label_id, &mut loop_stack, &mut func_dic, true, false, 0, 0));
  instr.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::Const((main_depth * 8) as i64)));
  instr.push(Instr::Return());

  let mut program = String::new();
  for i in instr {
      program.push_str(&instr_to_str(&i));
  }
  return program;
}

fn register_definition(d: &Definition, func_dic :&mut im::HashMap<String, i32>) -> () {
  match d {
    Definition::Func(_, args, _) => {
      if args.len() == 0 {
          panic!("parse error: Invalid function definition without function name");
      }
      let name = args[0].clone();
      if func_dic.contains_key(&name) {
        panic!("parse error: Duplicate function definition for function name {}", name);
      }
      func_dic.insert(name.clone(), (args.len() - 1) as i32);
    }
  }
}

fn compile_definition(d: &Definition, l :&mut i64, func_dic :&mut im::HashMap<String, i32>) -> Vec<Instr> {
    match d {
        Definition::Func(_, args, body) => {
          let mut env = HashMap::new();
          if args.len() == 0 {
              panic!("parse error: Invalid function definition without function name");
          }
          let name = args[0].clone();
          let fn_depth = (depth(body).max(0) / 2) * 2 + 1;
          // iterate through the args but skip the first element (which is name)
          let mut arg_names = vec![];
          for (i, arg) in args.iter().enumerate().skip(1) {
            env.insert(arg.clone(), i as i64 + fn_depth as i64);
            if arg_names.contains(&arg.clone()) {
              panic!("parse error: Duplicate argument name {}", arg);
            }
            arg_names.push(arg.clone());
          }
          let mut instrs = vec![];
          instrs.push(Instr::Label(name.clone()));
          instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::Const((fn_depth * 8) as i64)));
          instrs.append(&mut compile_to_instrs(body, 0, &env, l, &mut vec![], func_dic, false, true, (args.len() - 1) as i32, fn_depth * 8));
          instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::Const((fn_depth * 8) as i64)));
          instrs.push(Instr::Return());
          return instrs;
        }
    }
}

fn instr_to_str(i: &Instr) -> String {
  match i {
      Instr::IMov(dst, src) => format!("  mov {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::IAdd(dst, src) => format!("  add {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::ISub(dst, src) => format!("  sub {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::IMul(dst, src) => format!("  imul {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::Cmp(dst, src)  => format!("  cmp {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::And(dst, src)  => format!("  and {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::CMOV(dst, src)  => format!("  cmove {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::Jmp(s) => format!("  jmp {s}\n"),
      Instr::Jne(s) => format!("  jne {s}\n"),
      Instr::Je(s)  => format!("  je {s}\n"),
      Instr::Jge(s) => format!("  jge {s}\n"),
      Instr::Jle(s) => format!("  jle {s}\n"),
      Instr::Label(s) => format!("{s}:\n"),
      Instr::Sar(dst, cnt) => format!("  sar {}, {}\n", val_to_str(dst), val_to_str(cnt)),
      Instr::Jo(s) => format!("  jo {s}\n"),
      Instr::Call(s) => format!("  call {s}\n"),
      Instr::Return() => format!("  ret\n"),
      Instr::ICMovo(dst, src) => format!("  cmovo {}, {}\n", val_to_str(dst), val_to_str(src)),
      Instr::ICMovne(dst, src) => format!("  cmovne {}, {}\n", val_to_str(dst), val_to_str(src)),
  }
}

fn val_to_str(v: &Val) -> String {
  match v {
      Val::Reg(r) => match r {
          Reg::RAX => "rax".to_string(),
          Reg::RBX => "rbx".to_string(),
          Reg::RCX => "rcx".to_string(),
          Reg::RSP => "rsp".to_string(),
          Reg::RDI => "rdi".to_string(),
      },
      Val::Imm(n) => {
        let max_bound = 4611686018427387903 as i64;
        let min_bound = -4611686018427387904 as i64;
        if n > &max_bound || n < &min_bound {
          panic!("Invalid immutable, integer overflow");
        }
        (n << 1).to_string()
      },
      Val::Const(n) => n.to_string(),
      Val::RegOffset(r, n) => {
        if n >= &0 {
          format!("[{}+{}]", val_to_str(&Val::Reg(r.clone())), (n * 8).to_string())
        }
        else {
          format!("[{}-{}]", val_to_str(&Val::Reg(r.clone())), (-n * 8).to_string())
        }
      }
  }
}

fn check_not_bool(val :Val) -> Vec<Instr> {
  vec![
    Instr::IMov(Val::Reg(Reg::RBX), val),
    Instr::And(Val::Reg(Reg::RBX), Val::Const(1)),
    Instr::Cmp(Val::Reg(Reg::RBX), Val::Const(0)),
    Instr::IMov(Val::Reg(Reg::RBX), Val::Const(ERRCODE_INVALID_ARG)),
    Instr::ICMovne(Val::Reg(Reg::RDI), Val::Reg(Reg::RBX)),
    Instr::Jne("throw_error".to_string()),
  ]
}

fn check_not_overflow() -> Vec<Instr> {
  vec![
    Instr::IMov(Val::Reg(Reg::RBX), Val::Const(ERRCODE_OVERFLOW)),
    Instr::ICMovo(Val::Reg(Reg::RDI), Val::Reg(Reg::RBX)),
    Instr::Jo("throw_error".to_string()),
  ]
}

fn compile_to_instrs(e: &Expr, si: i64, env: &HashMap<String, i64>, l :&mut i64, loop_stack :&mut Vec<String>, func_dic :&mut im::HashMap<String, i32>, is_main :bool, is_tail :bool, tail_param_num: i32, frame_size: i32) -> Vec<Instr> {
  match e {
      Expr::Number(n) => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Imm(*n))],
      Expr::Boolean(b) => {
        match b {
          true => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Const(TRUE_CONST))],
          false => vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST))],
        }
      }
      Expr::Input() => {
        if is_main {
          vec![Instr::IMov(Val::Reg(Reg::RAX), Val::Reg(Reg::RDI))]
        }
        else {
          panic!("parse error: Not expected to use input in non-main function")
        }
      },
      Expr::Id(s) => {
          let offset = env.get(s);
          if offset.is_none() {
              panic!("Unbound variable identifier {s}");
          }
          vec![Instr::IMov(
              Val::Reg(Reg::RAX),
              Val::RegOffset(Reg::RSP, *offset.unwrap()),
          )]
      }
      Expr::Let(bindings, body) => {
          let mut instrs = vec![];
          let mut env_new = env.clone();
          let mut curr_names = HashSet::<String>::new();
          if bindings.len() == 0 {
              panic!("parse error: Invalid let without bindings");
          }
          for (i, (name, expr)) in bindings.iter().enumerate() {
              if curr_names.contains(&name.clone()) {
                  panic!("parse error: Duplicate binding {name} Invalid");
              }
              instrs.extend(compile_to_instrs(expr, i as i64 + si, &env_new, l, loop_stack, func_dic, is_main, false ,0, frame_size));
              instrs.push(Instr::IMov(
                  Val::RegOffset(Reg::RSP, i as i64 + si),
                  Val::Reg(Reg::RAX),
              ));
              curr_names.insert(name.clone());
              env_new = env_new.update(name.clone(), i as i64 + si);
          }
          instrs.extend(compile_to_instrs(body, si + bindings.len() as i64, &env_new, l, loop_stack, func_dic, is_main, is_tail, tail_param_num, frame_size));
          instrs
      }
      Expr::Print(expr) => {
          let mut instrs = compile_to_instrs(expr, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size);
          instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si), Val::Reg(Reg::RAX)));
          instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, si + 1), Val::Reg(Reg::RDI)));
          instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::Reg(Reg::RAX)));
          instrs.push(Instr::Call("snek_print".to_string()));
          instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
          instrs.push(Instr::IMov(Val::Reg(Reg::RDI), Val::RegOffset(Reg::RSP, si + 1)));
          instrs
      }
      Expr::UnOp(op, expr) => {
          let mut instrs = compile_to_instrs(expr, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size);
          match op {
              Op1::Add1 => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::Const(2)));
                instrs.extend(check_not_overflow());
              },
              Op1::Sub1 => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.push(Instr::ISub(Val::Reg(Reg::RAX), Val::Const(2)));
                instrs.extend(check_not_overflow());
              },
              Op1::IsNum => {
                instrs.push(Instr::And(Val::Reg(Reg::RAX), Val::Const(1)));
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::Const(0)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Const(TRUE_CONST)));
                instrs.push(Instr::CMOV(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
              },
              Op1::IsBool => {
                instrs.push(Instr::And(Val::Reg(Reg::RAX), Val::Const(1)));
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::Const(0)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(TRUE_CONST)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Const(FALSE_CONST)));
                instrs.push(Instr::CMOV(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
              },
          }
          instrs
      }
      Expr::BinOp(op, lhs, rhs) => {
          let mut instrs = compile_to_instrs(rhs, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size);
          instrs.push(Instr::IMov(
              Val::RegOffset(Reg::RSP, si),
              Val::Reg(Reg::RAX),
          ));
          instrs.extend(compile_to_instrs(lhs, si + 1, env, l, loop_stack, func_dic, is_main, false ,0, frame_size));
          match op {
              Op2::Plus => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.extend(check_not_bool(Val::RegOffset(Reg::RSP, si)));

                instrs.push(Instr::IAdd(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.extend(check_not_overflow());
              }
              Op2::Minus => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.extend(check_not_bool(Val::RegOffset(Reg::RSP, si)));
                
                instrs.push(Instr::ISub(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.extend(check_not_overflow());
              }
              Op2::Times => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.extend(check_not_bool(Val::RegOffset(Reg::RSP, si)));
                instrs.push(Instr::Sar(Val::Reg(Reg::RAX), Val::Const(1)));
                instrs.push(Instr::IMul(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.extend(check_not_overflow());
              },
              Op2::Equal => {
                // check if both have the same type
                instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Reg(Reg::RAX)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RCX), Val::RegOffset(Reg::RSP, si)));

                instrs.push(Instr::And(Val::Reg(Reg::RBX), Val::Const(1)));
                instrs.push(Instr::And(Val::Reg(Reg::RCX), Val::Const(1)));
                instrs.push(Instr::Cmp(Val::Reg(Reg::RBX), Val::Reg(Reg::RCX)));

                instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Const(ERRCODE_INVALID_ARG)));
                instrs.push(Instr::ICMovne(Val::Reg(Reg::RDI), Val::Reg(Reg::RBX)));
                instrs.push(Instr::Jne("throw_error".to_string()));
                
                // compare the equality
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RBX), Val::Const(TRUE_CONST)));
                instrs.push(Instr::CMOV(Val::Reg(Reg::RAX), Val::Reg(Reg::RBX)));
              },
              Op2::Greater => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.extend(check_not_bool(Val::RegOffset(Reg::RSP, si)));
                let cmp_end_label = new_label(l, "cmp_end_label");
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST)));
                instrs.push(Instr::Jle(cmp_end_label.clone()));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(TRUE_CONST)));
                instrs.push(Instr::Label(cmp_end_label.clone()));
              },
              Op2::GreaterEqual => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.extend(check_not_bool(Val::RegOffset(Reg::RSP, si)));
                let cmp_end_label = new_label(l, "cmp_end_label");
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(TRUE_CONST)));
                instrs.push(Instr::Jge(cmp_end_label.clone()));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST)));
                instrs.push(Instr::Label(cmp_end_label.clone()));
              },
              Op2::Less => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.extend(check_not_bool(Val::RegOffset(Reg::RSP, si)));
                let cmp_end_label = new_label(l, "cmp_end_label");
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST)));
                instrs.push(Instr::Jge(cmp_end_label.clone()));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(TRUE_CONST)));
                instrs.push(Instr::Label(cmp_end_label.clone()));
              },
              Op2::LessEqual => {
                instrs.extend(check_not_bool(Val::Reg(Reg::RAX)));
                instrs.extend(check_not_bool(Val::RegOffset(Reg::RSP, si)));
                let cmp_end_label = new_label(l, "cmp_end_label");
                instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::RegOffset(Reg::RSP, si)));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(TRUE_CONST)));
                instrs.push(Instr::Jle(cmp_end_label.clone()));
                instrs.push(Instr::IMov(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST)));
                instrs.push(Instr::Label(cmp_end_label.clone()));
              },
          }
          instrs
      },
      Expr::If(cond, thn, els) => {
        let end_label = new_label(l, "ifend");
        let els_label = new_label(l, "ifelse");
        let cond_instrs: Vec<Instr> = compile_to_instrs(cond, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size);
        let thn_instrs: Vec<Instr> = compile_to_instrs(thn, si, env, l, loop_stack, func_dic, is_main, is_tail, tail_param_num, frame_size);
        let els_instrs: Vec<Instr> = compile_to_instrs(els, si, env, l, loop_stack, func_dic, is_main, is_tail, tail_param_num, frame_size);
        
        let mut instrs: Vec<Instr> = vec![];
        instrs.extend(cond_instrs);
        instrs.push(Instr::Cmp(Val::Reg(Reg::RAX), Val::Const(FALSE_CONST)));
        instrs.push(Instr::Je(els_label.clone()));
        instrs.extend(thn_instrs);
        instrs.push(Instr::Jmp(end_label.clone()));
        instrs.push(Instr::Label(els_label.clone()));
        instrs.extend(els_instrs);
        instrs.push(Instr::Label(end_label.clone()));
        instrs
      },
      Expr::Loop(expr) => {
        let mut instrs: Vec<Instr> = vec![];
        let start_label = new_label(l, "loop_start");
        let end_label = new_label(l, "loop_end");
        loop_stack.push(end_label.clone());
        instrs.push(Instr::Label(start_label.clone()));
        println!("{:?}", expr);
        instrs.extend(compile_to_instrs(expr, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size));
        loop_stack.pop();
        if loop_stack.contains(&end_label) {
          panic!("Loop without break");
        }
        instrs.push(Instr::Jmp(start_label.clone()));
        instrs.push(Instr::Label(end_label));
        instrs
      },
      Expr::Break(expr) => {
        if loop_stack.len() == 0 {
          panic!("Unexpected break outside loop");
        }
        let break_label = loop_stack[loop_stack.len() - 1].clone();
        // TODO
        let mut instrs = compile_to_instrs(expr, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size);
        instrs.push(Instr::Jmp(break_label));
        instrs
      },
      Expr::Set(s, expr) => {
        let mut instrs = compile_to_instrs(expr, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size);
        let offset = env.get(s);
        if offset.is_none() {
            panic!("Unbound variable identifier {s}");
        }
        instrs.push(Instr::IMov(Val::RegOffset(Reg::RSP, *offset.unwrap()), Val::Reg(Reg::RAX)));
        instrs
      },
      Expr::Block(exprs) => {
        let mut instrs = vec![];
          if exprs.len() == 0 {
              panic!("parse error: Invalid: No instructions in block which is invalid");
          }
          for (i,  expr) in exprs.iter().enumerate() {
            if i == exprs.len() - 1 {
              instrs.extend(compile_to_instrs(expr, si, env, l, loop_stack, func_dic, is_main, is_tail ,tail_param_num, frame_size));
            }
            else {
              instrs.extend(compile_to_instrs(expr, si, env, l, loop_stack, func_dic, is_main, false ,0, frame_size));
            }
          }
          instrs
      },
      Expr::Call(fname, params) => {
        let mut instrs = vec![];
        if !(func_dic.contains_key(fname)) {
          panic!("parse error: Invalid Function {} is not defined", fname);
        }
        else if func_dic[fname] != params.len() as i32 {
          panic!("parse error: Function {} expects {} arguments, but actually receive {} arguments", fname, func_dic[fname], params.len());
        }
        let param_offset = (params.len() as i64 + 1) / 2 * 2;
        for (i,  expr) in params.iter().enumerate() {
          instrs.extend(compile_to_instrs(expr, si + i as i64, env, l, loop_stack, func_dic, is_main, false, 0, frame_size));
          instrs.push(Instr::IMov(
              Val::RegOffset(Reg::RSP, si + i as i64),
              Val::Reg(Reg::RAX),));
        }
        let enable_tail_call = true;
        if enable_tail_call && is_tail && params.len() as i32 <= tail_param_num  {
          // proper tail call
          // first put all the arguments into the right place
          for (i,  _) in params.iter().enumerate() {
            instrs.push(Instr::IMov(
              Val::Reg(Reg::RAX),
              Val::RegOffset(Reg::RSP, si + i as i64),));
            instrs.push(Instr::IMov(
                Val::RegOffset(Reg::RSP, param_offset * -1 + i as i64),
                Val::Reg(Reg::RAX),));
          }
          // Then put them back into the previous argument palce
          let frame_offset = (frame_size / 8) as i64;
          instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::Const(frame_size as i64)));
          for (i,  _) in params.iter().enumerate() {
            instrs.push(Instr::IMov(
              Val::Reg(Reg::RAX),
              Val::RegOffset(Reg::RSP, -frame_offset + param_offset * -1 + i as i64)));
            instrs.push(Instr::IMov(
                Val::RegOffset(Reg::RSP, (i + 1) as i64),
                Val::Reg(Reg::RAX),));
          }
          instrs.push(Instr::Jmp(fname.clone()));
        }
        else {
          // normal call
          for (i,  _) in params.iter().enumerate() {
            instrs.push(Instr::IMov(
              Val::Reg(Reg::RAX),
              Val::RegOffset(Reg::RSP, si + i as i64),));
            instrs.push(Instr::IMov(
                Val::RegOffset(Reg::RSP, param_offset * -1 + i as i64),
                Val::Reg(Reg::RAX),));
          }
          instrs.push(Instr::ISub(Val::Reg(Reg::RSP), Val::Const(param_offset * 8)));
          instrs.push(Instr::Call(fname.clone()));
          instrs.push(Instr::IAdd(Val::Reg(Reg::RSP), Val::Const(param_offset * 8)));
        }
        instrs
      },
  }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let in_name = &args[1];
    let out_name = &args[2];
    
    // read in file
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;
    
    // parse: string -> sexp
    let prog  = "(".to_owned() + &in_contents + ")";
    let parsed_sexp: Result<Sexp, Box<Error>> = parse(&prog);
    match &parsed_sexp {
      Err(e) => panic!("Invalid sexp {e}"),
      Ok(p) => println!("parsed_sexp: {:?}", p),
    }

    // parse: sexp -> program
    let prog = parse_program(&parsed_sexp.unwrap());
    println!("Program: {:?}", prog);

    // compile: program -> asm
    let result = compile_program(&prog);

    let asm_program: String = format!(
        "
section .text
extern snek_error
extern snek_print
global our_code_starts_here
throw_error:
  call snek_error
{}
",
        result
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}