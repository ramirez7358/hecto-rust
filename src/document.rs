use crate::FileType;
use crate::SearchDirection;
use crate::Position;
use crate::Row;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    /// # Errors
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let file_type = FileType::from(filename);
        let mut rows = Vec::new();
        for value in contents.lines() {
            let mut row = Row::from(value);
            row.highlight(file_type.highlighting_options(), None);
            rows.push(row);
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
            file_type
        })
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y == self.rows.len() {
            return;
        }
        if at.y == self.rows.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let current_row = &mut self.rows[at.y];
        let mut new_row = current_row.split(at.x);
        current_row.highlight(self.file_type.highlighting_options(), None);
        new_row.highlight(self.file_type.highlighting_options(), None);
        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }

    /// # Panics
    ///
    /// Will panic if the new capacity exceeds `isize::MAX` bytes
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            row.highlight(self.file_type.highlighting_options(), None);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
            row.highlight(self.file_type.highlighting_options(), None);
        }
    }

    /// # Panics
    ///
    /// Will panic if the index in `row` is out of bounds.
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
            row.highlight(self.file_type.highlighting_options(), None);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
            row.highlight(self.file_type.highlighting_options(), None);
        }
    }

    /// # Errors
    ///
    /// This function will return the first error of Error kind that [write] returns.
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            self.file_type = FileType::from(file_name);
            for row in &mut self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
                row.highlight(self.file_type.highlighting_options(), None)
            }
            self.dirty = false;
        }
        Ok(())
    }
    
    pub fn highlight(&mut self, word: Option<&str>) {
        for row in &mut self.rows {
            row.highlight(self.file_type.highlighting_options(), word)
        }
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[allow(clippy::indexing_slicing)]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut position = Position { x: at.x, y: at.y };

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.rows.len()
        } else {
            at.y.saturating_add(1)
        };
        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }
        None
    }
}
