use crate::PathBuf;
use crate::MouseEvent;
use crate::Rect;

pub struct PathBreadcrumbs {
    pub paths: Vec<(String, PathBuf)>,
    pub hovered_path: Option<usize>,
}

struct Interval {
    begin: u16,
    end: u16,
}

impl PathBreadcrumbs {

    pub fn new(path: &PathBuf) -> Self {
        let mut path_breadcrumbs = PathBreadcrumbs {
            paths: Vec::new(),
            hovered_path: None,
        };
        path_breadcrumbs.load_path(path);

        path_breadcrumbs
    }

    pub fn get_hovered_index(&self, event: MouseEvent) -> Option<usize> {
        let mut interval = Interval { begin: 0, end: 0 };

        for (i, (name, _)) in self.paths.iter().enumerate() {
            interval.begin = interval.end;
            interval.end = interval.end + name.len() as u16 + 3;

            if event.column >= interval.begin && event.column <= interval.end - 4 {
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
