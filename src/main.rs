#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;

mod token;
mod gist_api;

const USAGE: &'static str = "
rust-gist -- yet another gist.github.com creating client

Usage:
  rust-gist token [<token>]
  rust-gist [--public] [--anonymous] [--raw] [<file>]
  rust-gist --version
  rust-gist (-h|--help)

Options:
  --public        Make gist public instead of private
  --anonymous     Post as anonymous
  --raw           Get raw URL, not html_url
  -h, --help      Show this text
  --version       Display version
";

#[derive(Debug, Deserialize)]
struct Args {
  flag_public: bool,
  flag_anonymous: bool,
  flag_raw: bool,
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

  if args.cmd_token {
    if args.arg_token != "" {
      token::write_token(args.arg_token).unwrap();
    } else {
      token::drop_token().unwrap();
    }
  } else {
    let tok = token::read_token();

    let mut req = gist_api::Request::new(tok);
    req.public(args.flag_public);

    if args.arg_file != "" {
      match req.from_file(args.arg_file) {
        Ok(_) => {},
        Err(gist_api::Error::NoFile(e)) => {
          eprintln!("{}", e);

          std::process::exit(1); 
        },
        Err(_) => {}
      }
    } else {
      match req.from_stdin() {
        Ok(_) => {},
        Err(gist_api::Error::NoFile(e)) => {
          eprintln!("{}", e);

          std::process::exit(1);
        },
        Err(_) => {}
      }
    }

    match req.execute() {
      Ok(r) => {
        println!("{}", if args.flag_raw { r.url } else { r.html_url })
      }
      Err(gist_api::Error::BadRequest(e)) => {
        eprintln!("Problem with request: {}", e);
        std::process::exit(2);
      },
      Err(gist_api::Error::BadResponse(e)) => {
        println!("Bad response from Github, code is {}", e)
      },
      Err(_) => {}
    }
  }
}


