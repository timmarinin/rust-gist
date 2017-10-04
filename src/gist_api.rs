extern crate serde_json;
extern crate reqwest;

use std;
use std::fs::File;
use std::collections::HashMap;
use token::Token;
use std::io::prelude::*;

#[derive(Debug, Serialize)]
struct RequestBody {
   public: bool,
   files: HashMap<String, HashMap<String, String>>
}

#[derive(Debug)]
pub struct Request {
  token: Token,
  public: bool,
  filename: String,
  body: RequestBody
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
      body: RequestBody { public: false, files: HashMap::new() },
      public: false,
      filename: "".to_string()
    }
  }

  pub fn public(&mut self, is_public: bool) -> &mut Request {
    self.public = is_public;
    self
  }

  fn create_request(&mut self) -> Result<(reqwest::Client, reqwest::Request), Error> {
    let h = serde_json::to_string(&self.body);
    let c = reqwest::Client::new();
    
    match h {
      Err(_) =>  {
        return Err(Error::BadInput(()))
      },
      Ok(h) => {
        let mut req = c.post("https://api.github.com/gists");
        req
          .header(reqwest::header::ContentType::json())
          .body(h);

        if self.token != "" {
          req.header(reqwest::header::Authorization(self.token.to_owned()));
        }
        
        match req.build() {
          Ok(built) => Ok((c, built)),
          Err(e) => Err(Error::BadRequest(e))
        }
      }
    }
  }

  pub fn execute(&mut self) -> Result<Response, Error> {
    if self.body.files.len() == 0 {
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
    Ok(self)
  }

  pub fn from_file(&mut self, filename: String) -> Result<&mut Request, Error> {
    let mut new_body = RequestBody {
      public: self.public.clone(),
      files: HashMap::new()
    };

    let file = File::open(filename.clone());

    match file {
      Ok(mut f) => {
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let mut fmap = HashMap::new();
        fmap.insert("content".to_string(), s);
        new_body.files.insert(filename.clone(), fmap);
        self.body = new_body;
        Ok(self)
      },
      Err(e) => {
        println!("{}", e);
        Err(Error::NoFile(e))
      }
    }
  }
}
