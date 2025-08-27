use embedded_hal::digital::StatefulOutputPin;

use crate::channel::{CONTROLLER_CHANNEL, ControllerSub};
use crate::controller::{Controller, PollingController};
use crate::driver::gpio::OutputController;
use crate::event::ControllerEvent;
