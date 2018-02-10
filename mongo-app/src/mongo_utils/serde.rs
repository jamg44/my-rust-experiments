extern crate serde_json;
extern crate bson;

use self::bson::Bson;
use self::serde_json::{ Value };
use mongodb::cursor::Cursor;
use mongodb::{Result};

//use bson::{ Bson, Document };
// use bson::oid::ObjectId;

// fn get_data_string(result: MongoResult<Document>) -> Result<Json, String> {
//     match result {
//         Ok(doc) => Ok(Bson::Document(doc).to_json()),
//         Err(e) => Err(format!("{}", e))
//     }
// }

/*pub fn json_value_from_cursor(cursor: Cursor) -> Result<Value> {
    let jsons: Result<Vec<Value>> = cursor
        .map(|doc| {
            let json: Value = Bson::Document(doc?).into();
            json["_id"] = json["_id"]["$oid"].clone(); // extract _id from $oid
            Ok(json)
        })
        .collect();

    Ok(jsons.map(Value::Array)?)
}*/

pub fn json_string_from_cursor(cursor: Cursor) -> String {
    let jsons: Result<Vec<Value>> = cursor
        .map(|doc| {
            let mut json: Value = Bson::Document(doc?).into();
            //json["_id"] = serde_json::to_value(22).unwrap();
            json["_id"] = json["_id"]["$oid"].clone(); // extract _id from $oid
            Ok(json)
        })
        .collect();
    let json = jsons.map(Value::Array).unwrap();
    json.to_string()
}
