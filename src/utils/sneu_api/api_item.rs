use httpstatus::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Debug)]
pub struct ApiItem<T: Serialize> {
    #[serde(deserialize_with = "status_code_from_u16")]
    #[allow(dead_code)]
    pub status_code: StatusCode,
    pub item: T,
}

fn status_code_from_u16<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<StatusCode, D::Error> {
    let code: u16 = Deserialize::deserialize(deserializer)?;

    Ok(StatusCode::from(code))
}
