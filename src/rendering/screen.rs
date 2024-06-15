use std::io::Write;
use std::io::stdout;

pub(super) struct ScreenSize(u8, u8);

const VOID_CHAR: char = ' ';

pub struct Screen {
    screen_chars: Vec<char>,
    screen_size: ScreenSize
}

impl Screen {
    pub fn new(w: u8, h: u8) -> Self {
        let size = (w as u16 * h as u16) as usize;

        Self {
            screen_size: ScreenSize(w, h),
            screen_chars: vec![VOID_CHAR; size]
        }
    }

    // draw the map on the left, on the right the player stats
    pub fn set_string_at(&mut self,x:u8,y:u8,str: String) {
        let index = self.get_index_in_vector(y, x);

        for (i, c) in str.chars().enumerate() {
            self.screen_chars[index + i] = c;
        }
    }

    fn get_index_in_vector(&self, y: u8, x: u8) -> usize {
        (y as i16 * self.screen_size.0 as i16 + x as i16) as usize
    }
    
    pub fn set_string_at_with_word_wrap(&mut self,x:u8,y:u8,str: String) {
        let mut index = self.get_index_in_vector(y, x);
        let mut current_index = 0;

        for (i, c) in str.chars().enumerate() {
            if c == VOID_CHAR {
                current_index = i;
            }

            self.screen_chars[index + i] = c;

            if i > self.screen_size.0 as usize {
                index = (y as i16 * self.screen_size.0 as i16 + current_index as i16) as usize;
            }
        }
    }

    pub fn set_char_at(&mut self,x:u8,y:u8,c: char) {
        let index = self.get_index_in_vector(y, x);
        self.screen_chars[index] = c;
    }

    pub fn draw_screen(&mut self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        let mut screen = String::new();

        for (i, c) in self.screen_chars.iter().enumerate() {
            screen.push(*c);

            if (i + 1) % self.screen_size.0 as usize == 0 {
                screen.push('\n');
            }
        }

        // clear 
        write!(stdout, "{esc}[2J{esc}[1;1H", esc = 27 as char)?;
        write!(stdout, "{}", screen)?;
        self.flush();
        Ok(())
    }

    pub fn flush(&mut self) {
        self.screen_chars.fill(VOID_CHAR);
    }
}