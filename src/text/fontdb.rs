
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::convert::TryFrom;

use fnv::FnvHashMap;
use ttf_parser as ttf;

use crate::ErrorKind;

use super::{
    Font,
    Weight,
    FontStyle,
    TextStyle,
    WidthClass,
    freetype as ft,
};

// TODO: use generational arena for font_ids

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FontId(usize);

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FontDescription {
    family_name: String,
    weight: Weight,
    font_style: FontStyle,
    width_class: WidthClass,
}

impl FontDescription {
    fn degrade(&mut self) -> bool {
        if !self.family_name.is_empty() {
            // if family_name is "Roboto Regular" or "Roboto-Regular" try to find font by only "Roboto"
            let split: Vec<&str> = self.family_name.split(|c| c == ' ' || c == '-').collect();

            if split.len() > 1 {
                self.family_name = split[0].to_owned();
            } else {
                self.family_name.clear();
            }

            true
        } else if self.weight != Weight::Normal {
            self.weight = Weight::Normal;
            true
        } else if self.width_class != WidthClass::Normal {
            self.width_class = WidthClass::Normal;
            true
        } else if self.font_style != FontStyle::Normal {
            self.font_style = FontStyle::Normal;
            true
        } else {
            false
        }
    }
}

impl From<&TextStyle<'_>> for FontDescription {
    fn from(style: &TextStyle) -> Self {
        Self {
            family_name: style.family_name.to_owned(),
            weight: style.weight,
            font_style: style.font_style,
            width_class: style.width_class
        }
    }
}

impl TryFrom<ttf::Font<'_>> for FontDescription {
    type Error = ErrorKind;

    fn try_from(font: ttf::Font<'_>) -> Result<Self, ErrorKind> {
        let family_name = font.family_name().ok_or(ErrorKind::FontInfoExtracionError)?;
        let weight = Weight::from_value(font.weight().to_number());
        let width_class = WidthClass::from_value(font.width().to_number());

        let font_style = if font.is_oblique() {
            FontStyle::Oblique
        } else if font.is_italic() {
            FontStyle::Italic
        } else {
            FontStyle::Normal
        };

        Ok(Self {
            family_name,
            weight,
            font_style,
            width_class,
        })
    }
}

pub struct FontDb {
    pub library: ft::Library,
    fonts: Vec<Font>,
    font_descr: FnvHashMap<FontDescription, FontId>
}

impl FontDb {

    pub fn new() -> Result<Self, ErrorKind> {
        Ok(Self {
            library: ft::Library::init()?,
            fonts: Default::default(),
            font_descr: Default::default(),
        })
    }

    pub fn scan_dir<T: AsRef<Path>>(&mut self, path: T) -> Result<(), ErrorKind> {
        let path = path.as_ref();

        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    self.scan_dir(&path)?;
                } else {
                    if let Some("ttf") = path.extension().and_then(OsStr::to_str) {
                        self.add_font_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn add_font_file<T: AsRef<Path>>(&mut self, path: T) -> Result<FontId, ErrorKind> {
        let data = std::fs::read(path)?;

        self.add_font_mem(data)
    }

    pub fn add_font_mem(&mut self, data: Vec<u8>) -> Result<FontId, ErrorKind> {
        let font = ttf::Font::from_data(&data, 0)?;
        let description = FontDescription::try_from(font)?;

        if let Some(id) = self.font_descr.get(&description).copied() {
            Ok(id)
        } else {
            let face = self.library.new_memory_face(data, 0)?;

            let id = FontId(self.fonts.len());
            self.fonts.push(Font::new(id, face));
            self.font_descr.insert(description, id);
            Ok(id)
        }
    }

    pub fn get(&self, id: FontId) -> Option<&Font> {
        self.fonts.get(id.0)
    }

    pub fn get_mut(&mut self, id: FontId) -> Option<&mut Font> {
        self.fonts.get_mut(id.0)
    }

    pub fn find_font<F, T>(&mut self, text: &str, style: &TextStyle, callback: F) -> Result<T, ErrorKind> where F: Fn(&mut Font) -> (bool, T) {
        let mut description = FontDescription::from(style);

        loop {
            if let Some(font_id) = self.font_descr.get(&description) {
                let font = self.fonts.get_mut(font_id.0).ok_or(ErrorKind::NoFontFound)?;

                let (has_missing, result) = callback(font);

                if !has_missing || !description.degrade() {
                    return Ok(result);
                }
            } else if !description.degrade() {
                // cant degrade description any more
                break;
            }
        }

        // try every font
        for font in &mut self.fonts {
            if font.has_chars(text) {
                let (_has_missing, result) = callback(font);
                return Ok(result);
            }
        }

        // just return the first font at this point and let it render .nodef glyphs
        if let Some(font) = self.fonts.first_mut() {
            return Ok(callback(font).1);
        }

        Err(ErrorKind::NoFontFound)
    }

}
