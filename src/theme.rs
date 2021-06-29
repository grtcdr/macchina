use crate::data::ReadoutKey;
use clap::arg_enum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tui::style::Color;

/// Defines the different ways a key can be named, let's take the _OperatingSystem variant_ for example: \
/// - `AbbreviationType::Classic` -> OS \
/// - `AbbreviationType::Alternative` -> Ope \
/// - `AbbreviationType::Long` -> Operating System
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AbbreviationType {
    Classic,
    Alternative,
    Long,
}

/// This implements all the different ways a `Key` can be named using \
/// the predefined variants found in the `AbbreviationType` enum.
impl ReadoutKey {
    fn get_common_name(&self) -> HashMap<&AbbreviationType, &'static str> {
        let mut values = HashMap::new();

        match self {
            ReadoutKey::Host => {
                values.insert(&AbbreviationType::Classic, "Host");
                values.insert(&AbbreviationType::Alternative, "Hos");
                values.insert(&AbbreviationType::Long, "Host");
            }
            ReadoutKey::Machine => {
                values.insert(&AbbreviationType::Classic, "Machine");
                values.insert(&AbbreviationType::Alternative, "Mac");
                values.insert(&AbbreviationType::Long, "Machine");
            }
            ReadoutKey::Kernel => {
                values.insert(&AbbreviationType::Classic, "Kernel");
                values.insert(&AbbreviationType::Alternative, "Ker");
                values.insert(&AbbreviationType::Long, "Kernel");
            }
            ReadoutKey::Distribution => {
                values.insert(&AbbreviationType::Classic, "Distro");
                values.insert(&AbbreviationType::Alternative, "Dis");
                values.insert(&AbbreviationType::Long, "Distribution");
            }
            ReadoutKey::OperatingSystem => {
                values.insert(&AbbreviationType::Classic, "OS");
                values.insert(&AbbreviationType::Alternative, "Ope");
                values.insert(&AbbreviationType::Long, "Operating System");
            }
            ReadoutKey::DesktopEnvironment => {
                values.insert(&AbbreviationType::Classic, "DE");
                values.insert(&AbbreviationType::Alternative, "Des");
                values.insert(&AbbreviationType::Long, "Desktop Environment");
            }
            ReadoutKey::WindowManager => {
                values.insert(&AbbreviationType::Classic, "WM");
                values.insert(&AbbreviationType::Alternative, "Win");
                values.insert(&AbbreviationType::Long, "Window Manager");
            }
            ReadoutKey::Packages => {
                values.insert(&AbbreviationType::Classic, "Packages");
                values.insert(&AbbreviationType::Alternative, "Pac");
                values.insert(&AbbreviationType::Long, "Packages");
            }
            ReadoutKey::Shell => {
                values.insert(&AbbreviationType::Classic, "Shell");
                values.insert(&AbbreviationType::Alternative, "She");
                values.insert(&AbbreviationType::Long, "Shell");
            }
            ReadoutKey::Terminal => {
                values.insert(&AbbreviationType::Classic, "Terminal");
                values.insert(&AbbreviationType::Alternative, "Ter");
                values.insert(&AbbreviationType::Long, "Terminal");
            }
            ReadoutKey::Uptime => {
                values.insert(&AbbreviationType::Classic, "Uptime");
                values.insert(&AbbreviationType::Alternative, "Upt");
                values.insert(&AbbreviationType::Long, "Uptime");
            }
            ReadoutKey::Processor => {
                values.insert(&AbbreviationType::Classic, "CPU");
                values.insert(&AbbreviationType::Alternative, "Cpu");
                values.insert(&AbbreviationType::Long, "Processor");
            }
            ReadoutKey::ProcessorUsage => {
                values.insert(&AbbreviationType::Classic, "CPU%");
                values.insert(&AbbreviationType::Alternative, "Cp%");
                values.insert(&AbbreviationType::Long, "Processor Usage");
            }
            ReadoutKey::LocalIP => {
                values.insert(&AbbreviationType::Classic, "Local IP");
                values.insert(&AbbreviationType::Alternative, "Adr");
                values.insert(&AbbreviationType::Long, "Local IP");
            }
            ReadoutKey::Backlight => {
                values.insert(&AbbreviationType::Classic, "Brightness");
                values.insert(&AbbreviationType::Alternative, "Bri");
                values.insert(&AbbreviationType::Long, "Brightness");
            }
            ReadoutKey::Resolution => {
                values.insert(&AbbreviationType::Classic, "Resolution");
                values.insert(&AbbreviationType::Alternative, "Res");
                values.insert(&AbbreviationType::Long, "Resolution");
            }
            ReadoutKey::Memory => {
                values.insert(&AbbreviationType::Classic, "Memory");
                values.insert(&AbbreviationType::Alternative, "Mem");
                values.insert(&AbbreviationType::Long, "Memory");
            }
            ReadoutKey::Battery => {
                values.insert(&AbbreviationType::Classic, "Battery");
                values.insert(&AbbreviationType::Alternative, "Bat");
                values.insert(&AbbreviationType::Long, "Battery");
            }
        }

        values
    }
}

/// This struct stores the BarStyle to display when --bar or bar config option is used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarStyle {
    pub glyph: String,
    pub symbol_open: char,
    pub symbol_close: char,
}

/// This stores the predefined BarStyle's
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BarStyles {
    Squared,
    Rounded,
    Angled,
    Hidden,
    Custom(BarStyle),
}

impl BarStyle {
    fn new(style: BarStyles) -> Self {
        match style {
            BarStyles::Squared => BarStyle {
                glyph: "■".to_owned(),
                symbol_open: '[',
                symbol_close: ']',
            },
            BarStyles::Rounded => BarStyle {
                glyph: "●".to_owned(),
                symbol_open: '(',
                symbol_close: ')',
            },
            BarStyles::Angled => BarStyle {
                glyph: "×".to_owned(),
                symbol_open: '<',
                symbol_close: '>',
            },
            BarStyles::Hidden => BarStyle {
                glyph: "".to_owned(),
                symbol_open: '\0',
                symbol_close: '\0',
            },
            BarStyles::Custom(barstyle) => barstyle,
        }
    }

    pub fn hide_delimiters(&self) -> BarStyle {
        BarStyle {
            glyph: self.glyph.clone(),
            symbol_open: '\0',
            symbol_close: '\0',
        }
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum Themes {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    }
}

/// This is the struct which defines predefined as well as custom themes.
#[derive(Debug, Clone)]
pub struct Theme {
    bar: BarStyle,
    color: Color,
    separator_color: Color,
    separator: String,
    spacing: usize,
    padding: usize,
    block_title: String,
    abbreviation: AbbreviationType,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            bar: BarStyle::new(BarStyles::Rounded),
            color: Color::Red,
            separator_color: Color::White,
            separator: "-".to_owned(),
            spacing: 2,
            padding: 0,
            block_title: String::from(" Hydrogen "),
            abbreviation: AbbreviationType::Classic,
        }
    }
}

impl Theme {
    pub fn new(theme: Themes) -> Self {
        match theme {
            Themes::Hydrogen => Theme {
                bar: BarStyle::new(BarStyles::Rounded),
                color: Color::Red,
                separator_color: Color::White,
                separator: "-".to_owned(),
                spacing: 2,
                padding: 0,
                block_title: String::from(" Hydrogen "),
                abbreviation: AbbreviationType::Classic,
            },
            Themes::Helium => Theme {
                bar: BarStyle::new(BarStyles::Squared),
                color: Color::Green,
                separator_color: Color::White,
                separator: "=>".to_owned(),
                spacing: 2,
                padding: 0,
                block_title: String::from(" Helium "),
                abbreviation: AbbreviationType::Alternative,
            },
            Themes::Lithium => Theme {
                bar: BarStyle::new(BarStyles::Angled),
                color: Color::Magenta,
                separator_color: Color::White,
                separator: "~".to_owned(),
                spacing: 2,
                padding: 0,
                block_title: String::from(" Lithium "),
                abbreviation: AbbreviationType::Long,
            },
            Themes::Beryllium => Theme {
                bar: BarStyle::new(BarStyles::Rounded),
                color: Color::Yellow,
                separator_color: Color::White,
                separator: "->".to_owned(),
                spacing: 2,
                padding: 0,
                block_title: String::from(" Beryllium "),
                abbreviation: AbbreviationType::Alternative,
            },
            Themes::Boron => Theme {
                // will implement random emoji later
                bar: BarStyle::new(BarStyles::Rounded),
                color: Color::Blue,
                separator_color: Color::White,
                separator: "•".to_owned(),
                spacing: 2,
                padding: 0,
                block_title: String::from(" Boron "),
                abbreviation: AbbreviationType::Long,
            },
        }
    }
    pub fn get_bar_style(&self) -> &BarStyle {
        &self.bar
    }

    pub fn set_bar_style(&mut self, new_bar: BarStyle) {
        self.bar = new_bar
    }

    pub fn get_separator(&self) -> &str {
        &self.separator
    }

    pub fn set_separator(&mut self, separator: impl ToString) {
        self.separator = separator.to_string()
    }

    pub fn get_separator_color(&self) -> Color {
        self.separator_color
    }

    pub fn set_separator_color(&mut self, color: Color) {
        self.separator_color = color
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn get_padding(&self) -> usize {
        self.padding
    }

    pub fn set_padding(&mut self, size: usize) {
        self.padding = size
    }

    pub fn get_spacing(&self) -> usize {
        self.spacing
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing;
    }

    pub fn get_block_title(&self) -> &str {
        &self.block_title
    }

    pub fn set_block_title(&mut self, s: &str) {
        self.block_title = s.into()
    }

    pub fn key(&self, readout_key: &ReadoutKey, abbreviation: &AbbreviationType) -> &'static str {
        let abbreviated_names = readout_key.get_common_name();
        let name_entry = abbreviated_names.get(&abbreviation);

        if let Some(name) = name_entry {
            name
        } else {
            abbreviated_names.values().next().unwrap()
        }
    }

    pub fn default_abbreviation(&self) -> &AbbreviationType {
        &self.abbreviation
    }
}

impl From<CustomTheme> for Theme {
    fn from(custom: CustomTheme) -> Self {
        Self {
            bar: BarStyle::new(custom.bar),
            color: custom.color,
            separator: custom.separator,
            separator_color: custom.separator_color,
            spacing: custom.spacing,
            padding: custom.padding,
            block_title: custom.block_title,
            abbreviation: custom.abbreviation,
        }
    }
}

/// This is the struct which stores the CustomThemes which is serialized from a json file.
/// ## Example theme carbon
/// ```json
/// {
///   "name": "Carbon",
///   "bar": {
///     "Custom": {
///       "glyph": "ߋ",
///       "symbol_open": "[",
///       "symbol_close": "]"
///     }
///   },
///   "color": {
///     "Rgb": [
///       231,
///       198,
///       100
///     ]
///   },
///   "separator": "⇉",
///   "separator_color": {
///     "Rgb": [
///       158,
///       208,
///       114
///     ]
///   },
///   "spacing": 2,
///   "padding": 0,
///   "block_title": "┤ Carbon ├",
///   "abbreviation" : "Classic"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomTheme {
    bar: BarStyles,

    #[serde(with = "ColorDef")]
    color: Color,

    separator: String,
    #[serde(with = "ColorDef")]
    separator_color: Color,

    spacing: usize,
    padding: usize,
    block_title: String,
    abbreviation: AbbreviationType,
}
impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            bar: BarStyles::Squared,
            color: Color::Red,
            separator: "->".to_string(),
            separator_color: Color::White,
            spacing: 0,
            padding: 2,
            block_title: " Hydrogen ".to_string(),
            abbreviation: AbbreviationType::Classic,
        }
    }
}

impl CustomTheme {
    /// Get custom theme from ~/.local/share/macchina/themes/{name}.json
    /// Check the repo for example themes
    pub fn get_theme(name: &str) -> Result<Self, std::io::Error> {
        use std::io::Read;
        // check if the name exists in ~/.local/share/macchina/themes/{name}.json
        // need to add other data paths later ( /usr/share/macchina/themes/{name}.json )
        let mut theme_path = std::path::PathBuf::new();
        theme_path.push(dirs::data_local_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "data_local_dir (e.g. ~/.local/share) not found",
            )
        })?);
        theme_path.push(std::path::Path::new(&format!(
            "macchina/themes/{}.json",
            name
        )));

        let mut buffer: Vec<u8> = Vec::new();
        let mut theme = std::fs::File::open(theme_path)?;
        theme.read_to_end(&mut buffer)?;

        serde_json::from_slice(&buffer).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Unable to parse theme")
        })
    }

    // private function to print a custom theme for testing
    fn __print_theme_test() {
        let cust = CustomTheme {
            bar: BarStyles::Custom(BarStyle {
                glyph: "x".to_string(),
                symbol_open: '[',
                symbol_close: ']',
            }),
            separator: "=====>".to_string(),
            spacing: 10,
            padding: 10,
            block_title: "SomeTitle".to_string(),

            color: Color::Rgb(10, 33, 51),
            separator_color: Color::Indexed(100),
            abbreviation: AbbreviationType::Long,
        };
        println!("{}", serde_json::to_string_pretty(&cust).unwrap());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}
