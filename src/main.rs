use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;
use sysy_compiler::irgen;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

// 引用 lalrpop 生成的解析器
lalrpop_mod!(sysy);

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;

  // 调用 lalrpop 生成的 parser 解析输入文件
  let comp_unit = sysy::CompUnitParser::new().parse(&input).unwrap();

  let koopa_program = irgen::generate_koopa_program(&comp_unit);

  let mut koopa_generator = KoopaGenerator::new(Vec::new());
  koopa_generator.generate_on(&koopa_program).unwrap();
  let koopa_ir_text = std::str::from_utf8(&koopa_generator.writer()).unwrap().to_string();
  println!("{}", koopa_ir_text);

  Ok(())
}
