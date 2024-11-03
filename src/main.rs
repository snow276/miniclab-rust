use koopa::back::KoopaGenerator;
use koopa::ir::Program;
use lalrpop_util::lalrpop_mod;
use sysy_compiler::codegen;
use sysy_compiler::irgen;
use std::env::args;
use std::fs::read_to_string;
use std::io::Write;
use std::process::exit;
use std::{fmt, io, fs::File};

// 引用 lalrpop 生成的解析器
lalrpop_mod!(sysy);

fn main() {
  if let Err(err) = try_main() {
    eprintln!("{}", err);
    exit(-1);
  }
}

fn try_main() -> Result<(), Error> {
  // 解析命令行参数
  let CommandLineArgs {
    mode,
    input,
    output,
  } = CommandLineArgs::parse()?;

  // 读取输入文件
  let input = read_to_string(input).map_err(Error::File)?;

  // 调用 lalrpop 生成的 parser 解析输入文件
  let comp_unit = sysy::CompUnitParser::new().parse(&input).unwrap();

  let koopa_program = irgen::generate_koopa_program(&comp_unit);

  match mode {
    Mode::Koopa => print_koopa_ir(&koopa_program, &output),
    Mode::Riscv => print_riscv_asm(&koopa_program, &output),
  }

  Ok(())
}

fn print_koopa_ir(program: &Program, output_path: &String) {
  let mut koopa_generator = KoopaGenerator::new(Vec::new());
  koopa_generator.generate_on(program).unwrap();
  let koopa_ir_text = std::str::from_utf8(&koopa_generator.writer()).unwrap().to_string();

  let mut file = File::create(output_path).unwrap();
  file.write(koopa_ir_text.as_bytes()).unwrap();  
}

fn print_riscv_asm(program: &Program, output_path: &String) {
  let riscv_asm_text = codegen::generate_riscv(program);
  let mut file = File::create(output_path).unwrap();
  file.write(riscv_asm_text.as_bytes()).unwrap();
}

enum Error {
  InvalidArgs,
  File(io::Error),
  Io(io::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::InvalidArgs => write!(
        f,
        r#"Usage: kira MODE INPUT -o OUTPUT

Options:
  MODE:   can be `-koopa`, `-riscv` or `-perf`
  INPUT:  the input SysY source file
  OUTPUT: the output file"#
      ),
      Self::File(err) => write!(f, "invalid input SysY file: {}", err),
      Self::Io(err) => write!(f, "I/O error: {}", err),
    }
  }
}

struct CommandLineArgs {
  mode: Mode,
  input: String,
  output: String,
}

impl CommandLineArgs {
  fn parse() -> Result<Self, Error> {
    let mut args = args();
    args.next();
    match (args.next(), args.next(), args.next(), args.next()) {
      (Some(m), Some(input), Some(o), Some(output)) if o == "-o" => {
        let mode = match m.as_str() {
          "-koopa" => Mode::Koopa,
          "-riscv" => Mode::Riscv,
          _ => return Err(Error::InvalidArgs),
        };
        Ok(Self {
          mode,
          input,
          output,
        })
      }
      _ => Err(Error::InvalidArgs),
    }
  }
}

enum Mode {
  Koopa,
  Riscv,
}