use std::collections::BTreeMap;

use actix_web::{web, Responder, HttpResponse};
use serde_json::{Value, json};
use serde_merge::Map;

pub fn map_get_value(key: &str, value: &BTreeMap<String, Value>) -> Value {
    value
        .get(key)
        .unwrap_or(&Value::Null)
        .to_owned()
}

pub fn map_get_str(key: &str, value: &BTreeMap<String, Value>) -> String {
    value
        .get(key)
        .unwrap_or(&Value::Null)
        .as_str()
        .unwrap_or(&"").to_owned()
}

pub fn map_get_f64(key: &str, value: &BTreeMap<String, Value>) -> f64 {
    value
        .get(key)
        .unwrap_or(&Value::Null)
        .as_f64()
        .unwrap_or(0.0).to_owned()
}

pub fn map_get_i64(key: &str, value: &BTreeMap<String, Value>) -> i64 {
    value
        .get(key)
        .unwrap_or(&Value::Null)
        .as_i64()
        .unwrap_or(0).to_owned()
}

pub fn map_get_array(key: &str, value: &BTreeMap<String, Value>) -> Vec<Value> {
    value
        .get(key)
        .unwrap_or(&Value::Null)
        .as_array()
        .unwrap_or(&Vec::new()).to_owned()
}

pub fn map_get(key: &str, value: Value) -> Value {
    value
        .as_object()
        .unwrap_or(&Map::new())
        .get(key)
        .unwrap_or(&Value::Null)
        .to_owned()
}

pub fn object_get(key: &str, value: Map) -> Value {
    value
        .get(key)
        .unwrap_or(&Value::Null)
        .to_owned()
}

pub fn to_array(map: Value) -> Vec<Value> {
    map.as_array().unwrap_or(&Vec::new()).to_owned()
}

pub fn to_object(map: Value) -> Map {
    map.as_object().unwrap_or(&Map::new()).to_owned()
}

pub fn to_f64(map: Value) -> f64 {
    map.as_f64().unwrap_or(0.0).to_owned()
}

pub fn to_i64(map: Value) -> i64 {
    map.as_i64().unwrap_or(0).to_owned()
}

pub fn to_str(map: Value) -> String {
    map.as_str().unwrap_or("").to_owned()
}

pub fn get_str_map(key: &str, value: &BTreeMap<String, String>) -> String {
    value
        .get(key)
        .unwrap_or(&String::new())
        .to_owned()
}

pub fn get_first(vec: Vec<Value>) -> Value  {
    vec.first().unwrap_or(&Value::Null).to_owned()
}