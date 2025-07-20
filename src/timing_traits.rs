/// Represents any beatmap object that happens at a specific beat.
pub trait Timed {
    /// Returns the beat that an object takes place.
    fn get_beat(&self) -> f32;
}

/// Represents any beatmap object that happens over a duration of time in beats.
pub trait Duration: Timed {
    /// Returns the beat that an object ends.
    fn get_end_beat(&self) -> f32;
    /// Returns the length (in beats) that an object takes place.
    fn get_duration(&self) -> f32;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_timed {
    ($ident:ident::$beat:ident) => {
        impl crate::timing_traits::Timed for $ident {
            fn get_beat(&self) -> f32 {
                self.$beat
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_duration {
    ($ident:ident::$beat:ident, end: $end:ident) => {
        impl_timed!($ident::$beat);

        impl crate::timing_traits::Duration for $ident {
            fn get_end_beat(&self) -> f32 {
                self.$end
            }

            fn get_duration(&self) -> f32 {
                self.$end - self.$beat
            }
        }
    };
    ($ident:ident::$beat:ident, duration: $duration:ident) => {
        impl_timed!($ident::$beat);

        impl crate::timing_traits::Duration for $ident {
            fn get_end_beat(&self) -> f32 {
                self.$beat + self.$duration
            }

            fn get_duration(&self) -> f32 {
                self.$duration
            }
        }
    };
}
