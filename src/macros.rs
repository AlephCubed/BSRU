#[macro_export]
macro_rules! loose_enum {
    (
        $(#[$outer:meta])*
        $name:ident,
        {
            $(
                $(#[$meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        $(#[$outer])*
        pub enum $name {
            $(
                $(#[$meta])*
                $variant
            ),+,
            Unknown(i32),
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let val = i32::deserialize(deserializer)?;
                Ok(match val {
                    $( $value => $name::$variant, )+
                    other => $name::Unknown(other),
                })
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $name::$variant => serializer.serialize_i32($value),
                    )+
                    $name::Unknown(val) => serializer.serialize_i32(*val),
                }
            }
        }
    };
}
