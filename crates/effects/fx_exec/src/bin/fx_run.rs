//! fx_run: the command line of the effect runner. The implementation is `fx_exec::fxrun` (see its header for the options);
//! this wrapper adds `--device cpu|gpu|auto` and runs it on a thread with a large stack.
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    std::process::exit(fx_exec::fxrun::main_with_args(&a));
}
