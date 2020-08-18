extern crate clap;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions(
        "rpm-builder",     // We specify the bin name manually
        clap::Shell::Bash, // Which shell to build completions for
        "./target",
    ); // Where write the completions to
}
