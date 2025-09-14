use crate::text_buffer::TextBuffer;

impl TextBuffer {
    pub fn new_line(&mut self) {
        let binding = self.raw_lines[self.cursor.1].clone();
        let (line, next_line) = binding.split_at(self.cursor.0);
        self.raw_lines[self.cursor.1] = next_line.to_string();
        let (start_buffer, end_buffer) = self.raw_lines.split_at(self.cursor.1);
        self.raw_lines = [start_buffer, &[line.to_string()], end_buffer].concat();
        self.cursor = (0, self.cursor.1 + 1);
    }
    pub fn edit(&mut self, letter: String) -> bool {
        if self.aimed_scroll != 0 {
            return false;
        }
        self.raw_lines
            .get_mut(self.cursor.1)
            .and_then(|line| {
                if line.len() <= self.cursor.0 {
                    line.push_str(&" ".repeat(self.cursor.0 - line.len()));
                }
                line.split_at_checked(self.cursor.0)
                    .map(|(start, end)| (start.to_string(), end.to_string()))
                    .and_then(|(start, end)| Some((line, start, end)))
            })
            .and_then(|(line, start, end)| Some(*line = [start, letter, end].concat()))
            .is_some()
    }
    pub fn delete(&mut self) {
        if self.cursor.0 == 0 && self.cursor.1 != 0 && self.cursor.1 < self.raw_lines.len() {
            let add = self.raw_lines.remove(self.cursor.1);
            let len = self.raw_lines[self.cursor.1 - 1].len();
            self.raw_lines[self.cursor.1 - 1].push_str(&add);
            self.cursor.1 -= 1;
            self.cursor.0 = len;
        } else {
            self.raw_lines
                .get_mut(self.cursor.1)
                .and_then(|line| line.split_at_checked(self.cursor.0))
                .map(|(start, end)| (start.to_string(), end.to_string()))
                .and_then(|(mut start, end)| {
                    start.pop();
                    self.left();
                    Some(self.raw_lines[self.cursor.1] = [start, end].concat())
                });
        }
    }
}
