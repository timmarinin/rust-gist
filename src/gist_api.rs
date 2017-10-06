extern crate serde_json;
extern crate reqwest;

use std;
use std::fs::File;
use token::Token;
use std::io::{Read};

#[derive(Debug)]
pub struct Request {
  token: Token,
  public: bool,
  filename: String,
  text: String
}

#[derive(Deserialize, Debug)]
pub struct Response {
  pub html_url: String,
  pub url: String
}

pub enum Error {
  BadInput(()),
  NoFile(std::io::Error),
  BadRequest(reqwest::Error),
  BadResponse(reqwest::StatusCode)
}

impl Request {
  pub fn new(token: Result<Token, std::io::Error>) -> Request {
    let mut tok = Token::new();

    if let Ok(t) = token {
      tok = t
    }

    Request {
      token: tok,
      public: false,
      text: String::new(),
      filename: String::new()
    }
  }

  pub fn public(&mut self, is_public: bool) -> &mut Request {
    self.public = is_public;
    self
  }

  fn create_request(&mut self) -> Result<(reqwest::Client, reqwest::Request), Error> {
    let j = json!({
      "public": self.public,
      "files": {
        self.filename.clone(): {
          "content": self.text
        }
      }
    });
    let c = reqwest::Client::new();
    let mut req = c.post("https://api.github.com/gists");
    req
      .header(reqwest::header::ContentType::json())
      .body(j.to_string());

    if self.token != "" {
      req.header(reqwest::header::Authorization(self.token.to_owned()));
    }
    
    match req.build() {
      Ok(built) => Ok((c, built)),
      Err(e) => Err(Error::BadRequest(e))
    }
  }

  pub fn execute(&mut self) -> Result<Response, Error> {
    if self.text.len() == 0 {
      return Err(Error::BadInput(()))
    }
    let (c, req) = self.create_request()?;
  
    let mut resp = c.execute(req).unwrap();

    match resp.status() {
      reqwest::StatusCode::Created => {
        let r: Response = resp.json().unwrap();
        Ok(r)
      },
      reqwest::StatusCode::UnprocessableEntity => {
        let mut s = String::new();
        resp.read_to_string(&mut s).unwrap();
        println!("Response is NOT ok, body: {:?}", s);
        Err(Error::BadResponse(resp.status()))
      },
      _ => {
        println!("Something went wrong.\n{:?}", resp);
        Err(Error::BadResponse(resp.status()))
      }
    }
  }

  pub fn from_stdin(&mut self) -> Result<&mut Request, Error> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    match handle.read_to_string(&mut buffer) {
      Ok(_) => {
        self.filename = String::from("stdin");
        self.text = buffer;
        Ok(self)
      },
      Err(e) => Err(Error::NoFile(e))
    }
  }

  pub fn from_file(&mut self, filename: String) -> Result<&mut Request, Error> {
    let file = File::open(filename.clone());

    match file {
      Ok(mut f) => {
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        self.text = s;
        self.filename = filename;
        Ok(self)
      },
      Err(e) => {
        println!("{}", e);
        Err(Error::NoFile(e))
      }
    }
  }
}
