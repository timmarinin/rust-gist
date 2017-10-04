#![feature(plugin)]
#![plugin(docopt_macros)]

#[macro_use]
extern crate serde_derive;
extern crate docopt;

//mod token;

docopt!(Args derive Debug, "
rust-gist -- yet another gist.github.com creating client

Usage:
  rust-gist [--public] [--anonymous] [<file>]
  rust-gist token [<token>]
  rust-gist --version
  rust-gist (-h|--help)

Options:
  --public    Make gist public instead of private
  --anonymous	Post as anonymous
  -h, --help  Show this text
  --version	  Display version
");

fn main() {
  let args: Args = Args::docopt()
    .help(true)
    .version(Some("rust-gist, version ".to_string() + env!("CARGO_PKG_VERSION")))
    .deserialize()
    .unwrap_or_else(|e| e.exit());

  println!("{:?}", args);

  if (args.cmd_token) {
    token::save_token(args.arg_token);
  }
}


