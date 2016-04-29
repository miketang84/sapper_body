
extern crate sapper;
extern crate url;

use std::collections::HashMap;
use std::collections::hash_map::Entry::*;
use url::form_urlencoded;


use sapper::{Request, Result, Key};

pub type BodyMap = HashMap<String, Vec<String>>;

pub struct BodyParams;
impl Key for BodyParams { type Value = BodyMap; }

pub fn process(req: &mut Request) -> Result<()> {
    
    
    // should judge the content-type in the request headers
    let raw_body = req.raw_body().clone();
    match raw_body {
        Some(ref raw_body) => {
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
            
            req.get_ext_mut().insert::<BodyParams>(deduplicated);
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
