use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallConfig {
    information: Information,
    install: Vec<PackageGroup>,
    caveats: Vec<Caveat>,
    commands: Vec<CommandGroup>,
    dotfiles: Vec<SymlinkConfig>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    name: String,
    author: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageGroup {
    category: String,
    ingredients: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Caveat {
    title: String,
    content: I18nText
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandGroup {
    description: String,
    execute: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymlinkConfig {
    name: String,
    from: String,
    to: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct I18nText {
    ja: String,
    en: String
}
