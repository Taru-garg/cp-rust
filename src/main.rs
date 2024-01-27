mod cp_core;
use cp_core::Args;

fn main() {
    let copy_args = Args::parser();
    dbg!(copy_args);
}
