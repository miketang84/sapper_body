#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate sapper;
extern crate url;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use url::form_urlencoded;

use sapper::mime;
use sapper::header::ContentType;
use sapper::{Request, Result, Key, Error};

pub type BodyMap = HashMap<String, Vec<String>>;

pub struct ReqBodyParams;
impl Key for ReqBodyParams { type Value = BodyMap; }

use serde_json::Value as JsonValue;
pub struct ReqJsonParams;
impl Key for ReqJsonParams { type Value = JsonValue; }

pub fn process(req: &mut Request) -> Result<()> {
    
    
    // should judge the content-type in the request headers
    let raw_body = req.raw_body().clone();
    match raw_body {
        Some(ref raw_body) => {
            
            let typenum = req.headers().get::<ContentType>().map(|header| {
                match **header {
                    mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json, _) => 1,
                    _ => 0
                }
            }).unwrap_or(0);
            
            // judge json type first, json type is 1
            if typenum == 1 {
                match serde_json::from_str(raw_body) {
                    Ok(val) => {
                        // println!("parsing json {:?}", val);
                        req.get_ext_mut().insert::<ReqJsonParams>(val);
                        
                        return Ok(());
                    }
                    Err(_) => {
                        // return Err(Error::BeforeError);
                        return Ok(());
                    }
                }
            
            }
            // else if content_type == ContentType::form_url_encoded() {
            else {
                // default branch
                let body_iter = form_urlencoded::parse(raw_body.as_bytes());
        
                let mut deduplicated: BodyMap = HashMap::new();
                for (key, val) in body_iter {
                    match deduplicated.entry(key.into_owned()) {
                        // Already a Vec here, push onto it
                        Occupied(entry) => { entry.into_mut().push(val.into_owned()); },

                        // No value, create a one-element Vec.
                        Vacant(entry) => { entry.insert(vec![val.into_owned()]); },
                    };
                }
                
                req.get_ext_mut().insert::<ReqBodyParams>(deduplicated);
            }
        },
        None => {
            // do nothing
        }
    }
    
    
    
    Ok(())
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
