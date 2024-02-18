use crate::Path;

pub enum Command {
    Create,
    _Rename,
    _Move,
    _Delete,
    _Open,
    _Help,
    _Search,
    _Fill,
    None,
}

impl Command {
    pub fn get_prompt_string(&self, _path: &Path) -> String {
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
