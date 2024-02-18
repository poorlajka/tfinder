use crate::command::Command;
use crate::finder;
use std::path::Path;
use crate::Rect;
use crate::PathBuf;

pub struct Prompt {
    pub is_active: bool,
    pub command: Command,
    pub input: String,
    pub rect: Rect,
    pub root: PathBuf,
    pub tick: i32,
}

impl Prompt {
    pub fn get_prompt_string(&self) -> String {
        return self.command.get_prompt_string(&self.root);
    }
    pub fn begin_prompt(&mut self, command: Command) {
        self.is_active = true;
        self.command = command;
        self.input.clear();
    }

    pub fn enter_input(&mut self, input: char) {
        self.input.push(input);
    }

    pub fn delete_input(&mut self) {
        self.input.pop();
    }
    pub fn cancel(&mut self) {
        self.is_active = false;
        self.input.clear();
        self.command = Command::None;
    }

    pub fn is_active(&self) -> bool{
        return self.is_active;
    }

    pub fn run_command(&mut self, root: &Path) {
        match self.command {
            Command::Create => {
                let _ = finder::create_file(root, self.input.clone());
            }
            _ => {
            }
        }
        self.input.clear();
        self.is_active = false;
    }
}
