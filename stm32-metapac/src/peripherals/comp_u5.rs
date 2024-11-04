#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp {
    ptr: *mut u8,
}
unsafe impl Send for Comp {}
unsafe impl Sync for Comp {}
impl Comp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Comparator control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Input minus selection bits."]
        #[inline(always)]
        pub const fn inmsel(&self) -> super::vals::Inm {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Inm::from_bits(val as u8)
        }
        #[doc = "Input minus selection bits."]
        #[inline(always)]
        pub fn set_inmsel(&mut self, val: super::vals::Inm) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "Input plus selection bit."]
        #[inline(always)]
        pub const fn inpsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Input plus selection bit."]
        #[inline(always)]
        pub fn set_inpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Comparator 1 noninverting input selector for window mode."]
        #[inline(always)]
        pub const fn winmode(&self) -> super::vals::WindowMode {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::WindowMode::from_bits(val as u8)
        }
        #[doc = "Comparator 1 noninverting input selector for window mode."]
        #[inline(always)]
        pub fn set_winmode(&mut self, val: super::vals::WindowMode) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Comparator 1 output selector."]
        #[inline(always)]
        pub const fn winout(&self) -> super::vals::WindowOut {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::WindowOut::from_bits(val as u8)
        }
        #[doc = "Comparator 1 output selector."]
        #[inline(always)]
        pub fn set_winout(&mut self, val: super::vals::WindowOut) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Polarity selection bit."]
        #[inline(always)]
        pub const fn polarity(&self) -> super::vals::Polarity {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Polarity::from_bits(val as u8)
        }
        #[doc = "Polarity selection bit."]
        #[inline(always)]
        pub fn set_polarity(&mut self, val: super::vals::Polarity) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Hysteresis selection bits."]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hysteresis {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Hysteresis::from_bits(val as u8)
        }
        #[doc = "Hysteresis selection bits."]
        #[inline(always)]
        pub fn set_hyst(&mut self, val: super::vals::Hysteresis) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Power Mode."]
        #[inline(always)]
        pub const fn pwrmode(&self) -> super::vals::PowerMode {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::PowerMode::from_bits(val as u8)
        }
        #[doc = "Power Mode."]
        #[inline(always)]
        pub fn set_pwrmode(&mut self, val: super::vals::PowerMode) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Blanking source selection bits."]
        #[inline(always)]
        pub const fn blanksel(&self) -> super::vals::Blanking {
            let val = (self.0 >> 20usize) & 0x1f;
            super::vals::Blanking::from_bits(val as u8)
        }
        #[doc = "Blanking source selection bits."]
        #[inline(always)]
        pub fn set_blanksel(&mut self, val: super::vals::Blanking) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val.to_bits() as u32) & 0x1f) << 20usize);
        }
        #[doc = "Output status bit."]
        #[inline(always)]
        pub const fn value(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Output status bit."]
        #[inline(always)]
        pub fn set_value(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Register lock bit."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Register lock bit."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blanking(pub u8);
    impl Blanking {
        #[doc = "No blanking."]
        pub const NOBLANKING: Self = Self(0x0);
        #[doc = "Check data sheet for blanking options"]
        pub const BLANK1: Self = Self(0x01);
        #[doc = "Check data sheet for blanking options"]
        pub const BLANK2: Self = Self(0x02);
        #[doc = "Check data sheet for blanking options"]
        pub const BLANK3: Self = Self(0x04);
    }
    impl Blanking {
        pub const fn from_bits(val: u8) -> Blanking {
            Self(val & 0x1f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl From<u8> for Blanking {
        #[inline(always)]
        fn from(val: u8) -> Blanking {
            Blanking::from_bits(val)
        }
    }
    impl From<Blanking> for u8 {
        #[inline(always)]
        fn from(val: Blanking) -> u8 {
            Blanking::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hysteresis {
        NONE = 0x0,
        LOW = 0x01,
        MEDIUM = 0x02,
        HIGH = 0x03,
    }
    impl Hysteresis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hysteresis {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hysteresis {
        #[inline(always)]
        fn from(val: u8) -> Hysteresis {
            Hysteresis::from_bits(val)
        }
    }
    impl From<Hysteresis> for u8 {
        #[inline(always)]
        fn from(val: Hysteresis) -> u8 {
            Hysteresis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Inm {
        #[doc = "Inverting input set to 1/4 VRef"]
        QUARTERVREF = 0x0,
        #[doc = "Inverting input set to 1/2 VRef"]
        HALFVREF = 0x01,
        #[doc = "Inverting input set to 3/4 VRef"]
        THREEQUARTERVREF = 0x02,
        #[doc = "Inverting input set to VRef"]
        VREF = 0x03,
        #[doc = "Inverting input set to DAC1 output"]
        DAC1 = 0x04,
        #[doc = "Inverting input set to DAC2 output"]
        DAC2 = 0x05,
        #[doc = "Inverting input set to IO1 (PB7)"]
        INM1 = 0x06,
        #[doc = "Inverting input set to IO2 (PB3)"]
        INM2 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Inm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Inm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Inm {
        #[inline(always)]
        fn from(val: u8) -> Inm {
            Inm::from_bits(val)
        }
    }
    impl From<Inm> for u8 {
        #[inline(always)]
        fn from(val: Inm) -> u8 {
            Inm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Polarity {
        #[doc = "Output is not inverted."]
        NOTINVERTED = 0x0,
        #[doc = "Output is inverted."]
        INVERTED = 0x01,
    }
    impl Polarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Polarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Polarity {
        #[inline(always)]
        fn from(val: u8) -> Polarity {
            Polarity::from_bits(val)
        }
    }
    impl From<Polarity> for u8 {
        #[inline(always)]
        fn from(val: Polarity) -> u8 {
            Polarity::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PowerMode {
        #[doc = "High speed / full power."]
        HIGHSPEED = 0x0,
        #[doc = "Medium speed / medium power."]
        MEDIUMSPEED = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Very-low speed / ultra-low power."]
        ULTRALOW = 0x03,
    }
    impl PowerMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PowerMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PowerMode {
        #[inline(always)]
        fn from(val: u8) -> PowerMode {
            PowerMode::from_bits(val)
        }
    }
    impl From<PowerMode> for u8 {
        #[inline(always)]
        fn from(val: PowerMode) -> u8 {
            PowerMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum WindowMode {
        #[doc = "Signal selected with INPSEL\\[2:0\\]
bitfield of this register."]
        THISINPSEL = 0x0,
        #[doc = "Signal selected with INPSEL\\[2:0\\]
bitfield of the other register (required for window mode)."]
        OTHERINPSEL = 0x01,
    }
    impl WindowMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WindowMode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WindowMode {
        #[inline(always)]
        fn from(val: u8) -> WindowMode {
            WindowMode::from_bits(val)
        }
    }
    impl From<WindowMode> for u8 {
        #[inline(always)]
        fn from(val: WindowMode) -> u8 {
            WindowMode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum WindowOut {
        #[doc = "Comparator 1 value."]
        COMP1_VALUE = 0x0,
        #[doc = "Comparator 1 value XOR comparator 2 value (required for window mode)."]
        COMP1_VALUEXORCOMP2_VALUE = 0x01,
    }
    impl WindowOut {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WindowOut {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WindowOut {
        #[inline(always)]
        fn from(val: u8) -> WindowOut {
            WindowOut::from_bits(val)
        }
    }
    impl From<WindowOut> for u8 {
        #[inline(always)]
        fn from(val: WindowOut) -> u8 {
            WindowOut::to_bits(val)
        }
    }
}
