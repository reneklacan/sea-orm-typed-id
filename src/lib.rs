#[macro_export]
#[cfg(feature = "utoipa")]
macro_rules! define_id {
    ($name:ident) => {
        $crate::define_id_struct! {$name, utoipa::ToSchema}
        $crate::define_id_core! {$name}
        $crate::define_id_serde! {$name}
        $crate::define_id_schemars! {$name}
    };

    ($name:ident, i32) => {
        $crate::define_id_struct! {$name; i32; utoipa::ToSchema}
        $crate::define_id_core! {$name; i32}
        $crate::define_id_serde! {$name; i32}
        $crate::define_id_schemars! {$name; i32}
    };

    ($name:ident, i64) => {
        $crate::define_id_struct! {$name; i64; utoipa::ToSchema}
        $crate::define_id_core! {$name; i64}
        $crate::define_id_serde! {$name; i64}
        $crate::define_id_schemars! {$name; i64}
    };
}

#[macro_export]
#[cfg(not(feature = "utoipa"))]
macro_rules! define_id {
    ($name:ident) => {
        $crate::define_id_struct! {$name}
        $crate::define_id_core! {$name}
        $crate::define_id_serde! {$name}
        $crate::define_id_schemars! {$name}
    };

    ($name:ident, i32) => {
        $crate::define_id_struct! {$name; i32}
        $crate::define_id_core! {$name; i32}
        $crate::define_id_serde! {$name; i32}
        $crate::define_id_schemars! {$name; i32}
    };

    ($name:ident, i64) => {
        $crate::define_id_struct! {$name; i64}
        $crate::define_id_core! {$name; i64}
        $crate::define_id_serde! {$name; i64}
        $crate::define_id_schemars! {$name; i64}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_id_struct {
    ($name:ident) => {
        $crate::define_id_struct! {$name; i32}
    };

    ($name:ident; $type:ty) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            sea_orm::DeriveValueType,
            serde::Serialize,
            serde::Deserialize
        )]
        #[repr(transparent)]
        pub struct $name($type);
    };

    ($name:ident, $($trait:path),+) => {
        $crate::define_id_struct! {$name; i32; $($trait),+}
    };

    ($name:ident; $type:ty; $($trait:path),+) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            sea_orm::DeriveValueType,
            serde::Serialize,
            serde::Deserialize,
            $($trait),+
        )]
        #[repr(transparent)]
        pub struct $name($type);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_id_serde {
    ($name:ident) => {
        $crate::define_id_serde! {$name; i32}
    };

    ($name:ident; $type:ty) => {
        impl From<$name> for sea_orm::JsonValue {
            fn from(value: $name) -> Self {
                sea_orm::JsonValue::Number(value.0.into())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "schema"))]
macro_rules! define_id_schemars {
    ($name:ident) => {};
    ($name:ident; i32) => {};
    ($name:ident; i64) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "schema")]
macro_rules! define_id_schemars {
    ($name:ident) => {
        $crate::define_id_schemars! {$name; i32}
    };

    ($name:ident; i32) => {
        $crate::define_id_schemars! {@impl $name, i32, "int32"}
    };

    ($name:ident; i64) => {
        $crate::define_id_schemars! {@impl $name, i64, "int64"}
    };

    (@impl $name:ident, $type:ty, $format:literal) => {
        impl schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                format!("{}", stringify!($name)).into()
            }

            fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "integer",
                    "format": $format,
                    "minimum": 0,
                    "maximum": <$type>::MAX,
                    "description": format!("A unique identifier for the {}", stringify!($name)),
                })
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_id_core {
    ($name:ident) => {
        $crate::define_id_core! {$name; i32}
    };

    ($name:ident; $type:ty) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<&$name> for sea_orm::Value {
            fn from(value: &$name) -> Self {
                sea_orm::Value::from(*value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                if value.0 < 0 {
                    panic!("Negative ID. This should not happen.");
                }
                value.0 as u64
            }
        }

        impl From<$name> for $type {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<&$name> for $type {
            fn from(value: &$name) -> Self {
                value.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                $name(value)
            }
        }

        impl From<&$type> for $name {
            fn from(value: &$type) -> Self {
                $name(*value)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use sea_orm::sea_query::{ColumnType, ValueType};

    define_id!(DefaultI32Id);
    define_id!(ExplicitI32Id, i32);
    define_id!(I64Id, i64);

    define_id_struct!(LegacyHelperId);
    define_id_core!(LegacyHelperId);
    define_id_serde!(LegacyHelperId);
    define_id_schemars!(LegacyHelperId);

    #[test]
    fn default_and_explicit_i32_ids_use_integer_values() {
        let default_id = DefaultI32Id::from(42);
        let explicit_id = ExplicitI32Id::from(42);

        assert_eq!(i32::from(default_id), i32::from(explicit_id));
        assert!(matches!(
            sea_orm::Value::from(default_id),
            sea_orm::Value::Int(Some(42))
        ));
        assert!(matches!(
            <DefaultI32Id as ValueType>::column_type(),
            ColumnType::Integer
        ));
        assert!(matches!(
            <ExplicitI32Id as ValueType>::column_type(),
            ColumnType::Integer
        ));
    }

    #[test]
    fn i64_ids_preserve_large_values_across_conversions() {
        let value = i64::from(i32::MAX) + 1;
        let id = I64Id::from(value);

        assert_eq!(i64::from(id), value);
        assert_eq!(i64::from(&id), value);
        assert_eq!(I64Id::from(&value), id);
        assert_eq!(u64::from(id), value as u64);
        assert_eq!(id.to_string(), value.to_string());
        assert_eq!(serde_json::to_value(id).unwrap(), serde_json::json!(value));
        assert_eq!(sea_orm::JsonValue::from(id), serde_json::json!(value));
    }

    #[test]
    fn i64_ids_use_big_integer_values() {
        let value = i64::from(i32::MAX) + 1;
        let id = I64Id::from(value);

        assert!(matches!(
            sea_orm::Value::from(id),
            sea_orm::Value::BigInt(Some(inner)) if inner == value
        ));
        assert!(matches!(
            sea_orm::Value::from(&id),
            sea_orm::Value::BigInt(Some(inner)) if inner == value
        ));
        assert!(matches!(
            <I64Id as ValueType>::column_type(),
            ColumnType::BigInteger
        ));
    }

    #[test]
    #[should_panic(expected = "Negative ID. This should not happen.")]
    fn i64_ids_reject_negative_u64_conversion() {
        let _ = u64::from(I64Id::from(-1));
    }

    #[test]
    fn legacy_helper_macros_still_generate_i32_ids() {
        let id = LegacyHelperId::from(42);

        assert_eq!(i32::from(id), 42);
    }

    #[cfg(feature = "schema")]
    #[test]
    fn schemars_uses_the_backing_integer_width() {
        let i32_schema = serde_json::to_value(schemars::schema_for!(DefaultI32Id)).unwrap();
        let i64_schema = serde_json::to_value(schemars::schema_for!(I64Id)).unwrap();

        assert_eq!(i32_schema["format"], "int32");
        assert_eq!(i32_schema["minimum"], 0);
        assert_eq!(i32_schema["maximum"], i32::MAX);
        assert_eq!(i64_schema["format"], "int64");
        assert_eq!(i64_schema["minimum"], 0);
        assert_eq!(i64_schema["maximum"], i64::MAX);
    }

    #[cfg(feature = "utoipa")]
    #[test]
    fn utoipa_uses_the_backing_integer_width() {
        let i32_schema =
            serde_json::to_value(<DefaultI32Id as utoipa::PartialSchema>::schema()).unwrap();
        let i64_schema = serde_json::to_value(<I64Id as utoipa::PartialSchema>::schema()).unwrap();

        assert_eq!(i32_schema["type"], "integer");
        assert_eq!(i32_schema["format"], "int32");
        assert_eq!(i64_schema["type"], "integer");
        assert_eq!(i64_schema["format"], "int64");
    }
}
