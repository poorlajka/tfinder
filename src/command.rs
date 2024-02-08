
pub enum Command {
    Create,
    Rename,
    Move,
    Delete,
    Open,
    Help,
    Search,
    Fill,
    None,
}

impl Command {
    pub fn get_prompt_string(&self) -> String {
        match self {
            Self::Create => {
                String::from("Name of file to create: ")
            }
            _ => {
                String::from("")
            }
        }
    }
}
