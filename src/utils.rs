/// Defines a repr enum that supports any value. If a value does not match any case, it will be parsed as `Unknown`.
#[macro_export]
#[doc(hidden)]
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
        #[cfg_attr(
            feature = "bevy_reflect",
            derive(bevy_reflect::Reflect),
            reflect(Debug, Clone, PartialEq)
        )]
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
                    $( $name::$variant => str::serialize($value, serializer), )+
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

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value.to_string(), )+
                    $name::Unknown(val) => val,
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
        #[cfg_attr(
            feature = "bevy_reflect",
            derive(bevy_reflect::Reflect),
            reflect(Debug, Clone, PartialEq)
        )]
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
                    $( $name::$variant => $ty::serialize(&$value, serializer), )+
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

        impl From<$name> for $ty {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => $value, )+
                    $name::Unknown(val) => val,
                }
            }
        }
    };
}

loose_enum! {
    /// An integer repr bool, with 0 being false and 1 being true. Any other value will be saved as `Unknown`.
    #[derive(Default, Copy)]
    LooseBool: i32 {
        #[default]
        False = 0,
        True = 1,
    }
}

impl LooseBool {
    pub fn is_true(&self) -> bool {
        match self {
            LooseBool::False => false,
            LooseBool::True => true,
            LooseBool::Unknown(_) => false,
        }
    }

    pub fn is_false(&self) -> bool {
        match self {
            LooseBool::False => true,
            LooseBool::True => false,
            LooseBool::Unknown(_) => false,
        }
    }
}

impl From<bool> for LooseBool {
    fn from(value: bool) -> Self {
        match value {
            true => Self::True,
            false => Self::False,
        }
    }
}
