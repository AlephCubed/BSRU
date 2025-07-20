use crate::info::color_scheme::{Color, ColorScheme};
use crate::info::{AllDirectionEnvironment, Environment};

macro_rules! color_scheme {
    (
        $id:literal,
        $note_left:expr,
        $note_right:expr,
        $wall:expr,
        $light_primary:expr,
        $light_secondary:expr,
        $boost_light_primary:expr,
        $boost_light_secondary:expr
    ) => {
        ColorScheme {
            id: $id.to_string(),
            note_left: $note_left,
            note_right: $note_right,
            wall: $wall,
            light_primary: $light_primary,
            light_secondary: $light_secondary,
            boost_light_primary: $boost_light_primary,
            boost_light_secondary: $boost_light_secondary,
        }
    };

    (
        $id:literal,
        $note_left:expr,
        $note_right:expr,
        $wall:expr,
        $light_primary:expr,
        $light_secondary:expr
    ) => {
        ColorScheme {
            id: $id.to_string(),
            note_left: $note_left,
            note_right: $note_right,
            wall: $wall,
            light_primary: $light_primary,
            light_secondary: $light_secondary,
            boost_light_primary: $light_primary,
            boost_light_secondary: $light_secondary,
        }
    };
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Color {
            red: $r,
            green: $g,
            blue: $b,
            alpha: 1.0,
        }
    };
}

impl Environment {
    /// Values taken from [the wiki](https://bsmg.wiki/mapping/lighting-defaults.html#current-colors)
    /// and Kival Evan's [Typescript library](https://github.com/KivalEvan/BeatSaber-JSMap/blob/ef8afc42ab90e2f1100f1a163fa810ec56b6a9f8/src/beatmap/shared/colorScheme.ts).
    ///
    /// ChatGPT was used to help translate between formats, so there could be hallucinations.
    ///
    /// White light colors are not currently supported, and are therefor ignored.
    pub fn get_color_scheme(&self) -> ColorScheme {
        match self {
            Environment::Unknown(_)
            | Environment::TheFirst
            | Environment::Triangle
            | Environment::Nice
            | Environment::BigMirror
            | Environment::Monstercat
            | Environment::ImagineDragons
            | Environment::PanicAtTheDisco => ColorScheme::default(),
            Environment::KDA => color_scheme!(
                "KDA",
                rgb!(0.6588235, 0.2627451, 0.1607843),
                rgb!(0.5019608, 0.08235294, 0.572549),
                rgb!(1.0, 0.3960785, 0.2431373),
                rgb!(1.0, 0.3960785, 0.2431373),
                rgb!(0.7607844, 0.1254902, 0.8666667)
            ),
            Environment::CrabRave => color_scheme!(
                "CrabRave",
                rgb!(0.0, 0.7130001, 0.07806564),
                rgb!(0.04805952, 0.5068096, 0.734),
                rgb!(0.0, 0.8117648, 0.09019608),
                rgb!(0.134568, 0.756, 0.1557533),
                rgb!(0.05647058, 0.6211764, 0.9)
            ),
            Environment::Origins => color_scheme!(
                "Origins",
                rgb!(0.6792453, 0.5712628, 0.0),
                rgb!(0.7075472, 0.0, 0.5364411),
                rgb!(0.06167676, 0.2869513, 0.3962264),
                rgb!(0.4910995, 0.6862745, 0.7),
                rgb!(0.03844783, 0.6862745, 0.9056604)
            ),
            Environment::RocketLeague => color_scheme!(
                "Rocket",
                rgb!(1.0, 0.4980392, 0.0),
                rgb!(0.0, 0.5294118, 1.0),
                rgb!(0.3176471, 0.6117647, 0.7254902),
                rgb!(0.9, 0.4866279, 0.3244186),
                rgb!(0.4, 0.7180724, 1.0)
            ),
            Environment::GreenDay | Environment::GreenDayGrenade => color_scheme!(
                "GreenDay",
                rgb!(0.2588235, 0.7843138, 0.01960784),
                rgb!(0.0, 0.7137255, 0.6705883),
                rgb!(0.0, 0.8117648, 0.09019608),
                rgb!(0.0, 0.7137255, 0.6705883),
                rgb!(0.2588235, 0.7843137, 0.01960784)
            ),
            Environment::Timbaland => color_scheme!(
                "Timbaland",
                rgb!(0.5019608, 0.5019608, 0.5019608),
                rgb!(0.1, 0.5517647, 1.0),
                rgb!(0.5, 0.5, 0.5),
                rgb!(0.1, 0.5517647, 1.0),
                rgb!(0.1, 0.5517647, 1.0)
            ),
            Environment::FitBeat => color_scheme!(
                "FitBeat",
                rgb!(0.8000001, 0.6078432, 0.1568628),
                rgb!(0.7921569, 0.1607843, 0.682353),
                rgb!(0.2784314, 0.2784314, 0.4),
                rgb!(0.8, 0.5594772, 0.5594772),
                rgb!(0.5594772, 0.5594772, 0.8)
            ),
            Environment::LinkinPark => color_scheme!(
                "LinkinPark",
                rgb!(0.6627451, 0.1643608, 0.1690187),
                rgb!(0.3870196, 0.5168997, 0.5568628),
                rgb!(0.6627451, 0.1647059, 0.172549),
                rgb!(0.7529412, 0.672753, 0.5925647),
                rgb!(0.6241197, 0.6890281, 0.709),
                rgb!(0.922, 0.5957885, 0.255394),
                rgb!(0.282353, 0.4586275, 0.6235294)
            ),
            Environment::BTS => color_scheme!(
                "BTS",
                rgb!(1.0, 0.09019607, 0.4059771),
                rgb!(0.8018868, 0.0, 0.7517689),
                rgb!(0.6698113, 0.1800908, 0.5528399),
                rgb!(0.7843137, 0.1254902, 0.5010797),
                rgb!(0.6941177, 0.1254902, 0.8666667),
                rgb!(0.9019608, 0.5411765, 1.0),
                rgb!(0.3490196, 0.8078431, 1.0)
            ),
            Environment::Kaleidoscope => color_scheme!(
                "Kaleidoscope",
                rgb!(0.65882355, 0.1254902, 0.1254902),
                rgb!(0.28235295, 0.28235295, 0.28235295),
                rgb!(0.25098041, 0.25098041, 0.25098041),
                rgb!(0.65882355, 0.1254902, 0.1254902),
                rgb!(0.47058824, 0.47058824, 0.47058824),
                rgb!(0.50196081, 0.0, 0.0),
                rgb!(0.49244517, 0.0, 0.53725493)
            ),
            Environment::Interscope => color_scheme!(
                "Interscope",
                rgb!(0.726415, 0.62691, 0.31181),
                rgb!(0.589571, 0.297888, 0.723),
                rgb!(0.588235, 0.298039, 0.721569),
                rgb!(0.724254, 0.319804, 0.913725),
                rgb!(0.764706, 0.758971, 0.913725),
                rgb!(0.792453, 0.429686, 0.429868),
                rgb!(0.7038, 0.715745, 0.765)
            ),
            Environment::Skrillex => color_scheme!(
                "Skrillex",
                rgb!(0.69803923, 0.14117648, 0.36862746),
                rgb!(0.32933334, 0.32299998, 0.38),
                rgb!(0.15686275, 0.60392159, 0.60392159),
                rgb!(0.80000001, 0.28000003, 0.58594489),
                rgb!(0.06525807, 0.57800001, 0.56867743),
                rgb!(0.81176478, 0.30588236, 0.30588236),
                rgb!(0.27843139, 0.80000001, 0.44597632)
            ),
            Environment::BillieEilish => color_scheme!(
                "BillieEilish",
                rgb!(0.8, 0.64481932, 0.432),
                rgb!(0.54808509, 0.61276591, 0.64),
                rgb!(0.71325314, 0.56140977, 0.78301889),
                rgb!(0.81960785, 0.442, 0.184),
                rgb!(0.94117647, 0.70677096, 0.56470591),
                rgb!(0.8, 0.0, 0.0),
                rgb!(0.55686277, 0.7019608, 0.77647066)
            ),
            Environment::Spooky => color_scheme!(
                "Spooky",
                rgb!(0.81960785, 0.49807876, 0.27702752),
                rgb!(0.37894738, 0.35789475, 0.4),
                rgb!(0.81960791, 0.44313729, 0.18431373),
                rgb!(0.90196079, 0.23009226, 0.0),
                rgb!(0.46005884, 0.56889427, 0.92941177),
                rgb!(0.33768433, 0.63207543, 0.33690813),
                rgb!(0.60209066, 0.3280082, 0.85849059)
            ),
            Environment::LadyGaga => color_scheme!(
                "LadyGaga",
                rgb!(0.85, 0.4333333, 0.7833334),
                rgb!(0.4705882, 0.8, 0.4078431),
                rgb!(0.9921569, 0.0, 0.7719755),
                rgb!(0.706, 0.649, 0.2394706),
                rgb!(0.894, 0.1625455, 0.7485644),
                rgb!(0.754717, 0.3610244, 0.22071921),
                rgb!(0.0, 0.7058824, 1.0)
            ),
            Environment::Weave => color_scheme!(
                "Weave",
                rgb!(0.7843137, 0.07843138, 0.07843138),
                rgb!(0.1568627, 0.5568627, 0.8235294),
                rgb!(1.0, 0.1882353, 0.1882353),
                rgb!(0.85, 0.08499997, 0.08499997),
                rgb!(0.1882353, 0.675294, 1.0),
                rgb!(0.8218409, 0.08627451, 0.8509804),
                rgb!(0.6320754, 0.6320754, 0.6320754)
            ),
            Environment::FallOutBoy => color_scheme!(
                "Pyro",
                rgb!(0.5764706, 0.0, 0.03921569),
                rgb!(1.0, 0.6705883, 0.0),
                rgb!(0.8490566, 0.7037643, 0.4285333),
                rgb!(1.0, 0.1098039, 0.2039216),
                rgb!(0.8862745, 0.7372549, 0.2627451),
                rgb!(1.0, 0.0, 0.1764706),
                rgb!(0.7647059, 0.7647059, 0.7647059)
            ),
            Environment::EDM => color_scheme!(
                "EDM",
                rgb!(0.6320754, 0.6320754, 0.6320754),
                rgb!(0.1764706, 0.6980392, 0.8784314),
                rgb!(0.1764706, 0.6980392, 0.8784314),
                rgb!(0.08220173, 0.7169812, 0.0),
                rgb!(0.0, 0.3671638, 0.7169812),
                rgb!(0.735849, 0.0, 0.1758632),
                rgb!(0.4284593, 0.0, 0.754717)
            ),
            Environment::TheSecond => color_scheme!(
                "The Second",
                rgb!(0.7843137, 0.07843138, 0.07843138),
                rgb!(0.1568627, 0.5568627, 0.8235294),
                rgb!(1.0, 0.1882353, 0.1882353),
                rgb!(0.85, 0.08499997, 0.08499997),
                rgb!(0.1882353, 0.675294, 1.0),
                rgb!(0.8235294, 0.08627451, 0.8509804),
                rgb!(0.0, 1.0, 0.6478302)
            ),
            Environment::Lizzo => color_scheme!(
                "Lizzo",
                rgb!(1.0, 0.8132076, 0.3773585),
                rgb!(0.6705883, 0.254902, 0.8980392),
                rgb!(1.0, 0.5020987, 0.1882353),
                rgb!(0.8392157, 0.6470588, 0.2156863),
                rgb!(0.8196079, 0.2392157, 0.8784314),
                rgb!(1.0, 0.4, 0.5529412),
                rgb!(0.3686275, 0.7960784, 1.0)
            ),
            Environment::TheWeeknd => color_scheme!(
                "The Weeknd",
                rgb!(0.5843138, 0.1294118, 0.1294118),
                rgb!(0.2235294, 0.2901961, 0.3294118),
                rgb!(0.9176471, 0.2980392, 0.007843138),
                rgb!(1.0, 0.2979701, 0.1411765),
                rgb!(0.1668743, 0.3753689, 0.7075472),
                rgb!(0.9568628, 0.6039216, 0.1215686),
                rgb!(0.5254902, 0.8274511, 0.9921569)
            ),
            Environment::RockMixtape => color_scheme!(
                "Rock Mixtape",
                rgb!(0.6, 0.4233, 0.042),
                rgb!(0.6006, 0.7441199, 0.78),
                rgb!(1.0, 1.0, 1.0),
                rgb!(0.75, 0.12, 0.162),
                rgb!(0.95, 0.5820333, 0.1615),
                rgb!(0.96, 0.1344, 0.9187202),
                rgb!(0.378, 0.813, 0.9)
            ),
            Environment::Dragons2 => color_scheme!(
                "Dragons 2.0",
                rgb!(0.7264151, 0.6587077, 0.2809719),
                rgb!(0.2509804, 0.7647059, 0.405098),
                rgb!(0.5548979, 0.2470588, 1.0),
                rgb!(0.01960784, 0.9960785, 0.06666667),
                rgb!(0.0, 0.05490196, 1.0),
                rgb!(0.9764706, 0.03137255, 0.01960784),
                rgb!(1.0, 0.8292086, 0.2264151)
            ),
            Environment::PanicAtTheDisco2 => color_scheme!(
                "Panic 2.0",
                rgb!(0.9019608, 0.3333333, 0.5686275),
                rgb!(0.1529412, 0.5568628, 0.4862745),
                rgb!(0.9686275, 0.3803922, 0.2745098),
                rgb!(0.6980392, 0.1137255, 0.372549),
                rgb!(0.1882353, 0.6196079, 0.6235294),
                rgb!(0.9019608, 0.4470589, 0.06666667),
                rgb!(0.6365692, 0.4373443, 0.8584906)
            ),
            Environment::Queen => color_scheme!(
                "Queen",
                rgb!(0.58, 0.5675714, 0.5551428),
                rgb!(0.5236231, 0.1345675, 0.6792453),
                rgb!(0.9333334, 0.6392157, 0.1215686),
                rgb!(0.9333334, 0.6392157, 0.1215686),
                rgb!(0.04313726, 0.7176471, 0.8980393),
                rgb!(0.7686275, 0.145098, 0.07450981),
                rgb!(0.4, 0.007843138, 0.7254902)
            ),
            Environment::LinkinPark2 => color_scheme!(
                "Linkin Park 2.0",
                rgb!(0.6627451, 0.1643608, 0.1690187),
                rgb!(0.3870196, 0.5168997, 0.5568628),
                rgb!(0.6627451, 0.1647059, 0.172549),
                rgb!(0.6627451, 0.1647059, 0.172549),
                rgb!(0.6235294, 0.6901961, 0.7098039),
                rgb!(0.922, 0.5957885, 0.255394),
                rgb!(0.282353, 0.4586275, 0.6235294)
            ),
            Environment::TheRollingStones => color_scheme!(
                "The Rolling Stones",
                rgb!(0.8980392, 0.0, 0.1150319),
                rgb!(0.5254902, 0.1333333, 0.6784314),
                rgb!(0.9529412, 0.01176471, 0.4039216),
                rgb!(0.9529412, 0.01176471, 0.4039216),
                rgb!(0.4784314, 0.4039216, 1.0),
                rgb!(0.5647059, 0.4622677, 0.0),
                rgb!(0.003921554, 0.6383545, 0.6705883)
            ),
            Environment::Lattice => color_scheme!(
                "Lattice",
                rgb!(0.8392157, 0.172549, 0.5456773),
                rgb!(0.0, 0.6717121, 0.9803922),
                rgb!(0.4685534, 0.7095922, 1.0),
                rgb!(0.8941177, 0.1607843, 0.7490196),
                rgb!(0.1960784, 0.5843138, 0.7960785),
                rgb!(0.5450981, 0.1333333, 0.8156863),
                rgb!(0.4039216, 0.9176471, 0.9176471)
            ),
            Environment::DaftPunk => color_scheme!(
                "Daft Punk",
                rgb!(0.7215686, 0.2254902, 0.1803922),
                rgb!(0.1215686, 0.6980392, 0.6901961),
                rgb!(0.6068091, 0.0, 1.0),
                rgb!(1.0, 0.7017543, 0.2515723),
                rgb!(0.5215687, 0.3294118, 0.8196079),
                rgb!(0.8588235, 0.0, 0.4784314),
                rgb!(0.0, 0.8196079, 0.8039216)
            ),
            Environment::HipHop => color_scheme!(
                "Hip Hop Mixtape",
                rgb!(1.0, 0.583857, 0.3137255),
                rgb!(0.01542656, 0.6132076, 0.5896002),
                rgb!(1.0, 0.3137255, 0.5529412),
                rgb!(0.9137256, 0.4941177, 0.0),
                rgb!(0.05882353, 0.8039216, 0.1843137),
                rgb!(0.1411765, 1.0, 0.9686275),
                rgb!(0.227451, 0.2745098, 1.0)
            ),
            Environment::Collider => color_scheme!(
                "Collider",
                rgb!(0.9647059, 0.4947137, 0.1504941),
                rgb!(0.1686274, 0.5998134, 0.8588235),
                rgb!(0.8396226, 0.09639232, 0.0),
                rgb!(0.9637059, 0.4092787, 0.0),
                rgb!(0.1686275, 0.3921569, 0.8588236),
                rgb!(0.8980393, 0.03529412, 0.02352941),
                rgb!(0.854902, 0.4117647, 0.9725491)
            ),
            Environment::BritneySpears => color_scheme!(
                "Britney Spears",
                rgb!(0.9137255, 0.1411765, 0.6008013),
                rgb!(0.2484275, 0.578789, 1.0),
                rgb!(0.8396226, 0.09639232, 0.0),
                rgb!(0.9921569, 0.01176471, 0.9882353),
                rgb!(0.1921569, 0.5058824, 0.9058824),
                rgb!(1.0, 0.4591194, 0.5087922),
                rgb!(0.482353, 0.9294118, 0.7960785)
            ),
            Environment::Monstercat2 => color_scheme!(
                "Monstercat 2.0",
                rgb!(0.8745099, 0.3450981, 0.5215687),
                rgb!(0.3882353, 0.3019608, 0.6117647),
                rgb!(0.2313726, 0.1490196, 0.6392157),
                rgb!(0.6196079, 0.0509804, 0.8274511),
                rgb!(0.3372549, 0.7137255, 0.1098039),
                rgb!(0.7137255, 0.1098039, 0.1098039),
                rgb!(0.08627451, 0.5490196, 0.6470588)
            ),
            Environment::Metallica => color_scheme!(
                "Metallica",
                rgb!(0.282353, 0.3333333, 0.4039216),
                rgb!(0.5764706, 0.7176471, 0.8235294),
                rgb!(0.8392157, 0.09803922, 0.0),
                rgb!(0.8666667, 0.4941176, 0.3803922),
                rgb!(0.254902, 0.454902, 0.8666667),
                rgb!(0.8588235, 0.3921569, 0.09803922),
                rgb!(0.0, 0.7490196, 0.6313726)
            ),
        }
    }
}

impl AllDirectionEnvironment {
    /// Value taken from [the wiki](https://bsmg.wiki/mapping/lighting-defaults.html#current-colors).
    ///
    /// ChatGPT was used to help translate between formats, so there could be hallucinations.
    pub fn get_color_scheme(&self) -> ColorScheme {
        match self {
            AllDirectionEnvironment::GlassDesert => color_scheme!(
                "Origins",
                rgb!(0.6792453, 0.5712628, 0.0),
                rgb!(0.7075472, 0.0, 0.5364411),
                rgb!(0.06167676, 0.2869513, 0.3962264),
                rgb!(0.32222217, 0.6111111, 0.75),
                rgb!(0.03844783, 0.62239975, 0.90566039)
            ),
            AllDirectionEnvironment::Unknown(_) => ColorScheme::default(),
        }
    }
}
