#[derive(Debug)]
pub struct StringSplit<'a> {
    pub string: &'a str,
    pub left_delimiter: char,
    pub right_delimiter: char,
}

pub struct StringSplitIterator<'a> {
    string: &'a str,
    string_quote_type: u8,
    delimiters: &'a [char],
    string_quotes: &'a [char],
    pos: usize,
    last_delimiter: char,
}

impl<'a> StringSplitIterator<'a> {
    pub fn new(string: &'a str, delimiters: &'a [char], string_quotes: &'a [char]) -> StringSplitIterator<'a> {
        StringSplitIterator {
            string,
            string_quote_type: 0x00,
            delimiters,
            string_quotes,
            pos: 0,
            last_delimiter: '\0',
        }
    }
}

impl<'a> Iterator for StringSplitIterator<'a> {
    type Item = StringSplit<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.string.as_bytes();
        
        let get_current_char = |pos: usize| bytes[pos] as char;
        
        let from = self.pos;
        let to = {
            while {
                let is_end_reached = || self.pos >= self.string.len();
                let is_inside_string = || self.string_quote_type != 0x00;
                let has_found_delimiter = || self.delimiters.contains(&get_current_char(self.pos));
                !is_end_reached() && (is_inside_string() || !has_found_delimiter())
            } {
                let mut update_string_quote_type = || {
                    if self.string_quote_type == 0x00 {
                        if self.string_quotes.contains(&get_current_char(self.pos)) {
                            self.string_quote_type = bytes[self.pos];
                        }
                    } else {
                        if bytes[self.pos] == self.string_quote_type {
                            self.string_quote_type = 0x00;
                        }
                    }
                };
                update_string_quote_type();
                self.pos += 1;
            }
            self.pos
        };
        
        if self.pos > self.string.len() {
            return None;
        }
        
        let right_delimiter = if self.pos< self.string.len() {
            get_current_char(self.pos)
        } else {
            '\0'
        };
        let left_delimiter = self.last_delimiter;
        self.last_delimiter = right_delimiter;
        
        self.pos += 1;
        
        Some(StringSplit {
            string: &self.string[from..to].trim_matches(self.string_quotes),
            left_delimiter,
            right_delimiter,
        })
    }
}