// extern crate bson;

// use self::bson::Bson;
use mongodb::cursor::Cursor;
// use mongodb::Result;

extern crate serialize;
use self::serialize::json::{self, Json, ToJson};

pub fn json_value_from_cursor(cursor: Cursor) -> Json {
    let encoded = json::encode(&cursor).unwrap();

    // let jsons: Result<Vec<Value>> = cursor
    //     .map(|doc| {
    //         let json: Value = Bson::Document(doc?).into();
    //         Ok(json)
    //     })
    //     .collect();

    // Ok(jsons.map(Value::Array)?)
    Ok(encoded);
}
