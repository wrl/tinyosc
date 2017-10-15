// Copyright (c) 2015 William Light <wrl@illest.net>
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::ascii::AsciiExt;
use regex;
use regex::Regex;
use Message;

// all printable ascii characters except #*,/?[]{} and space
const OSC_ADDR_SYMBOL: &'static str = r#"[A-Za-z0-9!"$%&'()+.0-9:;<=>@\\^_`|~-]"#;
const OSC_ADDR_FULL:   &'static str = r#"[A-Za-z0-9!"$%&'()+.0-9:;<=>@\\^_`|~/-]"#;

pub struct Pattern {
    pattern_re: Regex
}

impl Pattern {
    pub fn new(pattern: &str) -> Result<Pattern, ()> {
        let mut pattern_re = String::new();

        let chars = pattern.chars().collect::<Vec<char>>();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            assert!(c.is_ascii());

            match c {
                '/' => {
                    let j = i + 1;
                    if chars[j] == '/' {
                        pattern_re.push_str(OSC_ADDR_FULL);
                        pattern_re.push_str("*/");
                        i = j;
                    } else {
                        pattern_re.push_str(regex::escape(c.to_string().as_str()).as_str());
                    }
                }
                '?' => {
                    pattern_re.push_str(OSC_ADDR_SYMBOL);
                }
                '*' => {
                    pattern_re.push_str(OSC_ADDR_SYMBOL);
                    pattern_re.push('*');
                }
                '[' => {
                    let j = pattern.find(']').unwrap();

                    pattern_re.push_str("[");
                    let mut sub = &pattern[i+1..j];

                    if sub.starts_with("!") {
                        sub = &sub[1..];
                        pattern_re.push_str("^");
                    }

                    for (ci, c) in sub.char_indices() {
                        if c == '-' && ci != sub.len() {
                            pattern_re.push_str("-");
                        } else {
                            pattern_re.push_str(regex::escape(c.to_string().as_str()).as_str());
                        }
                    }

                    pattern_re.push_str("]");
                    i = j;
                }
                '{' => {
                    let j = pattern.find('}').unwrap();
                    pattern_re.push_str("(");

                    let sub = &pattern[i+1..j];
                    let s = sub.split(',').collect::<Vec<&str>>().join("|");
                    pattern_re.push_str(s.as_str());
                    
                    pattern_re.push_str(")");
                    i = j;
                }
                _ => {
                    pattern_re.push_str(regex::escape(c.to_string().as_str()).as_str());
                }
            }
            i += 1;
        }

        Ok(
        Pattern {
            pattern_re: Regex::new(pattern_re.as_str()).unwrap()
        }
        )
    }

    pub fn matches(&self, m: Message) -> bool {
        return self.pattern_re.is_match(m.path);
    }

    pub fn matches_path(&self, path: &str) -> bool {
        return self.pattern_re.is_match(path);
    }
}
