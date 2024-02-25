use std::{cell::RefCell, fs, rc::Rc};

use ev3dev_lang_rust::{Button, Ev3Result, Screen};
use image::Rgb;
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

lazy_static! {
    static ref FONT: Font<'static> =
        Font::try_from_vec(fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf").unwrap()).unwrap();
}

pub struct Menu {
    entries: Vec<(String, Box::<dyn Fn()>)>,
    index: i32,
    screen: Screen,
}

impl Menu {
    pub fn new() -> Ev3Result<Self> {
        let screen = Screen::new()?;

        Ok(Self {
            entries: Vec::new(),
            index: 0,
            screen,
        })
    }

    pub fn add(&mut self, name: &str, handler: impl Fn() + 'static) {
        self.entries.push((name.to_string(), Box::new(handler)));
    }

    pub fn previous(&mut self) {
        let mut index = self.index - 1;
        if index < 0 {
            index = self.entries.len() as i32 - 1;
        }
        self.index = index;

        self.update();
    }

    pub fn next(&mut self) {
        let mut index = self.index + 1;
        if index >= self.entries.len() as i32 {
            index = 0;
        }
        self.index = index;

        self.update();
    }

    pub fn enter(&self) {
        self.entries[self.index as usize].1();
    }

    pub fn update(&mut self) {
        let screen_width = self.screen.xres();

        for (index, (entry, _)) in self.entries.iter().enumerate() {
            let mut background = Rgb([255, 255, 255]);
            let mut foreground = Rgb([0, 0, 0]);

            if index as i32 == self.index {
                std::mem::swap(&mut background, &mut foreground);
            }

            imageproc::drawing::draw_filled_rect_mut(
                &mut self.screen.image,
                Rect::at(0, 10 + (index as i32 * 20)).of_size(screen_width, 20),
                background,
            );
            imageproc::drawing::draw_text_mut(
                &mut self.screen.image,
                foreground,
                10,
                12 + (index as i32 * 20),
                Scale::uniform(16.0),
                &FONT,
                entry,
            );
        }

        self.screen.update();
    }

    pub fn run(self) -> Ev3Result<()> {
        let mut button = Button::new()?;

        let entries = Rc::new(RefCell::new(self));

        let up_entries = entries.clone();
        button.set_up_handler(move |is_pressed| {
            if is_pressed {
                up_entries.borrow_mut().previous();
            }
        });

        let down_entries = entries.clone();
        button.set_down_handler(move |is_pressed| {
            if is_pressed {
                down_entries.borrow_mut().next();
            }
        });

        let enter_entries = entries.clone();
        button.set_enter_handler(move |is_pressed| {
            if is_pressed {
                enter_entries.borrow().enter();
            }
        });

        entries.borrow_mut().update();

        loop {
            button.process();
        }
    }
}
