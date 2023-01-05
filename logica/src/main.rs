mod core;
mod ir;
lalrpop_mod!(pub lang);

use clap::Parser;
use ir::*;
use lalrpop_util::lalrpop_mod;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("./circuit.rs"))]
    output: String,
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value_t = String::from("True"))]
    context: String,
}

struct Compiler {
    context: Vec<BooleanExpr>,
    ouput: BufWriter<File>,
}
impl Compiler {
    fn compile(&mut self, circuit: BooleanExpr) {
        use BooleanExpr::*;
        match circuit {
            True => self.ouput.write_all(b"True").unwrap(),
            False => self.ouput.write_all(b"False").unwrap(),
            Variable(v) => self.compile(self.context[v].clone()),
            Not(v) => {
                self.ouput.write_all(b"Not::compute").unwrap();
                self.ouput.write_all(b"(").unwrap();
                self.compile(*v);
                self.ouput.write_all(b")").unwrap();
            }
            And(lhe, rhe) => {
                self.ouput.write_all(b"And::compute").unwrap();
                self.ouput.write_all(b"(").unwrap();
                self.compile(*lhe);
                self.ouput.write_all(b",").unwrap();
                self.compile(*rhe);
                self.ouput.write_all(b")").unwrap();
            }
            Or(lhe, rhe) => {
                self.ouput.write_all(b"Or::compute").unwrap();
                self.ouput.write_all(b"(").unwrap();
                self.compile(*lhe);
                self.ouput.write_all(b",").unwrap();
                self.compile(*rhe);
                self.ouput.write_all(b")").unwrap();
            }
        }
    }
}
fn main() {
    let args = Args::parse();
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);
    let circuit = std::fs::read_to_string(&input).unwrap();
    let expr_parser = lang::BooleanExprParser::new();
    let context_parser = lang::ContextParser::new();
    let circuit = expr_parser.parse(&circuit).unwrap();
    let mut compiler = Compiler {
        context: context_parser.parse(&args.context).unwrap(),
        ouput: BufWriter::new(File::create(output).unwrap()),
    };
    compiler.ouput.write_all(core::header().as_bytes()).unwrap();
    compiler.ouput.write_all(b"\n").unwrap();
    compiler.ouput.write_all(core::circuit_start().as_bytes()).unwrap();
    compiler.ouput.write_all(b"\n").unwrap();
    compiler.compile(circuit);
    compiler.ouput.flush().unwrap();
    compiler.ouput.write_all(b"\n").unwrap();
    compiler.ouput.write_all(core::circuit_end().as_bytes()).unwrap();
}
