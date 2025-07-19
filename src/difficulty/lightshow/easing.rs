use crate::loose_enum;
use simple_easing::*;

loose_enum! {
    #[derive(Default, Copy)]
    Easing: i32 {
        #[default]
        None = -1,

        Linear = 0,
        InQuad = 1,
        OutQuad = 2,
        InOutQuad = 3,
        InSine = 4,
        OutSine = 5,
        InOutSine = 6,
        InCubic = 7,
        OutCubic = 8,
        InOutCubic = 9,
        InQuart = 10,
        OutQuart = 11,
        InOutQuart = 12,
        InQuint = 13,
        OutQuint = 14,
        InOutQuint = 15,
        InExpo = 16,
        OutExpo = 17,
        InOutExpo = 18,
        InCirc = 19,
        OutCirc = 20,
        InOutCirc = 21,
        InBack = 22,
        OutBack = 23,
        InOutBack = 24,
        InElastic = 25,
        OutElastic = 26,
        InOutElastic = 27,
        InBounce = 28,
        OutBounce = 29,
        InOutBounce = 30,

        BeatSaberInOutBack = 100,
        BeatSaberInOutElastic = 101,
        BeatSaberInOutBounce = 102,
    }
}

impl Easing {
    pub fn ease(&self, num: f32) -> f32 {
        match self {
            Easing::None => 0.0,

            Easing::Linear => linear(num),
            Easing::InQuad => quad_in(num),
            Easing::OutQuad => quad_out(num),
            Easing::InOutQuad => quad_in_out(num),
            Easing::InSine => sine_in(num),
            Easing::OutSine => sine_out(num),
            Easing::InOutSine => sine_in_out(num),
            Easing::InCubic => cubic_in(num),
            Easing::OutCubic => cubic_out(num),
            Easing::InOutCubic => cubic_in_out(num),
            Easing::InQuart => quart_in(num),
            Easing::OutQuart => quart_out(num),
            Easing::InOutQuart => quart_in_out(num),
            Easing::InQuint => quint_in(num),
            Easing::OutQuint => quint_out(num),
            Easing::InOutQuint => quint_in_out(num),
            Easing::InExpo => expo_in(num),
            Easing::OutExpo => expo_out(num),
            Easing::InOutExpo => expo_in_out(num),
            Easing::InCirc => circ_in(num),
            Easing::OutCirc => circ_out(num),
            Easing::InOutCirc => circ_in_out(num),
            Easing::InBack => back_in(num),
            Easing::OutBack => back_out(num),
            Easing::InOutBack => back_in_out(num),
            Easing::InElastic => elastic_in(num),
            Easing::OutElastic => elastic_out(num),
            Easing::InOutElastic => elastic_in_out(num),
            Easing::InBounce => bounce_in(num),
            Easing::OutBounce => bounce_out(num),
            Easing::InOutBounce => bounce_in_out(num),

            Easing::BeatSaberInOutBack => back_in_out(num),
            Easing::BeatSaberInOutElastic => elastic_in_out(num),
            Easing::BeatSaberInOutBounce => bounce_in_out(num),
            Easing::Unknown(_) => 0.0,
        }
    }
}
