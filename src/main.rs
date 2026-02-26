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

    let template = if let Some(path) = template_path {
        fs::read_to_string(&path).expect("Failed to read template file")
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read template from stdin");
        input
    };

    let mut env = Environment::new();
    if strict {
        env.set_undefined_behavior(UndefinedBehavior::Strict);
    }

    env.add_template("template", &template)
        .expect("Failed to add template");

    let tmpl = env
        .get_template("template")
        .expect("Failed to get template");
    let ctx = env::vars().collect::<HashMap<_, _>>();

    let output = tmpl.render(&ctx).expect("Failed to render template");
    println!("{output}");
}
