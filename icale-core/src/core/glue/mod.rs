use gluesql::Payload;
use serde::de::DeserializeOwned;

pub mod convert;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    NonDeserializeableResult,
    DeserializeFailed,
    NotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn deserialize<T: DeserializeOwned>(payload: Payload) -> Result<Vec<T>> {
    let (ops, json) = convert::convert_payload(payload);
    if &*ops != "SELECT" {
        return Err(Error::NonDeserializeableResult)
    }

    let result: Vec<T> = serde_json::from_value(json).or_else(|_| Err(Error::DeserializeFailed))?;
    Ok(result)
}

pub fn deserialize_one<T: DeserializeOwned>(payload: Payload) -> Result<T> {
    let mut results = deserialize(payload)?;
    results.truncate(1);
    results.pop().ok_or(Error::NotFound)
}


mod test {
    use super::*;
    use serde::Deserialize;
    use gluesql::{Row, Value};

    #[derive(Deserialize, Debug)]
    struct Person {
        name: String,
        email: String,
    }

    #[test]
    fn deserialize_test() {
        let payload = Payload::Select{
            labels: vec!["name".to_string(), "email".to_string()],
            rows: vec![
                Row(vec![
                    Value::Str("Candra Kharista".to_string()),
                    Value::Str("candra.kharista@gmail.com".to_string()),
                ],)
            ],
        };


        let persons: Vec<Person> = deserialize(payload).unwrap();
        let kharis = persons.iter().nth(0).unwrap();
        
        assert_eq!(kharis.name, "Candra Kharista");
        assert_eq!(kharis.email, "candra.kharista@gmail.com");

    }
}