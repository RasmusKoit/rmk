use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;
use futures::StreamExt as _;
use memory_lcd_spi::{displays::LPM009M360A, pixelcolor::Rgb111, MemoryLCD};

use crate::channel::CONTROLLER_CHANNEL;
use crate::via::keycode_convert::to_ascii;
use crate::{channel::ControllerSub, controller::Controller, event::ControllerEvent};
use embedded_graphics::mono_font::ascii::FONT_7X14_BOLD;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;

pub struct LpmDisplayController<'a, SPI: SpiBus, CS> {
    display: MemoryLCD<LPM009M360A<Rgb111>, SPI, CS>,
    sub: ControllerSub<'a>,
}

impl<'a, SPI: SpiBus, CS: OutputPin> LpmDisplayController<'a, SPI, CS> {
    pub fn new(display: MemoryLCD<LPM009M360A<Rgb111>, SPI, CS>) -> Self {
        let sub = CONTROLLER_CHANNEL
            .subscriber()
            .expect("Failed to create subscriber for controller channel");
        Self { display, sub }
    }

    pub async fn run(&mut self) {
        loop {
            if let Some(event) = self.sub.next().await {
                self.process_event(event).await;
            }
        }
    }
}

impl<'a, SPI: SpiBus, CS: OutputPin> Controller for LpmDisplayController<'a, SPI, CS> {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::Battery(battery) => {
                Text::new(
                    "Pressed KEY:",
                    Point::new(4, 62),
                    MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::RED),
                )
                .draw(&mut *self.display)
                .unwrap();
                Text::new(
                    "Battery level:",
                    Point::new(4, 102),
                    MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::RED),
                )
                .draw(&mut *self.display)
                .unwrap();
                Text::new(
                    core::str::from_utf8(&[battery]).unwrap(),
                    Point::new(4, 122),
                    MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::RED),
                )
                .draw(&mut *self.display)
                .unwrap();
            }
            ControllerEvent::Key(event, action) => {
                if event.pressed {
                    match action {
                        crate::action::KeyAction::Single(Action::Key(key)) => {
                            // Convert key to &str
                            let str = core::str::from_utf8(&[to_ascii(key, false)]).unwrap();
                            // Draw the string
                            Text::new(str, Point::new(4, 82), MonoTextStyle::new(&FONT_7X14_BOLD, Rgb111::RED))
                                .draw(&mut *self.display)
                                .unwrap();
                        }
                        _ => (),
                    }
                }
            }
            _ => (), // TODO
        }
        self.display.update(&mut embassy_time::Delay).unwrap();
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next().await.unwrap()
    }
}
