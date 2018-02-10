#![feature(plugin)]
#![plugin(rocket_codegen)]
//#[macro_use]

extern crate rocket;
extern crate rocket_contrib;

//extern crate rustc_serialize;
use std::sync::Mutex;

//#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate serde_json;

use mongodb::{Client, ThreadedClient, Result};
use mongodb::db::ThreadedDatabase;
//use mongodb::error::Result as MongoResult;
use mongodb::cursor::Cursor;

use serde_json::Value;

use bson::Bson;

//use bson::{ Bson, Document };
// use bson::oid::ObjectId;

//use rustc_serialize::json::{Json, ToJson};

//use rocket_contrib::Json;

// #[derive(RustcDecodable, RustcEncodable)]
// struct User {
//     firstname: String,
//     lastname: String,
//     email: String
// }

// fn get_data_string(result: MongoResult<Document>) -> Result<Json, String> {
//     match result {
//         Ok(doc) => Ok(Bson::Document(doc).to_json()),
//         Err(e) => Err(format!("{}", e))
//     }
// }

type DbConn = Mutex<Client>;
use rocket::{State}; // Rocket

fn json_value_from_cursor(cursor: Cursor) -> Result<Value> {
    let jsons: Result<Vec<Value>> = cursor
        .map(|doc| {
            let json: Value = Bson::Document(doc?).into();
            Ok(json)
        })
        .collect();

    Ok(jsons.map(Value::Array)?)
}

// #[get("/", format = "application/json")] - para cuando piden json
#[get("/")]
fn index(client: State<DbConn>) -> String {
    //fn index() -> &'static str {
    let agentes = client.lock()
        .expect("db connection lock")
        .db("cursonode").collection("agentes");
    //let agentes = client.db("cursonode").collection("agentes");
    let cursor = agentes.find(None, None)
        //.ok().expect("Failed to execute find.");
        .unwrap();

    //"Hello, world!"

    let json = json_value_from_cursor(cursor).expect("Unable to receive all documents from cursor");
    let result = format!("{}", json);

    result
}

fn main() {

    // Me conecto a la base de datos
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    /*let movies = client.db("cursonode").collection("agentes");
    let cursor = movies.find(None, None)
        //.ok().expect("Failed to execute find.");
        .unwrap();
    */
    
    /*for result in cursor {
        if let Ok(item) = result {
            println!("{}", item);
            //println!("{:?}", item.get("_id").unwrap());
            //println!("to_string {:?}", item.get("_id").unwrap().to_string());
            //println!("to_json {:?}", item.get("_id").unwrap().to_json());
            //let mongo_id  = item.get("_id").unwrap().to_string();
            //let name     = item.get("name").map(|s| s.to_string());
            //let age      = item.get("age").map(|s| s.to_string());
        }
    }*/

    // Opening for the JSON string to be returned
    //let mut data_result = "{\"data\":[".to_owned();

    // for (i, result) in cursor.enumerate() {
    //     match get_data_string(result) {
    //         Ok(data) => {
    //             let string_data = if i == 0 {
    //                 format!("{}", data)
    //             } else {
    //                 format!("{},", data)
    //             };

    //             data_result.push_str(&string_data);
    //         },

    //         Err(e) => {
    //             println!("{}", e);
    //             //return response.send(format!("{}", e))
    //         }
    //     }
    // }

    // Close the JSON string
    //data_result.push_str("]}");

    //response.set(MediaType::Json);

    //println!("{}", data_result);

    //let item = cursor.next();
    // cursor.next() returns an Option<Result<Document>>
    // match item {
    //     Some(Ok(doc)) => match doc.get("title") {
    //         Some(&Bson::String(ref title)) => println!("{}", title),
    //         _ => panic!("Expected title to be a string!"),
    //     },
    //     Some(Err(_)) => panic!("Failed to get next from server!"),
    //     None => panic!("Server returned no results!"),
    // }

    rocket::ignite().mount("/", routes![index])
        .manage(Mutex::new(client))
        .launch();
}
