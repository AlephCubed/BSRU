pub mod basic;
pub mod color;
pub mod easing;
pub mod filter;
pub mod rotation;
pub mod translation;

use crate::loose_enum;

loose_enum! {
    /// The distribution value does different things depending on the type.
    ///
    /// # [Beat Distribution](https://bsmg.wiki/mapping/map-format/lightshow.html#light-color-event-boxes-beat-distribution):
    /// ### Wave:
    /// The value represents the total time for all steps to complete.
    /// ### Step:
    /// The value represents the time until the next step is completed.
    ///
    /// # [Brightness](https://bsmg.wiki/mapping/map-format/lightshow.html#light-color-event-boxes-effect-distribution) and [Rotation Distribution](https://bsmg.wiki/mapping/map-format/lightshow.html#light-rotation-event-boxes-effect-distribution):
    /// ### Wave:
    /// The value represents the total difference between the first and last step.
    /// ### Step:
    /// The value represents the different between the current and next step.
    #[derive(Default, Copy)]
    DistributionType: i32 {
        #[default]
        Wave = 1,
        Step = 2,
    }
}

loose_enum! {
    /// Controls how the value is changed from the previous event.
    /// - Transition: The value will blend from the previous event's value, using the
    /// [easing](easing::Easing) value.
    /// - Extend: The event's value will be ignored, replaced with the values from the previous event.
    ///
    /// More info [here](https://bsmg.wiki/mapping/map-format/lightshow.html#light-rotation-events-type).
    #[derive(Default, Copy)]
    TransitionType: i32 {
        #[default]
        Transition = 0,
        Extend = 1,
    }
}

loose_enum! {
    #[derive(Default, Copy)]
    Axis: i32 {
        #[default]
        X = 0,
        Y = 1,
        Z = 2,
    }
}
