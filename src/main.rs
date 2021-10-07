use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::{env, fs, process};

use font::FontType::OpenType;
use xml::reader::XmlEvent;
use xml::EventReader;

use crate::config::Config;
use crate::font_info::FontInfo;

mod config;
mod font_info;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Could not parse arguments: {}", err);
        process::exit(1);
    });

    let mut fonts = find_font_infos(&config.entitlements_file_path);
    let font_files: Vec<PathBuf> = find_available_font_files(&config.font_root_folder_path);

    // Create font path lookup
    let mut font_path_lookup = HashMap::new();
    for path in font_files {
        font_path_lookup.insert(
            path.file_name().unwrap().to_str().unwrap().to_string(),
            path,
        );
    }

    // Assign font file paths to each font
    for mut font in fonts.iter_mut() {
        font.file_path = font_path_lookup.remove(&font.id);
    }

    // Create target folder
    let target_dir = Path::new(&config.target_folder_path);
    fs::create_dir_all(target_dir).unwrap();

    // Parse each font and copy it over to the target directory
    let mut copied_fonts_counter = 0;
    for font in &fonts {
        if let Some(file_path) = &font.file_path {
            let mut font_file = File::open(file_path.as_path()).unwrap();

            // Read font info to buffer
            let mut buffer: Vec<u8> = Vec::new();
            font_file.read_to_end(&mut buffer).unwrap();
            let info = font::font_info(&buffer).unwrap();

            let font_name = String::clone(info.name.postscript_name.as_ref().unwrap());
            let family_name = String::clone(info.name.family.as_ref().unwrap());

            let target_name = match info.typ {
                OpenType => format!("{}.otf", font_name),
                _ => panic!("Font type is not supported yet"),
            };

            let target_file_path = target_dir.join(family_name).join(target_name);
            fs::create_dir_all(target_file_path.parent().unwrap()).unwrap();
            fs::copy(file_path.as_path(), target_file_path).expect("Could not copy font file");

            copied_fonts_counter += 1;
        }
    }

    println!(
        "Successfully copied {} fonts to `{}`",
        copied_fonts_counter, config.target_folder_path
    );

    Ok(())
}

fn find_available_font_files(root_folder_path: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();

    find_available_font_files_in_folder(Path::new(root_folder_path), &mut result);

    result
}

fn find_available_font_files_in_folder(folder: &Path, result: &mut Vec<PathBuf>) {
    if folder.is_dir() {
        for entry in fs::read_dir(folder).expect("Could not read folder") {
            let path = entry.unwrap().path();

            if path.is_dir() {
                find_available_font_files_in_folder(&path, result);
            } else {
                // Font file must not have extension and file name must be an integer
                let has_no_extension = path.extension() == None;
                let is_integer = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .is_ok();

                if has_no_extension && is_integer {
                    result.push(path);
                }
            }
        }
    }
}

fn find_font_infos(entitlements_xml_path: &str) -> Vec<FontInfo> {
    let file = File::open(entitlements_xml_path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut font_details: HashMap<String, String> = HashMap::new();
    let mut cur_key: Option<String> = None;

    let mut fonts: Vec<FontInfo> = Vec::new();

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => match name.local_name.as_ref() {
                "id" | "fullName" | "familyName" | "variationName" => {
                    cur_key = Some(name.local_name)
                }
                _ => cur_key = None,
            },
            Ok(XmlEvent::EndElement { name }) => {
                if let "font" = name.local_name.as_ref() {
                    let font = FontInfo::new(
                        font_details
                            .get("id")
                            .expect("Font needs an ID")
                            .to_string(),
                        font_details
                            .get("familyName")
                            .expect("Font needs a family name")
                            .to_string(),
                        font_details
                            .get("fullName")
                            .expect("Font needs a name")
                            .to_string(),
                        font_details
                            .get("variationName")
                            .expect("Font needs a variation name")
                            .to_string(),
                    );

                    fonts.push(font);
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                if let Some(key) = &cur_key {
                    font_details.insert(key.to_string(), s);
                }
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
            _ => {}
        }
    }

    fonts
}
