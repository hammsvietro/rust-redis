#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate redis;

use std::{thread, time::{Duration, Instant}};

use redis::{Commands, Connection};
use rocket::{Outcome, Request, request::{self, FromRequest}};
use serde::{Serialize, Deserialize};

#[derive(Clone)]
struct Ip(String);
#[derive(Serialize, Deserialize)]
struct ProcessResponse {
    result: String,
    duration: u64
}



impl<'a, 'r> FromRequest<'a, 'r> for Ip {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Ip, ()> {
        let ip = request.client_ip();
        match ip {
            Some(ip) => Outcome::Success(Ip(ip.to_string())),
            None => Outcome::Failure((rocket::http::Status::BadRequest, ()))
        }
    }
}

fn get_connection() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_connection().unwrap()
}

fn clear_redis(connection: &mut Connection) {
    redis::cmd("FLUSHALL").query::<()>(connection).unwrap();
}

fn set_value_for_ip(connection: &mut Connection, ip: Ip, value: String) {
    let _: () = connection.set(ip.0, value).unwrap();
}

fn process() -> String {
    thread::sleep(std::time::Duration::from_secs(2));
    "hard processing".into()
}


#[get("/")]
fn index(ip: Ip) -> rocket_contrib::json::Json<ProcessResponse>  {
    let start = Instant::now();
    start.elapsed();
    let mut connection = get_connection();
    let value:Option<String> = match connection.get(ip.0.clone()) {
        Ok(val) => val,
        Err(_) => None
    };
    let response = ProcessResponse {
        result: match value {
            Some(value) => value,
            None => {
                let proc: String = process();
                set_value_for_ip(&mut connection, ip.clone(), proc.clone());
                proc
            }
        },
        duration: start.elapsed().as_secs()
    };

    rocket_contrib::json::Json(response)
}

#[get("/clear")]
fn clear() -> rocket::http::Status {
    let mut connection = get_connection();
    clear_redis(&mut connection);
    rocket::http::Status::Accepted
}

fn main() {
    // open redis connection
    redis::Client::open("redis://127.0.0.1/").unwrap();
    // listen
    rocket::ignite().mount("/", routes![index, clear]).launch();
}