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
}

impl Document {
    /// # Errors
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
        })
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
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
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
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
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
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        }
    }

    /// # Errors
    ///
    /// This function will return the first error of Error kind that [write] returns.
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
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
