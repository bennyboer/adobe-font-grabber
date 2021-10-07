pub struct Config {
    /// Folder to search for font files in
    pub font_root_folder_path: String,

    /// Path to the entitlements.xml file
    pub entitlements_file_path: String,

    /// Folder to save found Adobe fonts to
    pub target_folder_path: String,
}

impl Config {
    pub(crate) fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 4 {
            return Err("Config needs exactly 3 arguments");
        }

        let font_root_folder_path = args[1].clone();
        let entitlements_file_path = args[2].clone();
        let target_folder_path = args[3].clone();

        Ok(Config {
            font_root_folder_path,
            entitlements_file_path,
            target_folder_path,
        })
    }
}
