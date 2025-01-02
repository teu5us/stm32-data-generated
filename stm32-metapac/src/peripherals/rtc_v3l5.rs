#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Real-time clock"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Time register"]
    #[inline(always)]
    pub const fn tr(self) -> crate::common::Reg<regs::Tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Date register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Sub second register"]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Initialization control and status register"]
    #[inline(always)]
    pub const fn icsr(self) -> crate::common::Reg<regs::Icsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Prescaler register"]
    #[inline(always)]
    pub const fn prer(self) -> crate::common::Reg<regs::Prer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Wakeup timer register"]
    #[inline(always)]
    pub const fn wutr(self) -> crate::common::Reg<regs::Wutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Privilege mode control register"]
    #[inline(always)]
    pub const fn privcr(self) -> crate::common::Reg<regs::Privcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Write protection register"]
    #[inline(always)]
    pub const fn wpr(self) -> crate::common::Reg<regs::Wpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Calibration register"]
    #[inline(always)]
    pub const fn calr(self) -> crate::common::Reg<regs::Calr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Shift control register"]
    #[inline(always)]
    pub const fn shiftr(self) -> crate::common::Reg<regs::Shiftr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Timestamp time register"]
    #[inline(always)]
    pub const fn tstr(self) -> crate::common::Reg<regs::Tstr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Timestamp date register"]
    #[inline(always)]
    pub const fn tsdr(self) -> crate::common::Reg<regs::Tsdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Timestamp sub second register"]
    #[inline(always)]
    pub const fn tsssr(self) -> crate::common::Reg<regs::Tsssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Alarm register"]
    #[inline(always)]
    pub const fn alrmr(self, n: usize) -> crate::common::Reg<regs::Alrmr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 8usize) as _) }
    }
    #[doc = "Alarm sub second register"]
    #[inline(always)]
    pub const fn alrmssr(self, n: usize) -> crate::common::Reg<regs::Alrmssr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize + n * 8usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Secure masked interrupt status register"]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Alarm register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrmr(pub u32);
    impl Alrmr {
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Alarm A seconds mask"]
        #[inline(always)]
        pub const fn msk1(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm A seconds mask"]
        #[inline(always)]
        pub fn set_msk1(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Alarm A minutes mask"]
        #[inline(always)]
        pub const fn msk2(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm A minutes mask"]
        #[inline(always)]
        pub fn set_msk2(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::AlrmrPm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::AlrmrPm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: super::vals::AlrmrPm) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Alarm A hours mask"]
        #[inline(always)]
        pub const fn msk3(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm A hours mask"]
        #[inline(always)]
        pub fn set_msk3(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Date units or day in BCD format"]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units or day in BCD format"]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Week day selection"]
        #[inline(always)]
        pub const fn wdsel(&self) -> super::vals::AlrmrWdsel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::AlrmrWdsel::from_bits(val as u8)
        }
        #[doc = "Week day selection"]
        #[inline(always)]
        pub fn set_wdsel(&mut self, val: super::vals::AlrmrWdsel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Alarm A date mask"]
        #[inline(always)]
        pub const fn msk4(&self) -> super::vals::AlrmrMsk {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::AlrmrMsk::from_bits(val as u8)
        }
        #[doc = "Alarm A date mask"]
        #[inline(always)]
        pub fn set_msk4(&mut self, val: super::vals::AlrmrMsk) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Alrmr {
        #[inline(always)]
        fn default() -> Alrmr {
            Alrmr(0)
        }
    }
    impl core::fmt::Debug for Alrmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrmr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("msk1", &self.msk1())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("msk2", &self.msk2())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .field("msk3", &self.msk3())
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("wdsel", &self.wdsel())
                .field("msk4", &self.msk4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrmr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Alrmr {
                su: u8,
                st: u8,
                msk1: super::vals::AlrmrMsk,
                mnu: u8,
                mnt: u8,
                msk2: super::vals::AlrmrMsk,
                hu: u8,
                ht: u8,
                pm: super::vals::AlrmrPm,
                msk3: super::vals::AlrmrMsk,
                du: u8,
                dt: u8,
                wdsel: super::vals::AlrmrWdsel,
                msk4: super::vals::AlrmrMsk,
            }
            let proxy = Alrmr {
                su: self.su(),
                st: self.st(),
                msk1: self.msk1(),
                mnu: self.mnu(),
                mnt: self.mnt(),
                msk2: self.msk2(),
                hu: self.hu(),
                ht: self.ht(),
                pm: self.pm(),
                msk3: self.msk3(),
                du: self.du(),
                dt: self.dt(),
                wdsel: self.wdsel(),
                msk4: self.msk4(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Alarm sub second register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alrmssr(pub u32);
    impl Alrmssr {
        #[doc = "Sub seconds value"]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Sub seconds value"]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Mask the most-significant bits starting at this bit"]
        #[inline(always)]
        pub const fn maskss(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Mask the most-significant bits starting at this bit"]
        #[inline(always)]
        pub fn set_maskss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Alrmssr {
        #[inline(always)]
        fn default() -> Alrmssr {
            Alrmssr(0)
        }
    }
    impl core::fmt::Debug for Alrmssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alrmssr")
                .field("ss", &self.ss())
                .field("maskss", &self.maskss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alrmssr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Alrmssr {
                ss: u16,
                maskss: u8,
            }
            let proxy = Alrmssr {
                ss: self.ss(),
                maskss: self.maskss(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calr(pub u32);
    impl Calr {
        #[doc = "Calibration minus"]
        #[inline(always)]
        pub const fn calm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Calibration minus"]
        #[inline(always)]
        pub fn set_calm(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Calibration low-power mode"]
        #[inline(always)]
        pub const fn lpcal(&self) -> super::vals::Lpcal {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Lpcal::from_bits(val as u8)
        }
        #[doc = "Calibration low-power mode"]
        #[inline(always)]
        pub fn set_lpcal(&mut self, val: super::vals::Lpcal) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Use a 16-second calibration cycle period"]
        #[inline(always)]
        pub const fn calw16(&self) -> super::vals::Calw16 {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Calw16::from_bits(val as u8)
        }
        #[doc = "Use a 16-second calibration cycle period"]
        #[inline(always)]
        pub fn set_calw16(&mut self, val: super::vals::Calw16) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Use an 8-second calibration cycle period"]
        #[inline(always)]
        pub const fn calw8(&self) -> super::vals::Calw8 {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Calw8::from_bits(val as u8)
        }
        #[doc = "Use an 8-second calibration cycle period"]
        #[inline(always)]
        pub fn set_calw8(&mut self, val: super::vals::Calw8) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Increase frequency of RTC by 488.5 ppm"]
        #[inline(always)]
        pub const fn calp(&self) -> super::vals::Calp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Calp::from_bits(val as u8)
        }
        #[doc = "Increase frequency of RTC by 488.5 ppm"]
        #[inline(always)]
        pub fn set_calp(&mut self, val: super::vals::Calp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Calr {
        #[inline(always)]
        fn default() -> Calr {
            Calr(0)
        }
    }
    impl core::fmt::Debug for Calr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calr")
                .field("calm", &self.calm())
                .field("lpcal", &self.lpcal())
                .field("calw16", &self.calw16())
                .field("calw8", &self.calw8())
                .field("calp", &self.calp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Calr {
                calm: u16,
                lpcal: super::vals::Lpcal,
                calw16: super::vals::Calw16,
                calw8: super::vals::Calw8,
                calp: super::vals::Calp,
            }
            let proxy = Calr {
                calm: self.calm(),
                lpcal: self.lpcal(),
                calw16: self.calw16(),
                calw8: self.calw8(),
                calp: self.calp(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Wakeup clock selection"]
        #[inline(always)]
        pub const fn wucksel(&self) -> super::vals::Wucksel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Wucksel::from_bits(val as u8)
        }
        #[doc = "Wakeup clock selection"]
        #[inline(always)]
        pub fn set_wucksel(&mut self, val: super::vals::Wucksel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Timestamp event active edge"]
        #[inline(always)]
        pub const fn tsedge(&self) -> super::vals::Tsedge {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Tsedge::from_bits(val as u8)
        }
        #[doc = "Timestamp event active edge"]
        #[inline(always)]
        pub fn set_tsedge(&mut self, val: super::vals::Tsedge) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
        #[inline(always)]
        pub const fn refckon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
        #[inline(always)]
        pub fn set_refckon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Bypass the shadow registers"]
        #[inline(always)]
        pub const fn bypshad(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Bypass the shadow registers"]
        #[inline(always)]
        pub fn set_bypshad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Hour format"]
        #[inline(always)]
        pub const fn fmt(&self) -> super::vals::Fmt {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Fmt::from_bits(val as u8)
        }
        #[doc = "Hour format"]
        #[inline(always)]
        pub fn set_fmt(&mut self, val: super::vals::Fmt) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Alarm enable"]
        #[inline(always)]
        pub const fn alre(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm enable"]
        #[inline(always)]
        pub fn set_alre(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer enable"]
        #[inline(always)]
        pub const fn wute(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer enable"]
        #[inline(always)]
        pub fn set_wute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timestamp enable"]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp enable"]
        #[inline(always)]
        pub fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Alarm interrupt enable"]
        #[inline(always)]
        pub const fn alrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm interrupt enable"]
        #[inline(always)]
        pub fn set_alrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer interrupt enable"]
        #[inline(always)]
        pub const fn wutie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer interrupt enable"]
        #[inline(always)]
        pub fn set_wutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Timestamp interrupt enable"]
        #[inline(always)]
        pub const fn tsie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp interrupt enable"]
        #[inline(always)]
        pub fn set_tsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Add 1 hour (summer time change)"]
        #[inline(always)]
        pub const fn add1h(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Add 1 hour (summer time change)"]
        #[inline(always)]
        pub fn set_add1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Subtract 1 hour (winter time change)"]
        #[inline(always)]
        pub const fn sub1h(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Subtract 1 hour (winter time change)"]
        #[inline(always)]
        pub fn set_sub1h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Backup"]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Backup"]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Calibration output selection"]
        #[inline(always)]
        pub const fn cosel(&self) -> super::vals::Cosel {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Cosel::from_bits(val as u8)
        }
        #[doc = "Calibration output selection"]
        #[inline(always)]
        pub fn set_cosel(&mut self, val: super::vals::Cosel) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Output polarity"]
        #[inline(always)]
        pub const fn pol(&self) -> super::vals::Pol {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Pol::from_bits(val as u8)
        }
        #[doc = "Output polarity"]
        #[inline(always)]
        pub fn set_pol(&mut self, val: super::vals::Pol) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Output selection"]
        #[inline(always)]
        pub const fn osel(&self) -> super::vals::Osel {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Osel::from_bits(val as u8)
        }
        #[doc = "Output selection"]
        #[inline(always)]
        pub fn set_osel(&mut self, val: super::vals::Osel) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "Calibration output enable"]
        #[inline(always)]
        pub const fn coe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration output enable"]
        #[inline(always)]
        pub fn set_coe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Timestamp on internal event enable"]
        #[inline(always)]
        pub const fn itse(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp on internal event enable"]
        #[inline(always)]
        pub fn set_itse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Activate timestamp on tamper detection event"]
        #[inline(always)]
        pub const fn tampts(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Activate timestamp on tamper detection event"]
        #[inline(always)]
        pub fn set_tampts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Tamper detection output enable on TAMPALRM"]
        #[inline(always)]
        pub const fn tampoe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection output enable on TAMPALRM"]
        #[inline(always)]
        pub fn set_tampoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "TAMPALRM pull-up enable"]
        #[inline(always)]
        pub const fn tampalrm_pu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TAMPALRM pull-up enable"]
        #[inline(always)]
        pub fn set_tampalrm_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "TAMPALRM output type"]
        #[inline(always)]
        pub const fn tampalrm_type(&self) -> super::vals::TampalrmType {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::TampalrmType::from_bits(val as u8)
        }
        #[doc = "TAMPALRM output type"]
        #[inline(always)]
        pub fn set_tampalrm_type(&mut self, val: super::vals::TampalrmType) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "RTC_OUT2 output enable"]
        #[inline(always)]
        pub const fn out2en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RTC_OUT2 output enable"]
        #[inline(always)]
        pub fn set_out2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("wucksel", &self.wucksel())
                .field("tsedge", &self.tsedge())
                .field("refckon", &self.refckon())
                .field("bypshad", &self.bypshad())
                .field("fmt", &self.fmt())
                .field("alre", &[self.alre(0usize), self.alre(1usize)])
                .field("wute", &self.wute())
                .field("tse", &self.tse())
                .field("alrie", &[self.alrie(0usize), self.alrie(1usize)])
                .field("wutie", &self.wutie())
                .field("tsie", &self.tsie())
                .field("add1h", &self.add1h())
                .field("sub1h", &self.sub1h())
                .field("bkp", &self.bkp())
                .field("cosel", &self.cosel())
                .field("pol", &self.pol())
                .field("osel", &self.osel())
                .field("coe", &self.coe())
                .field("itse", &self.itse())
                .field("tampts", &self.tampts())
                .field("tampoe", &self.tampoe())
                .field("tampalrm_pu", &self.tampalrm_pu())
                .field("tampalrm_type", &self.tampalrm_type())
                .field("out2en", &self.out2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr {
                wucksel: super::vals::Wucksel,
                tsedge: super::vals::Tsedge,
                refckon: bool,
                bypshad: bool,
                fmt: super::vals::Fmt,
                alre: [bool; 2usize],
                wute: bool,
                tse: bool,
                alrie: [bool; 2usize],
                wutie: bool,
                tsie: bool,
                add1h: bool,
                sub1h: bool,
                bkp: bool,
                cosel: super::vals::Cosel,
                pol: super::vals::Pol,
                osel: super::vals::Osel,
                coe: bool,
                itse: bool,
                tampts: bool,
                tampoe: bool,
                tampalrm_pu: bool,
                tampalrm_type: super::vals::TampalrmType,
                out2en: bool,
            }
            let proxy = Cr {
                wucksel: self.wucksel(),
                tsedge: self.tsedge(),
                refckon: self.refckon(),
                bypshad: self.bypshad(),
                fmt: self.fmt(),
                alre: [self.alre(0usize), self.alre(1usize)],
                wute: self.wute(),
                tse: self.tse(),
                alrie: [self.alrie(0usize), self.alrie(1usize)],
                wutie: self.wutie(),
                tsie: self.tsie(),
                add1h: self.add1h(),
                sub1h: self.sub1h(),
                bkp: self.bkp(),
                cosel: self.cosel(),
                pol: self.pol(),
                osel: self.osel(),
                coe: self.coe(),
                itse: self.itse(),
                tampts: self.tampts(),
                tampoe: self.tampoe(),
                tampalrm_pu: self.tampalrm_pu(),
                tampalrm_type: self.tampalrm_type(),
                out2en: self.out2en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Date register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub fn set_wdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Year units in BCD format"]
        #[inline(always)]
        pub const fn yu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Year units in BCD format"]
        #[inline(always)]
        pub fn set_yu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Year tens in BCD format"]
        #[inline(always)]
        pub const fn yt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Year tens in BCD format"]
        #[inline(always)]
        pub fn set_yt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr")
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("mu", &self.mu())
                .field("mt", &self.mt())
                .field("wdu", &self.wdu())
                .field("yu", &self.yu())
                .field("yt", &self.yt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Dr {
                du: u8,
                dt: u8,
                mu: u8,
                mt: bool,
                wdu: u8,
                yu: u8,
                yt: u8,
            }
            let proxy = Dr {
                du: self.du(),
                dt: self.dt(),
                mu: self.mu(),
                mt: self.mt(),
                wdu: self.wdu(),
                yu: self.yu(),
                yt: self.yt(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Initialization control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icsr(pub u32);
    impl Icsr {
        #[doc = "Wakeup timer write flag"]
        #[inline(always)]
        pub const fn wutwf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer write flag"]
        #[inline(always)]
        pub fn set_wutwf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Shift operation pending"]
        #[inline(always)]
        pub const fn shpf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Shift operation pending"]
        #[inline(always)]
        pub fn set_shpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Initialization status flag"]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization status flag"]
        #[inline(always)]
        pub fn set_inits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Registers synchronization flag"]
        #[inline(always)]
        pub const fn rsf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Registers synchronization flag"]
        #[inline(always)]
        pub fn set_rsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Initialization flag"]
        #[inline(always)]
        pub const fn initf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization flag"]
        #[inline(always)]
        pub fn set_initf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Initialization mode"]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization mode"]
        #[inline(always)]
        pub fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Recalibration pending Flag"]
        #[inline(always)]
        pub const fn recalpf(&self) -> super::vals::Recalpf {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Recalpf::from_bits(val as u8)
        }
        #[doc = "Recalibration pending Flag"]
        #[inline(always)]
        pub fn set_recalpf(&mut self, val: super::vals::Recalpf) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Icsr {
        #[inline(always)]
        fn default() -> Icsr {
            Icsr(0)
        }
    }
    impl core::fmt::Debug for Icsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icsr")
                .field("wutwf", &self.wutwf())
                .field("shpf", &self.shpf())
                .field("inits", &self.inits())
                .field("rsf", &self.rsf())
                .field("initf", &self.initf())
                .field("init", &self.init())
                .field("recalpf", &self.recalpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icsr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Icsr {
                wutwf: bool,
                shpf: bool,
                inits: bool,
                rsf: bool,
                initf: bool,
                init: bool,
                recalpf: super::vals::Recalpf,
            }
            let proxy = Icsr {
                wutwf: self.wutwf(),
                shpf: self.shpf(),
                inits: self.inits(),
                rsf: self.rsf(),
                initf: self.initf(),
                init: self.init(),
                recalpf: self.recalpf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Alarm masked flag"]
        #[inline(always)]
        pub const fn alrmf(&self, n: usize) -> super::vals::Alrmf {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Alrmf::from_bits(val as u8)
        }
        #[doc = "Alarm masked flag"]
        #[inline(always)]
        pub fn set_alrmf(&mut self, n: usize, val: super::vals::Alrmf) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer masked flag"]
        #[inline(always)]
        pub const fn wutmf(&self) -> super::vals::Wutmf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wutmf::from_bits(val as u8)
        }
        #[doc = "Wakeup timer masked flag"]
        #[inline(always)]
        pub fn set_wutmf(&mut self, val: super::vals::Wutmf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp masked flag"]
        #[inline(always)]
        pub const fn tsmf(&self) -> super::vals::Tsmf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Tsmf::from_bits(val as u8)
        }
        #[doc = "Timestamp masked flag"]
        #[inline(always)]
        pub fn set_tsmf(&mut self, val: super::vals::Tsmf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp overflow masked flag"]
        #[inline(always)]
        pub const fn tsovmf(&self) -> super::vals::Tsovmf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Tsovmf::from_bits(val as u8)
        }
        #[doc = "Timestamp overflow masked flag"]
        #[inline(always)]
        pub fn set_tsovmf(&mut self, val: super::vals::Tsovmf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal timestamp masked flag"]
        #[inline(always)]
        pub const fn itsmf(&self) -> super::vals::Itsmf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Itsmf::from_bits(val as u8)
        }
        #[doc = "Internal timestamp masked flag"]
        #[inline(always)]
        pub fn set_itsmf(&mut self, val: super::vals::Itsmf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("alrmf", &[self.alrmf(0usize), self.alrmf(1usize)])
                .field("wutmf", &self.wutmf())
                .field("tsmf", &self.tsmf())
                .field("tsovmf", &self.tsovmf())
                .field("itsmf", &self.itsmf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Misr {
                alrmf: [super::vals::Alrmf; 2usize],
                wutmf: super::vals::Wutmf,
                tsmf: super::vals::Tsmf,
                tsovmf: super::vals::Tsovmf,
                itsmf: super::vals::Itsmf,
            }
            let proxy = Misr {
                alrmf: [self.alrmf(0usize), self.alrmf(1usize)],
                wutmf: self.wutmf(),
                tsmf: self.tsmf(),
                tsovmf: self.tsovmf(),
                itsmf: self.itsmf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prer(pub u32);
    impl Prer {
        #[doc = "Synchronous prescaler factor"]
        #[inline(always)]
        pub const fn prediv_s(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Synchronous prescaler factor"]
        #[inline(always)]
        pub fn set_prediv_s(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Asynchronous prescaler factor"]
        #[inline(always)]
        pub const fn prediv_a(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Asynchronous prescaler factor"]
        #[inline(always)]
        pub fn set_prediv_a(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Prer {
        #[inline(always)]
        fn default() -> Prer {
            Prer(0)
        }
    }
    impl core::fmt::Debug for Prer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prer")
                .field("prediv_s", &self.prediv_s())
                .field("prediv_a", &self.prediv_a())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prer {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Prer {
                prediv_s: u16,
                prediv_a: u8,
            }
            let proxy = Prer {
                prediv_s: self.prediv_s(),
                prediv_a: self.prediv_a(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Privilege mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcr(pub u32);
    impl Privcr {
        #[doc = "ALRAPRIV"]
        #[inline(always)]
        pub const fn alrpriv(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ALRAPRIV"]
        #[inline(always)]
        pub fn set_alrpriv(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "WUTPRIV"]
        #[inline(always)]
        pub const fn wutpriv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WUTPRIV"]
        #[inline(always)]
        pub fn set_wutpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TSPRIV"]
        #[inline(always)]
        pub const fn tspriv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TSPRIV"]
        #[inline(always)]
        pub fn set_tspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CALPRIV"]
        #[inline(always)]
        pub const fn calpriv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CALPRIV"]
        #[inline(always)]
        pub fn set_calpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "INITPRIV"]
        #[inline(always)]
        pub const fn initpriv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "INITPRIV"]
        #[inline(always)]
        pub fn set_initpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "PRIV"]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PRIV"]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Privcr {
        #[inline(always)]
        fn default() -> Privcr {
            Privcr(0)
        }
    }
    impl core::fmt::Debug for Privcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcr")
                .field("alrpriv", &[self.alrpriv(0usize), self.alrpriv(1usize)])
                .field("wutpriv", &self.wutpriv())
                .field("tspriv", &self.tspriv())
                .field("calpriv", &self.calpriv())
                .field("initpriv", &self.initpriv())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Privcr {
                alrpriv: [bool; 2usize],
                wutpriv: bool,
                tspriv: bool,
                calpriv: bool,
                initpriv: bool,
                priv_: bool,
            }
            let proxy = Privcr {
                alrpriv: [self.alrpriv(0usize), self.alrpriv(1usize)],
                wutpriv: self.wutpriv(),
                tspriv: self.tspriv(),
                calpriv: self.calpriv(),
                initpriv: self.initpriv(),
                priv_: self.priv_(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear alarm x flag"]
        #[inline(always)]
        pub const fn calrf(&self, n: usize) -> super::vals::Calrf {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear alarm x flag"]
        #[inline(always)]
        pub fn set_calrf(&mut self, n: usize, val: super::vals::Calrf) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Clear wakeup timer flag"]
        #[inline(always)]
        pub const fn cwutf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear wakeup timer flag"]
        #[inline(always)]
        pub fn set_cwutf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear timestamp flag"]
        #[inline(always)]
        pub const fn ctsf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear timestamp flag"]
        #[inline(always)]
        pub fn set_ctsf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear timestamp overflow flag"]
        #[inline(always)]
        pub const fn ctsovf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear timestamp overflow flag"]
        #[inline(always)]
        pub fn set_ctsovf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear internal timestamp flag"]
        #[inline(always)]
        pub const fn citsf(&self) -> super::vals::Calrf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Calrf::from_bits(val as u8)
        }
        #[doc = "Clear internal timestamp flag"]
        #[inline(always)]
        pub fn set_citsf(&mut self, val: super::vals::Calrf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("calrf", &[self.calrf(0usize), self.calrf(1usize)])
                .field("cwutf", &self.cwutf())
                .field("ctsf", &self.ctsf())
                .field("ctsovf", &self.ctsovf())
                .field("citsf", &self.citsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Scr {
                calrf: [super::vals::Calrf; 2usize],
                cwutf: super::vals::Calrf,
                ctsf: super::vals::Calrf,
                ctsovf: super::vals::Calrf,
                citsf: super::vals::Calrf,
            }
            let proxy = Scr {
                calrf: [self.calrf(0usize), self.calrf(1usize)],
                cwutf: self.cwutf(),
                ctsf: self.ctsf(),
                ctsovf: self.ctsovf(),
                citsf: self.citsf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Shift control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shiftr(pub u32);
    impl Shiftr {
        #[doc = "Subtract a fraction of a second"]
        #[inline(always)]
        pub const fn subfs(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Subtract a fraction of a second"]
        #[inline(always)]
        pub fn set_subfs(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "Add one second"]
        #[inline(always)]
        pub const fn add1s(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Add one second"]
        #[inline(always)]
        pub fn set_add1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Shiftr {
        #[inline(always)]
        fn default() -> Shiftr {
            Shiftr(0)
        }
    }
    impl core::fmt::Debug for Shiftr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Shiftr")
                .field("subfs", &self.subfs())
                .field("add1s", &self.add1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Shiftr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Shiftr {
                subfs: u16,
                add1s: bool,
            }
            let proxy = Shiftr {
                subfs: self.subfs(),
                add1s: self.add1s(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RTC secure mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr(pub u32);
    impl Smcr {
        #[doc = "Alarm x protection"]
        #[inline(always)]
        pub const fn alrdprot(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm x protection"]
        #[inline(always)]
        pub fn set_alrdprot(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer protection"]
        #[inline(always)]
        pub const fn wutdprot(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup timer protection"]
        #[inline(always)]
        pub fn set_wutdprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp protection"]
        #[inline(always)]
        pub const fn tsdprot(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timestamp protection"]
        #[inline(always)]
        pub fn set_tsdprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Shift register, daylight saving, calibration and reference clock protection"]
        #[inline(always)]
        pub const fn caldprot(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Shift register, daylight saving, calibration and reference clock protection"]
        #[inline(always)]
        pub fn set_caldprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Initialization protection"]
        #[inline(always)]
        pub const fn initdprot(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Initialization protection"]
        #[inline(always)]
        pub fn set_initdprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "RTC global protection"]
        #[inline(always)]
        pub const fn decprot(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC global protection"]
        #[inline(always)]
        pub fn set_decprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Smcr {
        #[inline(always)]
        fn default() -> Smcr {
            Smcr(0)
        }
    }
    impl core::fmt::Debug for Smcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smcr")
                .field("alrdprot", &[self.alrdprot(0usize), self.alrdprot(1usize)])
                .field("wutdprot", &self.wutdprot())
                .field("tsdprot", &self.tsdprot())
                .field("caldprot", &self.caldprot())
                .field("initdprot", &self.initdprot())
                .field("decprot", &self.decprot())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Smcr {
                alrdprot: [bool; 2usize],
                wutdprot: bool,
                tsdprot: bool,
                caldprot: bool,
                initdprot: bool,
                decprot: bool,
            }
            let proxy = Smcr {
                alrdprot: [self.alrdprot(0usize), self.alrdprot(1usize)],
                wutdprot: self.wutdprot(),
                tsdprot: self.tsdprot(),
                caldprot: self.caldprot(),
                initdprot: self.initdprot(),
                decprot: self.decprot(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Secure masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "Alarm x interrupt secure masked flag"]
        #[inline(always)]
        pub const fn alrmf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alarm x interrupt secure masked flag"]
        #[inline(always)]
        pub fn set_alrmf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "WUTMF"]
        #[inline(always)]
        pub const fn wutmf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WUTMF"]
        #[inline(always)]
        pub fn set_wutmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TSMF"]
        #[inline(always)]
        pub const fn tsmf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TSMF"]
        #[inline(always)]
        pub fn set_tsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TSOVMF"]
        #[inline(always)]
        pub const fn tsovmf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TSOVMF"]
        #[inline(always)]
        pub fn set_tsovmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ITSMF"]
        #[inline(always)]
        pub const fn itsmf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ITSMF"]
        #[inline(always)]
        pub fn set_itsmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Smisr {
        #[inline(always)]
        fn default() -> Smisr {
            Smisr(0)
        }
    }
    impl core::fmt::Debug for Smisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smisr")
                .field("alrmf", &[self.alrmf(0usize), self.alrmf(1usize)])
                .field("wutmf", &self.wutmf())
                .field("tsmf", &self.tsmf())
                .field("tsovmf", &self.tsovmf())
                .field("itsmf", &self.itsmf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smisr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Smisr {
                alrmf: [bool; 2usize],
                wutmf: bool,
                tsmf: bool,
                tsovmf: bool,
                itsmf: bool,
            }
            let proxy = Smisr {
                alrmf: [self.alrmf(0usize), self.alrmf(1usize)],
                wutmf: self.wutmf(),
                tsmf: self.tsmf(),
                tsovmf: self.tsovmf(),
                itsmf: self.itsmf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Alarm flag"]
        #[inline(always)]
        pub const fn alrf(&self, n: usize) -> super::vals::Alrf {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Alrf::from_bits(val as u8)
        }
        #[doc = "Alarm flag"]
        #[inline(always)]
        pub fn set_alrf(&mut self, n: usize, val: super::vals::Alrf) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup timer flag"]
        #[inline(always)]
        pub const fn wutf(&self) -> super::vals::Wutf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wutf::from_bits(val as u8)
        }
        #[doc = "Wakeup timer flag"]
        #[inline(always)]
        pub fn set_wutf(&mut self, val: super::vals::Wutf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Timestamp flag"]
        #[inline(always)]
        pub const fn tsf(&self) -> super::vals::Tsf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Tsf::from_bits(val as u8)
        }
        #[doc = "Timestamp flag"]
        #[inline(always)]
        pub fn set_tsf(&mut self, val: super::vals::Tsf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Timestamp overflow flag"]
        #[inline(always)]
        pub const fn tsovf(&self) -> super::vals::Tsovf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Tsovf::from_bits(val as u8)
        }
        #[doc = "Timestamp overflow flag"]
        #[inline(always)]
        pub fn set_tsovf(&mut self, val: super::vals::Tsovf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal timestamp flag"]
        #[inline(always)]
        pub const fn itsf(&self) -> super::vals::Itsf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Itsf::from_bits(val as u8)
        }
        #[doc = "Internal timestamp flag"]
        #[inline(always)]
        pub fn set_itsf(&mut self, val: super::vals::Itsf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("alrf", &[self.alrf(0usize), self.alrf(1usize)])
                .field("wutf", &self.wutf())
                .field("tsf", &self.tsf())
                .field("tsovf", &self.tsovf())
                .field("itsf", &self.itsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Sr {
                alrf: [super::vals::Alrf; 2usize],
                wutf: super::vals::Wutf,
                tsf: super::vals::Tsf,
                tsovf: super::vals::Tsovf,
                itsf: super::vals::Itsf,
            }
            let proxy = Sr {
                alrf: [self.alrf(0usize), self.alrf(1usize)],
                wutf: self.wutf(),
                tsf: self.tsf(),
                tsovf: self.tsovf(),
                itsf: self.itsf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Sub second register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssr(pub u32);
    impl Ssr {
        #[doc = "Synchronous binary counter"]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Synchronous binary counter"]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ssr {
        #[inline(always)]
        fn default() -> Ssr {
            Ssr(0)
        }
    }
    impl core::fmt::Debug for Ssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ssr").field("ss", &self.ss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ssr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ssr {
                ss: u16,
            }
            let proxy = Ssr { ss: self.ss() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr(pub u32);
    impl Tr {
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub const fn pm(&self) -> super::vals::Ampm {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Ampm::from_bits(val as u8)
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: super::vals::Ampm) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Tr {
        #[inline(always)]
        fn default() -> Tr {
            Tr(0)
        }
    }
    impl core::fmt::Debug for Tr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tr {
                su: u8,
                st: u8,
                mnu: u8,
                mnt: u8,
                hu: u8,
                ht: u8,
                pm: super::vals::Ampm,
            }
            let proxy = Tr {
                su: self.su(),
                st: self.st(),
                mnu: self.mnu(),
                mnt: self.mnt(),
                hu: self.hu(),
                ht: self.ht(),
                pm: self.pm(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timestamp date register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsdr(pub u32);
    impl Tsdr {
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub const fn du(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Date units in BCD format"]
        #[inline(always)]
        pub fn set_du(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub const fn dt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Date tens in BCD format"]
        #[inline(always)]
        pub fn set_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub const fn mu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Month units in BCD format"]
        #[inline(always)]
        pub fn set_mu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub const fn mt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Month tens in BCD format"]
        #[inline(always)]
        pub fn set_mt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub const fn wdu(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Week day units"]
        #[inline(always)]
        pub fn set_wdu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
    }
    impl Default for Tsdr {
        #[inline(always)]
        fn default() -> Tsdr {
            Tsdr(0)
        }
    }
    impl core::fmt::Debug for Tsdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsdr")
                .field("du", &self.du())
                .field("dt", &self.dt())
                .field("mu", &self.mu())
                .field("mt", &self.mt())
                .field("wdu", &self.wdu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsdr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tsdr {
                du: u8,
                dt: u8,
                mu: u8,
                mt: bool,
                wdu: u8,
            }
            let proxy = Tsdr {
                du: self.du(),
                dt: self.dt(),
                mu: self.mu(),
                mt: self.mt(),
                wdu: self.wdu(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timestamp sub second register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsssr(pub u32);
    impl Tsssr {
        #[doc = "Sub second value"]
        #[inline(always)]
        pub const fn ss(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sub second value"]
        #[inline(always)]
        pub fn set_ss(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Tsssr {
        #[inline(always)]
        fn default() -> Tsssr {
            Tsssr(0)
        }
    }
    impl core::fmt::Debug for Tsssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsssr").field("ss", &self.ss()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsssr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tsssr {
                ss: u16,
            }
            let proxy = Tsssr { ss: self.ss() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Timestamp time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tstr(pub u32);
    impl Tstr {
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub const fn su(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Second units in BCD format"]
        #[inline(always)]
        pub fn set_su(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub const fn st(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Second tens in BCD format"]
        #[inline(always)]
        pub fn set_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub const fn mnu(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Minute units in BCD format"]
        #[inline(always)]
        pub fn set_mnu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub const fn mnt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Minute tens in BCD format"]
        #[inline(always)]
        pub fn set_mnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub const fn hu(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Hour units in BCD format"]
        #[inline(always)]
        pub fn set_hu(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub const fn ht(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Hour tens in BCD format"]
        #[inline(always)]
        pub fn set_ht(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "AM/PM notation"]
        #[inline(always)]
        pub fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Tstr {
        #[inline(always)]
        fn default() -> Tstr {
            Tstr(0)
        }
    }
    impl core::fmt::Debug for Tstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tstr")
                .field("su", &self.su())
                .field("st", &self.st())
                .field("mnu", &self.mnu())
                .field("mnt", &self.mnt())
                .field("hu", &self.hu())
                .field("ht", &self.ht())
                .field("pm", &self.pm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tstr {
                su: u8,
                st: u8,
                mnu: u8,
                mnt: u8,
                hu: u8,
                ht: u8,
                pm: bool,
            }
            let proxy = Tstr {
                su: self.su(),
                st: self.st(),
                mnu: self.mnu(),
                mnt: self.mnt(),
                hu: self.hu(),
                ht: self.ht(),
                pm: self.pm(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wpr(pub u32);
    impl Wpr {
        #[doc = "Write protection key"]
        #[inline(always)]
        pub const fn key(&self) -> super::vals::Key {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Key::from_bits(val as u8)
        }
        #[doc = "Write protection key"]
        #[inline(always)]
        pub fn set_key(&mut self, val: super::vals::Key) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wpr {
        #[inline(always)]
        fn default() -> Wpr {
            Wpr(0)
        }
    }
    impl core::fmt::Debug for Wpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wpr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wpr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Wpr {
                key: super::vals::Key,
            }
            let proxy = Wpr { key: self.key() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Wakeup timer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wutr(pub u32);
    impl Wutr {
        #[doc = "Wakeup auto-reload value bits"]
        #[inline(always)]
        pub const fn wut(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Wakeup auto-reload value bits"]
        #[inline(always)]
        pub fn set_wut(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Wakeup auto-reload output clear value"]
        #[inline(always)]
        pub const fn wutoclr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Wakeup auto-reload output clear value"]
        #[inline(always)]
        pub fn set_wutoclr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Wutr {
        #[inline(always)]
        fn default() -> Wutr {
            Wutr(0)
        }
    }
    impl core::fmt::Debug for Wutr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wutr")
                .field("wut", &self.wut())
                .field("wutoclr", &self.wutoclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wutr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Wutr {
                wut: u16,
                wutoclr: u16,
            }
            let proxy = Wutr {
                wut: self.wut(),
                wutoclr: self.wutoclr(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Alrf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
        MATCH = 0x01,
    }
    impl Alrf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Alrf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Alrf {
        #[inline(always)]
        fn from(val: u8) -> Alrf {
            Alrf::from_bits(val)
        }
    }
    impl From<Alrf> for u8 {
        #[inline(always)]
        fn from(val: Alrf) -> u8 {
            Alrf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Alrmf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the Alarm A register (RTC_ALRMAR)"]
        MATCH = 0x01,
    }
    impl Alrmf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Alrmf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Alrmf {
        #[inline(always)]
        fn from(val: u8) -> Alrmf {
            Alrmf::from_bits(val)
        }
    }
    impl From<Alrmf> for u8 {
        #[inline(always)]
        fn from(val: Alrmf) -> u8 {
            Alrmf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AlrmrMsk {
        #[doc = "Alarm set if the date/day match"]
        TO_MATCH = 0x0,
        #[doc = "Date/day don’t care in Alarm comparison"]
        NOT_MATCH = 0x01,
    }
    impl AlrmrMsk {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AlrmrMsk {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AlrmrMsk {
        #[inline(always)]
        fn from(val: u8) -> AlrmrMsk {
            AlrmrMsk::from_bits(val)
        }
    }
    impl From<AlrmrMsk> for u8 {
        #[inline(always)]
        fn from(val: AlrmrMsk) -> u8 {
            AlrmrMsk::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AlrmrPm {
        #[doc = "AM or 24-hour format"]
        AM = 0x0,
        #[doc = "PM"]
        PM = 0x01,
    }
    impl AlrmrPm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AlrmrPm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AlrmrPm {
        #[inline(always)]
        fn from(val: u8) -> AlrmrPm {
            AlrmrPm::from_bits(val)
        }
    }
    impl From<AlrmrPm> for u8 {
        #[inline(always)]
        fn from(val: AlrmrPm) -> u8 {
            AlrmrPm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AlrmrWdsel {
        #[doc = "DU\\[3:0\\]
represents the date units"]
        DATE_UNITS = 0x0,
        #[doc = "DU\\[3:0\\]
represents the week day. DT\\[1:0\\]
is don’t care."]
        WEEK_DAY = 0x01,
    }
    impl AlrmrWdsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AlrmrWdsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AlrmrWdsel {
        #[inline(always)]
        fn from(val: u8) -> AlrmrWdsel {
            AlrmrWdsel::from_bits(val)
        }
    }
    impl From<AlrmrWdsel> for u8 {
        #[inline(always)]
        fn from(val: AlrmrWdsel) -> u8 {
            AlrmrWdsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ampm {
        #[doc = "AM or 24-hour format"]
        AM = 0x0,
        #[doc = "PM"]
        PM = 0x01,
    }
    impl Ampm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ampm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ampm {
        #[inline(always)]
        fn from(val: u8) -> Ampm {
            Ampm::from_bits(val)
        }
    }
    impl From<Ampm> for u8 {
        #[inline(always)]
        fn from(val: Ampm) -> u8 {
            Ampm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calp {
        #[doc = "No RTCCLK pulses are added"]
        NO_CHANGE = 0x0,
        #[doc = "One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)"]
        INCREASE_FREQ = 0x01,
    }
    impl Calp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calp {
        #[inline(always)]
        fn from(val: u8) -> Calp {
            Calp::from_bits(val)
        }
    }
    impl From<Calp> for u8 {
        #[inline(always)]
        fn from(val: Calp) -> u8 {
            Calp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calrf {
        _RESERVED_0 = 0x0,
        #[doc = "Clear interrupt flag by writing 1"]
        CLEAR = 0x01,
    }
    impl Calrf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calrf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calrf {
        #[inline(always)]
        fn from(val: u8) -> Calrf {
            Calrf::from_bits(val)
        }
    }
    impl From<Calrf> for u8 {
        #[inline(always)]
        fn from(val: Calrf) -> u8 {
            Calrf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calw16 {
        _RESERVED_0 = 0x0,
        #[doc = "When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1"]
        SIXTEEN_SECONDS = 0x01,
    }
    impl Calw16 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calw16 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calw16 {
        #[inline(always)]
        fn from(val: u8) -> Calw16 {
            Calw16::from_bits(val)
        }
    }
    impl From<Calw16> for u8 {
        #[inline(always)]
        fn from(val: Calw16) -> u8 {
            Calw16::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Calw8 {
        _RESERVED_0 = 0x0,
        #[doc = "When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected"]
        EIGHT_SECONDS = 0x01,
    }
    impl Calw8 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Calw8 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Calw8 {
        #[inline(always)]
        fn from(val: u8) -> Calw8 {
            Calw8::from_bits(val)
        }
    }
    impl From<Calw8> for u8 {
        #[inline(always)]
        fn from(val: Calw8) -> u8 {
            Calw8::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cosel {
        #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
        CAL_FREQ_512HZ = 0x0,
        #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
        CAL_FREQ_1HZ = 0x01,
    }
    impl Cosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cosel {
        #[inline(always)]
        fn from(val: u8) -> Cosel {
            Cosel::from_bits(val)
        }
    }
    impl From<Cosel> for u8 {
        #[inline(always)]
        fn from(val: Cosel) -> u8 {
            Cosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmt {
        #[doc = "24 hour/day format"]
        TWENTY_FOUR_HOUR = 0x0,
        #[doc = "AM/PM hour format"]
        AM_PM = 0x01,
    }
    impl Fmt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmt {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmt {
        #[inline(always)]
        fn from(val: u8) -> Fmt {
            Fmt::from_bits(val)
        }
    }
    impl From<Fmt> for u8 {
        #[inline(always)]
        fn from(val: Fmt) -> u8 {
            Fmt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Itsf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when a timestamp on the internal event occurs"]
        TIMESTAMP_EVENT = 0x01,
    }
    impl Itsf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Itsf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Itsf {
        #[inline(always)]
        fn from(val: u8) -> Itsf {
            Itsf::from_bits(val)
        }
    }
    impl From<Itsf> for u8 {
        #[inline(always)]
        fn from(val: Itsf) -> u8 {
            Itsf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Itsmf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when a timestamp on the internal event occurs"]
        TIMESTAMP_EVENT = 0x01,
    }
    impl Itsmf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Itsmf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Itsmf {
        #[inline(always)]
        fn from(val: u8) -> Itsmf {
            Itsmf::from_bits(val)
        }
    }
    impl From<Itsmf> for u8 {
        #[inline(always)]
        fn from(val: Itsmf) -> u8 {
            Itsmf::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key(pub u8);
    impl Key {
        #[doc = "Activate write protection (any value that is not the keys)"]
        pub const ACTIVATE: Self = Self(0x0);
        #[doc = "Key 2"]
        pub const DEACTIVATE2: Self = Self(0x53);
        #[doc = "Key 1"]
        pub const DEACTIVATE1: Self = Self(0xca);
    }
    impl Key {
        pub const fn from_bits(val: u8) -> Key {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Key {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("ACTIVATE"),
                0x53 => f.write_str("DEACTIVATE2"),
                0xca => f.write_str("DEACTIVATE1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Key {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "ACTIVATE"),
                0x53 => defmt::write!(f, "DEACTIVATE2"),
                0xca => defmt::write!(f, "DEACTIVATE1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Key {
        #[inline(always)]
        fn from(val: u8) -> Key {
            Key::from_bits(val)
        }
    }
    impl From<Key> for u8 {
        #[inline(always)]
        fn from(val: Key) -> u8 {
            Key::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpcal {
        #[doc = "Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required"]
        RTCCLK = 0x0,
        #[doc = "Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode"]
        CK_APRE = 0x01,
    }
    impl Lpcal {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpcal {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpcal {
        #[inline(always)]
        fn from(val: u8) -> Lpcal {
            Lpcal::from_bits(val)
        }
    }
    impl From<Lpcal> for u8 {
        #[inline(always)]
        fn from(val: Lpcal) -> u8 {
            Lpcal::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Osel {
        #[doc = "Output disabled"]
        DISABLED = 0x0,
        #[doc = "Alarm A output enabled"]
        ALARM_A = 0x01,
        #[doc = "Alarm B output enabled"]
        ALARM_B = 0x02,
        #[doc = "Wakeup output enabled"]
        WAKEUP = 0x03,
    }
    impl Osel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Osel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Osel {
        #[inline(always)]
        fn from(val: u8) -> Osel {
            Osel::from_bits(val)
        }
    }
    impl From<Osel> for u8 {
        #[inline(always)]
        fn from(val: Osel) -> u8 {
            Osel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pol {
        #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
        HIGH = 0x0,
        #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
        LOW = 0x01,
    }
    impl Pol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pol {
        #[inline(always)]
        fn from(val: u8) -> Pol {
            Pol::from_bits(val)
        }
    }
    impl From<Pol> for u8 {
        #[inline(always)]
        fn from(val: Pol) -> u8 {
            Pol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Recalpf {
        _RESERVED_0 = 0x0,
        #[doc = "The RECALPF status flag is automatically set to 1 when software writes to the RTC_CALR register, indicating that the RTC_CALR register is blocked. When the new calibration settings are taken into account, this bit returns to 0"]
        PENDING = 0x01,
    }
    impl Recalpf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Recalpf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Recalpf {
        #[inline(always)]
        fn from(val: u8) -> Recalpf {
            Recalpf::from_bits(val)
        }
    }
    impl From<Recalpf> for u8 {
        #[inline(always)]
        fn from(val: Recalpf) -> u8 {
            Recalpf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TampalrmType {
        #[doc = "TAMPALRM is push-pull output"]
        PUSH_PULL = 0x0,
        #[doc = "TAMPALRM is open-drain output"]
        OPEN_DRAIN = 0x01,
    }
    impl TampalrmType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TampalrmType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TampalrmType {
        #[inline(always)]
        fn from(val: u8) -> TampalrmType {
            TampalrmType::from_bits(val)
        }
    }
    impl From<TampalrmType> for u8 {
        #[inline(always)]
        fn from(val: TampalrmType) -> u8 {
            TampalrmType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsedge {
        #[doc = "RTC_TS input rising edge generates a time-stamp event"]
        RISING_EDGE = 0x0,
        #[doc = "RTC_TS input falling edge generates a time-stamp event"]
        FALLING_EDGE = 0x01,
    }
    impl Tsedge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsedge {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsedge {
        #[inline(always)]
        fn from(val: u8) -> Tsedge {
            Tsedge::from_bits(val)
        }
    }
    impl From<Tsedge> for u8 {
        #[inline(always)]
        fn from(val: Tsedge) -> u8 {
            Tsedge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when a time-stamp event occurs"]
        TIMESTAMP_EVENT = 0x01,
    }
    impl Tsf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsf {
        #[inline(always)]
        fn from(val: u8) -> Tsf {
            Tsf::from_bits(val)
        }
    }
    impl From<Tsf> for u8 {
        #[inline(always)]
        fn from(val: Tsf) -> u8 {
            Tsf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsmf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when a time-stamp event occurs"]
        TIMESTAMP_EVENT = 0x01,
    }
    impl Tsmf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsmf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsmf {
        #[inline(always)]
        fn from(val: u8) -> Tsmf {
            Tsmf::from_bits(val)
        }
    }
    impl From<Tsmf> for u8 {
        #[inline(always)]
        fn from(val: Tsmf) -> u8 {
            Tsmf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsovf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
        OVERFLOW = 0x01,
    }
    impl Tsovf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsovf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsovf {
        #[inline(always)]
        fn from(val: u8) -> Tsovf {
            Tsovf::from_bits(val)
        }
    }
    impl From<Tsovf> for u8 {
        #[inline(always)]
        fn from(val: Tsovf) -> u8 {
            Tsovf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tsovmf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when a time-stamp event occurs while TSF is already set"]
        OVERFLOW = 0x01,
    }
    impl Tsovmf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tsovmf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tsovmf {
        #[inline(always)]
        fn from(val: u8) -> Tsovmf {
            Tsovmf::from_bits(val)
        }
    }
    impl From<Tsovmf> for u8 {
        #[inline(always)]
        fn from(val: Tsovmf) -> u8 {
            Tsovmf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wucksel {
        #[doc = "RTC/16 clock is selected"]
        DIV16 = 0x0,
        #[doc = "RTC/8 clock is selected"]
        DIV8 = 0x01,
        #[doc = "RTC/4 clock is selected"]
        DIV4 = 0x02,
        #[doc = "RTC/2 clock is selected"]
        DIV2 = 0x03,
        #[doc = "ck_spre (usually 1 Hz) clock is selected"]
        CLOCK_SPARE = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
        CLOCK_SPARE_WITH_OFFSET = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Wucksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wucksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wucksel {
        #[inline(always)]
        fn from(val: u8) -> Wucksel {
            Wucksel::from_bits(val)
        }
    }
    impl From<Wucksel> for u8 {
        #[inline(always)]
        fn from(val: Wucksel) -> u8 {
            Wucksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wutf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
        ZERO = 0x01,
    }
    impl Wutf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wutf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wutf {
        #[inline(always)]
        fn from(val: u8) -> Wutf {
            Wutf::from_bits(val)
        }
    }
    impl From<Wutf> for u8 {
        #[inline(always)]
        fn from(val: Wutf) -> u8 {
            Wutf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wutmf {
        _RESERVED_0 = 0x0,
        #[doc = "This flag is set by hardware when the wakeup auto-reload counter reaches 0"]
        ZERO = 0x01,
    }
    impl Wutmf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wutmf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wutmf {
        #[inline(always)]
        fn from(val: u8) -> Wutmf {
            Wutmf::from_bits(val)
        }
    }
    impl From<Wutmf> for u8 {
        #[inline(always)]
        fn from(val: Wutmf) -> u8 {
            Wutmf::to_bits(val)
        }
    }
}
