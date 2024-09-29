mod bindings;

use clap::Parser;
use std::fmt;

use bindings::component::calculator::{calculate, calculate::Op};

fn parse_operator(op: &str) -> anyhow::Result<Op> {
    match op {
        "add" => Ok(Op::Add),
        "sub" => Ok(Op::Sub),
        _ => anyhow::bail!("Unknown operation: {}", op),
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
        }
    }
}

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))]
struct App {
    x: u32,
    y: u32,
    #[clap(value_parser = parse_operator)]
    op: Op,
}

impl App {
    fn run(self) {
        let res = calculate::calculate(self.op, self.x, self.y);
        println!("{} {} {} = {res}", self.x, self.op, self.y);
    }
}

fn main() {
    App::parse().run()
}
