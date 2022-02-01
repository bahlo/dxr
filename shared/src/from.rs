use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::error::DxrError;
use crate::traits::FromDXR;
use crate::types::{Type, Value};

impl FromDXR for Value {
    fn from_dxr(value: &Value) -> Result<Value, DxrError> {
        Ok(value.clone())
    }
}

impl FromDXR for i32 {
    fn from_dxr(value: &Value) -> Result<i32, DxrError> {
        match value.inner() {
            Type::Integer(int) => Ok(*int),
            t => Err(DxrError::wrong_type(t.name(), "i4")),
        }
    }
}

#[cfg(feature = "i8")]
impl FromDXR for i64 {
    fn from_dxr(value: &Value) -> Result<i64, DxrError> {
        match value.inner() {
            Type::Long(long) => Ok(*long),
            t => Err(DxrError::wrong_type(t.name(), "i8")),
        }
    }
}

impl FromDXR for bool {
    fn from_dxr(value: &Value) -> Result<bool, DxrError> {
        match value.inner() {
            Type::Boolean(boo) => Ok(*boo),
            t => Err(DxrError::wrong_type(t.name(), "boolean")),
        }
    }
}

impl FromDXR for String {
    fn from_dxr(value: &Value) -> Result<String, DxrError> {
        match value.inner() {
            Type::String(string) => Value::string_unescape(string),
            t => Err(DxrError::wrong_type(t.name(), "string")),
        }
    }
}

impl FromDXR for f64 {
    fn from_dxr(value: &Value) -> Result<f64, DxrError> {
        match value.inner() {
            Type::Double(double) => Ok(*double),
            t => Err(DxrError::wrong_type(t.name(), "double")),
        }
    }
}

impl FromDXR for DateTime<Utc> {
    fn from_dxr(value: &Value) -> Result<DateTime<Utc>, DxrError> {
        match value.inner() {
            Type::DateTime(date) => Ok(*date),
            t => Err(DxrError::wrong_type(t.name(), "dateTime.iso8861")),
        }
    }
}

impl FromDXR for Vec<u8> {
    fn from_dxr(value: &Value) -> Result<Vec<u8>, DxrError> {
        match value.inner() {
            Type::Base64(bytes) => Ok(bytes.clone()),
            t => Err(DxrError::wrong_type(t.name(), "base64")),
        }
    }
}

#[cfg(feature = "nil")]
impl<T> FromDXR for Option<T>
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Option<T>, DxrError> {
        if let Type::Nil = value.inner() {
            Ok(None)
        } else {
            Ok(Some(T::from_dxr(value)?))
        }
    }
}

impl<T> FromDXR for Vec<T>
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Vec<T>, DxrError> {
        let values = match value.inner() {
            Type::Array { data } => Ok(data.inner()),
            t => Err(DxrError::wrong_type(t.name(), "array")),
        };

        values?.iter().map(|value| T::from_dxr(value)).collect()
    }
}

impl<T> FromDXR for HashMap<String, T>
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<HashMap<String, T>, DxrError> {
        let values = match value.inner() {
            Type::Struct { members } => Ok(members),
            t => Err(DxrError::wrong_type(t.name(), "struct")),
        };

        values?
            .iter()
            .map(|v| {
                let name = v.name().to_string();
                match T::from_dxr(v.inner()) {
                    Ok(value) => Ok((name, value)),
                    Err(error) => Err(error),
                }
            })
            .collect()
    }
}

// some implementations for exact numbers of values (with possibly different types)
impl FromDXR for () {
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        match value.inner() {
            Type::Array { data } => {
                let values = data.inner();

                match values.len() {
                    0 => Ok(()),
                    n => Err(DxrError::return_mismatch(n, 0)),
                }
            },
            Type::Nil => Ok(()),
            other => Err(DxrError::wrong_type(other.name(), "array | nil")),
        }
    }
}

impl<T> FromDXR for (T,)
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                1 => {
                    let value = values.get(0).unwrap();

                    Ok((T::from_dxr(value)?,))
                },
                n => Err(DxrError::return_mismatch(n, 1)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B> FromDXR for (A, B)
where
    A: FromDXR,
    B: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                2 => {
                    let v0 = values.get(0).unwrap();
                    let v1 = values.get(1).unwrap();

                    Ok((A::from_dxr(v0)?, B::from_dxr(v1)?))
                },
                n => Err(DxrError::return_mismatch(n, 2)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C> FromDXR for (A, B, C)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                3 => {
                    let v0 = values.get(0).unwrap();
                    let v1 = values.get(1).unwrap();
                    let v2 = values.get(2).unwrap();

                    Ok((A::from_dxr(v0)?, B::from_dxr(v1)?, C::from_dxr(v2)?))
                },
                n => Err(DxrError::return_mismatch(n, 3)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C, D> FromDXR for (A, B, C, D)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                4 => {
                    let v0 = values.get(0).unwrap();
                    let v1 = values.get(1).unwrap();
                    let v2 = values.get(2).unwrap();
                    let v3 = values.get(3).unwrap();

                    Ok((A::from_dxr(v0)?, B::from_dxr(v1)?, C::from_dxr(v2)?, D::from_dxr(v3)?))
                },
                n => Err(DxrError::return_mismatch(n, 4)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}
