extern crate reqwest;
extern crate serde_json;

use std;
use std::fs::File;
use token::Token;
use std::io::Read;
use errors::*;

#[derive(Debug)]
pub struct Request {
    token: Token,
    public: bool,
    filename: String,
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub html_url: String,
    pub url: String,
}

impl Request {
    pub fn new(token: Result<Token>) -> Request {
        let mut tok = Token::new();

        if let Ok(t) = token {
            tok = t
        }

        Request {
            token: tok,
            public: false,
            text: String::new(),
            filename: String::new(),
        }
    }

    pub fn public(&mut self, is_public: bool) -> &mut Request {
        self.public = is_public;
        self
    }

    fn create_request(&mut self) -> Result<(reqwest::Client, reqwest::Request)> {
        let j = json!(
        {
            "public": self.public,
            "files": {
                self.filename.clone(): {
                    "content": self.text
                }
            }
        });
        let c = reqwest::Client::new();
        let mut req = c.post("https://api.github.com/gists");
        req.header(reqwest::header::ContentType::json())
            .body(j.to_string());

        if self.token != "" {
            req.header(reqwest::header::Authorization(self.token.to_owned()));
        }

        let built = req.build().chain_err(|| ErrorKind::BadRequest)?;
        Ok((c, built))
    }

    pub fn execute(&mut self) -> Result<Response> {
        if self.text.len() == 0 {
           bail!(ErrorKind::BadInput)
        }
        let (c, req) = self.create_request().chain_err(|| ErrorKind::BadRequest)?;

        let mut resp = c.execute(req).chain_err(|| ErrorKind::Network)?;

        match resp.status() {
            reqwest::StatusCode::Created => {
                let r: Response = resp.json().unwrap();
                Ok(r)
            }
            reqwest::StatusCode::UnprocessableEntity => {
                let mut s = String::new();
                resp.read_to_string(&mut s)?;
                bail!(ErrorKind::UnprocessableEntityResponse(s))
            }
            _ => {
                bail!(ErrorKind::BadResponse(resp.status()))
            }
        }
    }

    pub fn from_stdin(&mut self) -> Result<&mut Request> {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer).chain_err(|| ErrorKind::BadInput)?;
        self.filename = String::from("stdin");
        self.text = buffer;
        Ok(self)
    }

    pub fn from_file(&mut self, filename: String) -> Result<&mut Request> {
        let mut f = File::open(filename.clone()).chain_err(|| ErrorKind::BadInput)?;
        let mut s = String::new();
        f.read_to_string(&mut s).chain_err(|| ErrorKind::BadInput)?;
        self.text = s;
        self.filename = String::from(filename.split('/').last().unwrap());
        Ok(self)
    }
}
