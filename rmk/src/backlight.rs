use bitfield_struct::bitfield;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BacklightMode {
    CONSTANT,
    BREATHING,
    RAINBOW,
    SCROLLING,
}

impl BacklightMode {
    pub const fn from_bits(bits: u8) -> Self {
        match bits & 0b11 {
            0b00 => BacklightMode::CONSTANT,
            0b01 => BacklightMode::BREATHING,
            0b10 => BacklightMode::RAINBOW,
            0b11 => BacklightMode::SCROLLING,
            _ => BacklightMode::CONSTANT, // Default case for const fn
        }
    }

    pub const fn into_bits(self) -> u8 {
        match self {
            BacklightMode::CONSTANT => 0b00,
            BacklightMode::BREATHING => 0b01,
            BacklightMode::RAINBOW => 0b10,
            BacklightMode::SCROLLING => 0b11,
        }
    }
}

#[bitfield(u8, defmt = cfg(feature = "defmt"))]
#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct BacklightIndicator {
    #[bits(1)]
    pub(crate) backlight_en: bool,
    #[bits(1)]
    pub(crate) pwm: bool,
    #[bits(2)]
    pub(crate) mode: u8, // Use u8 instead of BacklightMode
    #[bits(4)]
    pub(crate) brightness: u8,
}

impl BitOr for BacklightIndicator {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_bits(self.into_bits() | rhs.into_bits())
    }
}
impl BitAnd for BacklightIndicator {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_bits(self.into_bits() & rhs.into_bits())
    }
}
impl Not for BacklightIndicator {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::from_bits(!self.into_bits())
    }
}
impl BitAndAssign for BacklightIndicator {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}
impl BitOrAssign for BacklightIndicator {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BacklightIndicator {
    pub const BACKLIGHT_EN: Self = Self::new().with_backlight_en(true);
    pub const PWM: Self = Self::new().with_pwm(false);
    pub const BRIGHTNESS: Self = Self::new().with_brightness(0b1111);
    pub const MODE: Self = Self::new().with_mode(BacklightMode::CONSTANT.into_bits());

    pub const fn new_from(backlight_en: bool, pwm: bool, mode: BacklightMode, brightness: u8) -> Self {
        Self::new()
            .with_backlight_en(backlight_en)
            .with_pwm(pwm)
            .with_mode(mode.into_bits())
            .with_brightness(brightness & 0b1111)
    }
}
