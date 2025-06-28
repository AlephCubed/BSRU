#[macro_export]
macro_rules! loose_enum {
    // Special case for strings:
    (
        $(#[$outer:meta])*
        $name:ident: String
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        $(#[$outer])*
        pub enum $name {
            $(
                $(#[$meta])*
                $variant
            ),+,
            Unknown(String),
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;
                Ok(match val.as_str() {
                    $( $value => $name::$variant, )+
                    other => $name::Unknown(other.to_string()),
                })
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $name::$variant => str::serialize($value, serializer),
                    )+
                    $name::Unknown(val) => str::serialize(val, serializer),
                }
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $( $value => $name::$variant, )+
                    other => $name::Unknown(other.to_string()),
                }
            }
        }
    };
    // All other types:
    (
        $(#[$outer:meta])*
        $name:ident: $ty:ident
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        $(#[$outer])*
        pub enum $name {
            $(
                $(#[$meta])*
                $variant
            ),+,
            Unknown($ty),
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let val = $ty::deserialize(deserializer)?;
                Ok(match val {
                    $( $value => $name::$variant, )+
                    other => $name::Unknown(other),
                })
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $name::$variant => $ty::serialize(&$value, serializer),
                    )+
                    $name::Unknown(val) => $ty::serialize(val, serializer),
                }
            }
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                match value {
                    $( $value => $name::$variant, )+
                    other => $name::Unknown(other),
                }
            }
        }
    };
}

loose_enum! {
    #[derive(Default, Copy)]
    LooseBool: i32 {
        #[default]
        False = 0,
        True = 1,
    }
}

impl LooseBool {
    /// Returns as a bool, with unknown values counting as `false`.
    pub fn as_bool(&self) -> bool {
        match self {
            LooseBool::False => false,
            LooseBool::True => true,
            LooseBool::Unknown(_) => false,
        }
    }
}
