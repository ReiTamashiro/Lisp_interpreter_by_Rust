mod lisp_structure;
use lisp_structure::*;
pub mod LEval;
use LEval::*;

fn main() {
    LEval::hello();
}