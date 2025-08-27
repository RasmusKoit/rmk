use embedded_hal::digital::StatefulOutputPin;

use crate::channel::{CONTROLLER_CHANNEL, ControllerSub};
use crate::controller::{Controller, PollingController};
use crate::driver::gpio::OutputController;
use crate::event::ControllerEvent;

impl<P: StatefulOutputPin> Controller for BacklightController<P> {
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::BacklightToggle(on) => {
                if on {
                    self.state.backlight_en = true;
                } else {
                    self.state.backlight_en = false;
                }
                let _ = self.pin.set_state(self.state.backlight_en);
            }
            _ => (),
        }
    }
}
