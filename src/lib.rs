#[macro_export]
#[cfg(all(feature = "postgres", feature = "utoipa"))]
macro_rules! define_id {
    ($name:ident) => {
        $crate::define_id_struct! {$name, utoipa::ToSchema}
        $crate::define_id_core! {$name}
        $crate::define_id_serde! {$name}
        $crate::define_id_postgres! {$name}
    };
}

#[macro_export]
#[cfg(all(feature = "postgres", not(feature = "utoipa")))]
macro_rules! define_id {
    ($name:ident) => {
        $crate::define_id_struct! {$name}
        $crate::define_id_core! {$name}
        $crate::define_id_serde! {$name}
        $crate::define_id_postgres! {$name}
    };
}

#[macro_export]
#[cfg(all(feature = "utoipa", not(feature = "postgres")))]
macro_rules! define_id {
    ($name:ident) => {
        $crate::define_id_struct! {$name, utoipa::ToSchema}
        $crate::define_id_core! {$name}
        $crate::define_id_serde! {$name}
    };
}

#[macro_export]
#[cfg(all(not(feature = "postgres"), not(feature = "utoipa")))]
macro_rules! define_id {
    ($name:ident) => {
        $crate::define_id_struct! {$name}
        $crate::define_id_core! {$name}
        $crate::define_id_serde! {$name}
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
macro_rules! define_id_postgres {
    ($name:ident) => {
        impl sea_orm::sea_query::value::with_array::NotU8 for $name {}
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
macro_rules! define_id_core {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl sea_orm::TryFromU64 for $name {
            fn try_from_u64(n: u64) -> Result<Self, sea_orm::DbErr> {
                Ok($name(i32::try_from_u64(n)?))
            }
        }

        impl sea_orm::IntoActiveValue<$name> for $name {
            fn into_active_value(self) -> sea_orm::ActiveValue<$name> {
                sea_orm::ActiveValue::Set(self)
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
    use super::*;

    #[test]
    fn it_works() {
        define_id!(TestId);
    }
}
