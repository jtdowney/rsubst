use minijinja::{Environment, UndefinedBehavior};
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
    process,
};

const HELP: &str = "\
rsubst - envsubst-like utility with Jinja2 templating

USAGE:
    rsubst [OPTIONS] [TEMPLATE_FILE]

ARGS:
    <TEMPLATE_FILE>  Path to template file (reads stdin if omitted)

OPTIONS:
    --strict   Treat missing variables as errors
    -h, --help Show this help message
";

fn report_template_error(err: minijinja::Error) -> ! {
    eprint!("error: {err}");
    let debug_info = err.display_debug_info();
    let debug_info = debug_info.to_string();
    if !debug_info.is_empty() {
        eprint!("\n\n{debug_info}");
    }
    eprintln!();
    process::exit(1);
}

fn main() {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        print!("{HELP}");
        process::exit(0);
    }

    let strict = args.contains("--strict");
    let template_path = args
        .opt_free_from_str::<String>()
        .expect("Failed to parse arguments");

    let template = if let Some(ref path) = template_path {
        fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("error: {err} (path: {path})");
            process::exit(1);
        })
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read template from stdin");
        input
    };

    let mut env = Environment::new();
    env.set_debug(true);
    if strict {
        env.set_undefined_behavior(UndefinedBehavior::Strict);
    }

    if let Err(err) = env.add_template("template", &template) {
        report_template_error(err);
    }

    let tmpl = env
        .get_template("template")
        .expect("Failed to get template");
    let ctx = env::vars().collect::<HashMap<_, _>>();

    let output = match tmpl.render(&ctx) {
        Ok(output) => output,
        Err(err) => report_template_error(err),
    };
    println!("{output}");
}
