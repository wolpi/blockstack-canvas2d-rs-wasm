pub struct Colours {
    pub colour_1: &'static str,
    pub colour_2: &'static str,
    pub colour_bg: &'static str,
}

pub fn colours_for_level(level: i32) -> &'static Colours {
    &COLOURS[level as usize % COLOURS.len() - 1]
}

pub const COLOURS: [Colours; 10] = [
    Colours {
        colour_1: "#00F",
        colour_2: "#009",
        colour_bg: "#00F",
    },
    Colours {
        colour_1: "#F00",
        colour_2: "#900",
        colour_bg: "#F00",
    },
    Colours {
        colour_1: "#0F0",
        colour_2: "#090",
        colour_bg: "#0F0",
    },
    Colours {
        colour_1: "#F0F",
        colour_2: "#909",
        colour_bg: "#F0F",
    },
    Colours {
        colour_1: "#FF0",
        colour_2: "#990",
        colour_bg: "#FF0",
    },
    Colours {
        colour_1: "#0FF",
        colour_2: "#099",
        colour_bg: "#0FF",
    },
    Colours {
        colour_1: "#009",
        colour_2: "#003",
        colour_bg: "#009",
    },
    Colours {
        colour_1: "#900",
        colour_2: "#300",
        colour_bg: "#900",
    },
    Colours {
        colour_1: "#090",
        colour_2: "#030",
        colour_bg: "#090",
    },
    Colours {
        colour_1: "#999",
        colour_2: "#333",
        colour_bg: "#999",
    },
];
