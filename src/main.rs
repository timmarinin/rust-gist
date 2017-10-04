#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

mod token;

const USAGE: &'static str = "
rust-gist -- yet another gist.github.com creating client

Usage:
  rust-gist [--public] [--anonymous] <file>
  rust-gist token [<token>]
  rust-gist --version
  rust-gist (-h|--help)

Options:
  --public        Make gist public instead of private
  --anonymous     Post as anonymous
  -h, --help      Show this text
  --version       Display version
";

#[derive(Debug, Deserialize)]
struct Args {
  flag_public: bool,
  flag_anonymous: bool,
  arg_file: String,
  arg_token: String,
  cmd_token: bool
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d|
      d
        .help(true)
        .version(Some("rust-gist, version ".to_string() + env!("CARGO_PKG_VERSION")))
        .deserialize())
    .unwrap_or_else(|e| e.exit());

  println!("{:?}", args);

  if args.cmd_token {
    token::save_token(args.arg_token).unwrap();
  }
}


