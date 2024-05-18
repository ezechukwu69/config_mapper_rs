use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    pub item: Vec<Config>
}


#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    // The name is required
    pub name: String,
    // This is where the file is in the current directory
    pub target: String,
    // The symlink is required, this is where the file will be created for the system
    pub external: String,
    // pub mode: Option<String>
    pub repo: Option<String>
}

