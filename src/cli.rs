use docopt::Docopt;

use super::{token,gist_api};
use errors::Result;

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
    cmd_token: bool,
}

fn cmd_token(tok: String) -> Result<()> {
    match tok.as_ref() {
      "" => token::drop_token(),
      _ => token::write_token(tok)
    }
}

pub fn run() -> Result<()> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| {
            d.help(true)
                .version(Some(
                    "rust-gist, version ".to_string() + env!("CARGO_PKG_VERSION"),
                ))
                .deserialize()
        })
        .unwrap_or_else(|e| e.exit());

    if args.cmd_token {
        cmd_token(args.arg_token)
    } else {
        let tok = if args.flag_anonymous {
            token::get_anonymous_token()
        } else {
            token::read_token()
        };

        let mut req = gist_api::Request::new(tok);
        req.public(args.flag_public);

        if args.arg_file != "" {
            req.from_file(args.arg_file)?;
        } else {
            req.from_stdin()?;
        }

        let result = req.execute()?;
        println!("{}", if args.flag_raw { result.url } else { result.html_url });
        Ok(())
    }
}
