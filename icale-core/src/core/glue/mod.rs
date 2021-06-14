use gluesql::Payload;
use serde::de::DeserializeOwned;
use std::fmt::Display;

pub mod convert;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    NonDeserializeableResult,
    DeserializeFailed,
    NotFound,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            Error::NonDeserializeableResult => fmt.write_str("cannot deserialize result payload, only works for SELECT query result")?,
            Error::DeserializeFailed => fmt.write_str("failed to deserialize")?,
            Error::NotFound => fmt.write_str("result not found")?,
        };

        Ok(())
    }
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
    use super::{Payload, deserialize, deserialize_one};
    use serde::Deserialize;
    use gluesql::{Row, Value};

    #[derive(Deserialize, Debug)]
    struct Person {
        name: String,
        email: String,
    }

    fn empty_payload() -> Payload {
        Payload::Select{
            labels: vec![],
            rows: vec![],
        }
    }

    fn empty_row_payload() -> Payload {
        Payload::Select{
            labels: vec!["name".to_string(), "email".to_string()],
            rows: vec![],
        }
    }

    fn single_row_payload() -> Payload {
        Payload::Select{
            labels: vec!["name".to_string(), "email".to_string()],
            rows: vec![
                Row(vec![
                    Value::Str("Candra Kharista".to_string()),
                    Value::Str("candra.kharista@mail.com".to_string()),
                ]),
            ],
        }
    }

    fn multi_row_payload() -> Payload {
        Payload::Select{
            labels: vec!["name".to_string(), "email".to_string()],
            rows: vec![
                Row(vec![
                    Value::Str("Candra Kharista".to_string()),
                    Value::Str("candra.kharista@mail.com".to_string()),
                ]),
                Row(vec![
                    Value::Str("Kharista Putra".to_string()),
                    Value::Str("kharista.putra@mail.com".to_string()),
                ]),
            ],
        }
    }

    #[test]
    fn deserialize_test() {
        let payload = multi_row_payload();

        let persons: Vec<Person> = deserialize(payload).unwrap();
        assert_eq!(persons.len(), 2);

        let person1 = persons.iter().nth(0).unwrap();


        let person2 = persons.iter().nth(1).unwrap();
        assert_eq!(person2.name, "Kharista Putra");
        assert_eq!(person2.email, "kharista.putra@mail.com");
    }

    #[test]
    fn deserialize_one_test() {
        // expect success
        let person1: Person = deserialize_one(multi_row_payload()).unwrap();
        let another_person1: Person = deserialize_one(single_row_payload()).unwrap();
        assert_eq!(person1.name, another_person1.name);
        assert_eq!(person1.email, another_person1.email);
        assert_eq!(person1.name, "Candra Kharista");
        assert_eq!(person1.email, "candra.kharista@mail.com");

        // expect error
        deserialize_one::<Person>(empty_payload()).unwrap_err();
        deserialize_one::<Person>(empty_row_payload()).unwrap_err();
    }
}