#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate docopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate reqwest;

use std::io::{self,Write};

mod errors {
    use std::{fmt, io};
    use reqwest::StatusCode;

    error_chain!{
        foreign_links {
            Fmt(fmt::Error);
            Io(io::Error) #[cfg(unix)];
        }
        errors {
            SaveToken
            LoadToken
            BadRequest
            BadResponse(code: StatusCode) {
                description("Got wrong answer from Github API")
                display("Got wrong response code from Github API: {}", code)
            }
            UnprocessableEntityResponse(body: String) {
                description("Github refuses to accept this body")
                display("Github won't accept this Gist. See the answer below:\n{}", body)
            }

            Network {
                description("Something wrong with a network.")
                display("Unable to connect to Github API")
            }

            BadInput {
                description("No suitable content for Gist")
                display("Incorrect or insufficient input")
            }
        }
    }
}

mod cli;
mod token;
mod gist_api;

fn main() {
    if let Err(ref e) = cli::run() {
        let stderr = &mut io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        std::process::exit(1);

    }
}
