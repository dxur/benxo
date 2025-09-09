use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[allow(unused_imports)]
use serde_json::json;
use ts_rs::TS;

#[derive(Debug, Clone, Default)]
pub enum JsonOption<T> {
    #[default]
    Undefined,
    Null,
    Value(T),
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

    pub fn ok_then<U, F: FnOnce(Option<T>) -> U>(self, f: F) -> Option<U> {
        match self {
            Self::Undefined => None,
            _ => Some(f(self.to_option())),
        }
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            JsonOption::Value(v) => Option::Some(v),
            _ => Option::None,
        }
    }
}

pub fn json_option<'de, D, T>(deserializer: D) -> Result<JsonOption<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Visitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = JsonOption<T>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "JsonOption<T>")
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            T::deserialize(deserializer).map(JsonOption::Value)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(JsonOption::Null)
        }
    }

    deserializer.deserialize_option(Visitor(std::marker::PhantomData))
}

impl<T: TS> TS for JsonOption<T> {
    type WithoutGenerics = JsonOption<ts_rs::Dummy>;
    type OptionInnerType = Self;

    fn decl() -> String {
        todo!()
    }
    fn decl_concrete() -> String {
        todo!()
    }
    fn name() -> String {
        format!(
            "undefined | null | {0}",
            <[_]>::into_vec(Box::new([<T as ::ts_rs::TS>::name()]),).join(", "),
        )
    }
    fn inline() -> String {
        todo!()
    }
    fn inline_flattened() -> String {
        todo!()
    }
    fn visit_generics(v: &mut impl ::ts_rs::TypeVisitor)
    where
        Self: 'static,
    {
        v.visit::<T>();
        <T as ::ts_rs::TS>::visit_generics(v);
    }
    fn visit_dependencies(v: &mut impl ::ts_rs::TypeVisitor)
    where
        Self: 'static,
    {
        <T as ::ts_rs::TS>::visit_generics(v);
        v.visit::<T>();
    }
}
