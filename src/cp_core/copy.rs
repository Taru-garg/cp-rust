use crate::Args;

pub fn copy(args: Args) {
    for (copy_source, copy_target) in args.source.iter().zip(args.target.iter()) {
        let _ = std::fs::create_dir_all(copy_target.parent().unwrap());
        let _ = std::fs::copy(copy_source, copy_target);
    }
}
