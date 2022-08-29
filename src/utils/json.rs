use serde::Serialize;

pub fn stringify<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap_or_else(|e| panic!("stringify() failed: {e:?}"))
}
