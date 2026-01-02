#[macro_export]
#[cfg(feature = "utoipa")]
macro_rules! define_id {
    ($name:ident) => {
        $crate::define_id_struct! {$name, utoipa::ToSchema}
        $crate::define_id_core! {$name}
        $crate::define_id_serde! {$name}
        $crate::define_id_schemars! {$name}
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
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_id_struct {
    ($name:ident) => {
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
        pub struct $name(i32);
    };

    ($name:ident, $($trait:path),+) => {
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
        pub struct $name(i32);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_id_serde {
    ($name:ident) => {
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
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "schema")]
macro_rules! define_id_schemars {
    ($name:ident) => {
        impl schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                format!("{}", stringify!($name)).into()
            }

            fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
                schemars::json_schema!({
                    "type": "integer",
                    "format": "int32",
                    "minimum": 0,
                    "maximum": i32::MAX,
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

        impl From<$name> for i32 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<&$name> for i32 {
            fn from(value: &$name) -> Self {
                value.0
            }
        }

        impl From<i32> for $name {
            fn from(value: i32) -> Self {
                $name(value)
            }
        }

        impl From<&i32> for $name {
            fn from(value: &i32) -> Self {
                $name(*value)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        define_id!(TestId);
    }
}
