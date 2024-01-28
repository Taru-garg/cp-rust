mod cp_core;
use cp_core::copy;
use cp_core::Args;

fn main() {
    let copy_args = Args::parser().expect("Unable to parse the passed argss");
    copy(copy_args);
}
