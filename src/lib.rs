#[macro_export]
macro_rules! define_id {
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
        )]
        #[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[repr(transparent)]
        pub struct $name(i32);

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

        impl sea_orm::sea_query::Nullable for $name {
            fn null() -> sea_orm::Value {
                i32::null()
            }
        }

        impl sea_orm::IntoActiveValue<$name> for $name {
            fn into_active_value(self) -> sea_orm::ActiveValue<$name> {
                sea_orm::ActiveValue::Set(self)
            }
        }

        #[cfg(feature = "postgres-array")]
        impl sea_orm::sea_query::value::with_array::NotU8 for $name {}

        #[cfg(feature = "serde")]
        impl From<$name> for sea_orm::JsonValue {
            fn from(value: $name) -> Self {
                sea_orm::JsonValue::Number(value.0.into())
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
