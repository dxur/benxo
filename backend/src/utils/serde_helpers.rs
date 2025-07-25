use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[allow(unused_imports)]
use serde_json::json;
use ts_rs::TS;

#[derive(Debug, Clone, TS)]
pub enum JsonOption<T> {
    Undefined,
    Null,
    Value(T),
}

pub struct JsonOptionVisitor<T> {
    marker: std::marker::PhantomData<T>,
}

impl<T> JsonOption<T> {
    pub const fn is_undefined(&self) -> bool {
        matches!(*self, JsonOption::Undefined)
    }
    pub const fn undefined() -> Self {
        JsonOption::Undefined
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> JsonOption<U> {
        match self {
            Self::Undefined => JsonOption::<_>::Undefined,
            Self::Null => JsonOption::<_>::Null,
            Self::Value(x) => JsonOption::<_>::Value(f(x)),
        }
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            JsonOption::Value(v) => Option::Some(v),
            _ => Option::None,
        }
    }
}

impl<T> Default for JsonOption<T> {
    fn default() -> Self {
        JsonOption::Undefined
    }
}

impl<T> Serialize for JsonOption<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            JsonOption::Undefined => serializer.serialize_none(), // Problem here, I can't figure out how to NOT serialize a null (or serialize nothing)
            // the problem is solved by #[serde(skip_serializing_if = "JsonOption::is_undefined")] on the struct field
            // however if we could return somethin here that is not an error and does not cause a serialization to occur, serde skip_serializing_if would not be necessary on the struct
            JsonOption::Null => serializer.serialize_none(),
            JsonOption::Value(ref value) => value.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for JsonOption<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(JsonOptionVisitor::<T> {
            marker: std::marker::PhantomData,
        })
    }
}

impl<'de, T> serde::de::Visitor<'de> for JsonOptionVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = JsonOption<T>;

    #[inline]
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("JsonOption<T>")
    }

    #[inline]
    fn visit_none<E>(self) -> Result<JsonOption<T>, E>
    where
        E: serde::de::Error,
    {
        Ok(JsonOption::Null)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(JsonOption::Value)
    }
}

#[test]
fn test_json_option() {
    // This is not an actual TEST with asserts, just look at the standard output and check the print line messages
    //////// TYPES

    #[derive(
        // Default, // this unfortunately deserializes required strings to "", if this wasn't the case #[serde(default)] would not be necessary on each field
        Debug,
        Serialize,
        Deserialize,
    )]
    #[serde(deny_unknown_fields)]
    // #[serde(default)] // this unfortunately deserializes required strings to ""
    struct TestStruct {
        name: String,
        nik: String,
        #[serde(default)] // use default 'undefined' if value not present
        #[serde(skip_serializing_if = "JsonOption::is_undefined")]
        // do not serialize undefined
        // it would be awesome if we could place this on the JsonOption enum variant directly, however that causes an error in either serial or deserial
        age: JsonOption<u8>,
        #[serde(default)]
        #[serde(skip_serializing_if = "JsonOption::is_undefined")]
        p5: JsonOption<InnerStruct>,
        #[serde(default)]
        #[serde(rename = "coolValueUnit")]
        #[serde(skip_serializing_if = "JsonOption::is_undefined")]
        unit: JsonOption<()>,
        #[serde(alias = "seTwo")] // can deser both se and setwo
        #[serde(default)]
        #[serde(skip_serializing_if = "JsonOption::is_undefined")]
        se: JsonOption<SomeEnum>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum SomeEnum {
        #[serde(rename = "UAN")]
        OneFirst,
        TwoSecond,
        #[serde(rename = "TRE")]
        Three,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename = "AnotherName!")] // currently buggy : https://github.com/serde-rs/serde/issues/2402
    struct InnerStruct {
        x: u32,
    }

    // let option = JsonOption::Value(3).to_option();

    //////// TEST

    // Serialize to JSON undefined (does not serialize undefined)
    let person = TestStruct {
        name: "John Doe".to_owned(),
        nik: "John nik Doe".to_owned(),
        age: JsonOption::Undefined, // field will be omitted
        p5: JsonOption::Undefined,
        unit: JsonOption::Undefined,
        se: JsonOption::Undefined,
    };

    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Serialize to JSON null (serializes null)
    let person = TestStruct {
        name: "Jane Doe".to_owned(),
        nik: "John nik Doe".to_owned(),
        age: JsonOption::Null, // field will be set to null
        p5: JsonOption::Null,
        unit: JsonOption::Null,
        se: JsonOption::Null,
    };

    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Serialize to JSON value (serializes value as if JsonOption wasn't there)
    let person = TestStruct {
        name: "Jim Doe".to_owned(),
        nik: "Jim nik Doe".to_owned(),
        age: JsonOption::Value(30), // field will be set to 30
        p5: JsonOption::Value(InnerStruct { x: 3 }),
        unit: JsonOption::Value(()),
        se: JsonOption::Value(SomeEnum::OneFirst),
    };

    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Serialize to JSON value (serializes value as if JsonOption wasn't there) (se is two second variant)
    let person = TestStruct {
        name: "Jim Doe".to_owned(),
        nik: "Jim nik Doe".to_owned(),
        age: JsonOption::Value(30), // field will be set to 30
        p5: JsonOption::Value(InnerStruct { x: 3 }),
        unit: JsonOption::Value(()),
        se: JsonOption::Value(SomeEnum::TwoSecond),
    };

    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // Deserialize Errors, garbage
    match serde_json::from_str::<TestStruct>(r#"{ dsd[] }"#) {
        Ok(deserialized) => {
            println!("Deserialized: {:?}", deserialized);
        }
        Err(e) => {
            println!("Deserialize ERROR: {}", e);
        }
    }

    // Deserialize Errors, missing required field (non JsonOption field)
    match serde_json::from_str::<TestStruct>(r#"{ "name" : "Got A Name" }"#) {
        Ok(deserialized) => {
            println!("Deserialized: {:?}", deserialized);
        }
        Err(e) => {
            println!("Deserialize ERROR: {}", e);
        }
    }

    // Deserialize Errors, unknown field this is because #[serde(deny_unknown_fields)]
    match serde_json::from_value::<TestStruct>(
        json!({"name": "Janet Doe", "nik" : "2", "surname" : "Doe"}),
    ) {
        Ok(deserialized) => {
            println!("Deserialized: {:?}", deserialized);
        }
        Err(e) => {
            println!("Deserialize ERROR: {}", e);
        }
    }

    // Deserialize Errors, wrong type
    match serde_json::from_value::<TestStruct>(
        json!({"name": "Janet Doe", "nik" : "2", "p5" : "inv_val"}),
    ) {
        Ok(deserialized) => {
            println!("Deserialized: {:?}", deserialized);
        }
        Err(e) => {
            println!("Deserialize ERROR: {}", e);
        }
    }

    // Deserialize optional Undefined, you should see JsonOption::Undefined rather than Option::None
    let deserialized: TestStruct =
        serde_json::from_value(json!({"name": "Janet Doe", "nik" : "2"})).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // Deserialize optional Null, you should see JsonOption::Null rather than Option::None
    let deserialized: TestStruct = serde_json::from_value(
        json!({"name": "Janet Doe", "nik" : "2", "age" : null, "p5":null, "coolValueUnit":null}),
    )
    .unwrap();
    println!("Deserialized: {:?}", deserialized);

    // Deserialize Full with substructure
    let deserialized: TestStruct =
        serde_json::from_value(json!({"name": "Janet Doe", "nik" : "2", "age" : 45, "p5":{"x":3}}))
            .unwrap();
    println!("Deserialized: {:?}", deserialized);
}
