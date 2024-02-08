use crate::PathBuf;
use crate::MouseEvent;
use crate::Rect;

pub struct PathTrail {
    pub paths: Vec<(String, PathBuf)>,
    pub hovered_path: Option<usize>,
    pub rect: Rect,
}

struct Interval {
    begin: u16,
    end: u16,
}

impl PathTrail {

    pub fn is_mouse_on(&self, column: u16, row: u16) -> bool {
        let Rect {x, y, width, height} = self.rect;

        column >= x && column <= x + width
            && row >= y && row <= y + height
    }

    pub fn get_hovered_index(&self, event: MouseEvent) -> Option<usize> {
        let mut interval = Interval { begin: 0, end: 0 };

        for (i, (name, _)) in self.paths.iter().enumerate() {
            interval.begin = interval.end;
            interval.end = interval.end + name.len() as u16 + 3;

            if event.column >= interval.begin && event.column <= interval.end {
                return Some(i);
            }
        }
        None
    }

    pub fn load_path(&mut self, path: &PathBuf) {
        self.paths.clear();
        for ancestor in &mut path.ancestors() {
            self.paths.push((
                ancestor
                    .iter()
                    .last()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                ancestor.to_path_buf(),
            ));
        }
        self.paths.reverse();
    }
}
