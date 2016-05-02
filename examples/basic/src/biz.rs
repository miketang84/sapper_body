
use sapper::Result;
use sapper::SModule;
use sapper::Request;
use sapper::Response;
use sapper::SRouter;

#[derive(Clone)]
pub struct Biz;

use sapper_body_params::{ReqBodyParams, ReqJsonParams};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}


impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
        
        let mut response = Response::new();
        response.write_body("hello, boy!".to_string());
        
        Ok(response)
    }
    
    fn test(req: &mut Request) -> Result<Response> {
        // GET http://localhost:1337/test?a=1&b=2&c=3&a=4
        // Some({"a": ["1", "4"], "b": ["2"], "c": ["3"]})

        
        let mut response = Response::new();
        response.write_body("hello, tang gang gang!".to_string());
        
        Ok(response)
    }
    
    fn test_post(req: &mut Request) -> Result<Response> {
        
        println!("in test_post, raw_body: {:?}", req.raw_body());
        // POST http://localhost:1337/test 
        // with body a=1&b=2&c=3&a=4
        // Some({"a": ["1", "4"], "b": ["2"], "c": ["3"]})
        println!("{:?}", req.get_ext().get::<ReqBodyParams>());
        
        // queries is now an Option<HashMap<String, Vec<String>>>
        let body_params = req.get_ext().get::<ReqBodyParams>();
        if body_params.is_some() {
            
            // do something
            // let a = queries.get("a");
            // println!("{}", a);
            
        }
        
        let mut response = Response::new();
        response.write_body("hello, I'am !".to_string());
        
        Ok(response)
    }
    
    fn test_jsonbody(req: &mut Request) -> Result<Response> {
        
        println!("in test_jsonbody, raw_body: {:?}", req.raw_body());
        // POST http://localhost:1337/test 
        // with body a=1&b=2&c=3&a=4
        // Some({"a": ["1", "4"], "b": ["2"], "c": ["3"]})
        println!("{:?}", req.get_ext().get::<ReqJsonParams>());
        
        // queries is now an Option<HashMap<String, Vec<String>>>
        let json_params = req.get_ext().get::<ReqJsonParams>();
        if json_params.is_some() {
            println!("{:?}", json_params);
            // do something
            // let a = queries.get("a");
            // println!("{}", a);
            
        }
        else {
            
        }
        
        let mut response = Response::new();
        response.write_body("hello, from json handler !".to_string());
        
        Ok(response)
    }
    
    
    fn test_jsonbody2(req: &mut Request) -> Result<Response> {
        
        println!("in test_jsonbody2, raw_body: {:?}", req.raw_body());
        // POST http://localhost:1337/test 
        // with body a=1&b=2&c=3&a=4
        // Some({"a": ["1", "4"], "b": ["2"], "c": ["3"]})
        // println!("{:?}", req.get_ext().get::<ReqJsonParams>());
        
        // queries is now an Option<HashMap<String, Vec<String>>>
        // let object: Option<User> = req.get_ext().get::<ReqJsonParams>();
        // if object.is_some() {
        //     println!("{:?}", object);
        //     // do something
        //     // let a = queries.get("a");
        //     // println!("{}", a);
            
        // }
        // else {
            
        // }
        
        let mut response = Response::new();
        response.write_body("hello, from json2 handler !".to_string());
        
        Ok(response)
    }
}

// set before, after middleware, and add routers
impl SModule for Biz {
    
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in Biz before.");
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in Biz after.");
        
        Ok(())
    }
    
    // here add routers ....
    fn router(&self, router: &mut SRouter) -> Result<()> {
        // need to use Router struct here
        // XXX: here could not write as this, should record first, not parse it now
        
        
        router.get("/", Biz::index);
        router.get("/123", Biz::index);
        router.get("/test", Biz::test);
        router.post("/test", Biz::test_post);

        router.post("/test_jsonbody", Biz::test_jsonbody);
        
        router.post("/test_jsonbody2", Biz::test_jsonbody2);
        
        
        
        Ok(())
        
    }
    
    
}

