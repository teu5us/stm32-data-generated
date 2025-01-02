#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "External interrupt/event controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exti {
    ptr: *mut u8,
}
unsafe impl Send for Exti {}
unsafe impl Sync for Exti {}
impl Exti {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Rising Trigger selection register"]
    #[inline(always)]
    pub const fn rtsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Falling Trigger selection register"]
    #[inline(always)]
    pub const fn ftsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 32usize) as _) }
    }
    #[doc = "Software interrupt event register"]
    #[inline(always)]
    pub const fn swier(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 32usize) as _) }
    }
    #[doc = "Rising pending register"]
    #[inline(always)]
    pub const fn rpr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 32usize) as _) }
    }
    #[doc = "Falling pending register"]
    #[inline(always)]
    pub const fn fpr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 32usize) as _) }
    }
    #[doc = "Security configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self, n: usize) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 36usize) as _) }
    }
    #[doc = "Privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self, n: usize) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 28usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "EXTI lock register"]
    #[inline(always)]
    pub const fn lockrg(self) -> crate::common::Reg<regs::Lockrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 16usize) as _) }
    }
    #[doc = "Event mask register"]
    #[inline(always)]
    pub const fn emr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize + n * 16usize) as _) }
    }
}
pub mod regs {
    #[doc = "external interrupt configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "EXTI configuration bits"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field(
                    "exti",
                    &[
                        self.exti(0usize),
                        self.exti(1usize),
                        self.exti(2usize),
                        self.exti(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Exticr {
                exti: [u8; 4usize],
            }
            let proxy = Exticr {
                exti: [
                    self.exti(0usize),
                    self.exti(1usize),
                    self.exti(2usize),
                    self.exti(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EXTI lines register, 1 bit per line"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lines(pub u32);
    impl Lines {
        #[doc = "EXTI line"]
        #[inline(always)]
        pub const fn line(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "EXTI line"]
        #[inline(always)]
        pub fn set_line(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Lines {
        #[inline(always)]
        fn default() -> Lines {
            Lines(0)
        }
    }
    impl core::fmt::Debug for Lines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lines")
                .field(
                    "line",
                    &[
                        self.line(0usize),
                        self.line(1usize),
                        self.line(2usize),
                        self.line(3usize),
                        self.line(4usize),
                        self.line(5usize),
                        self.line(6usize),
                        self.line(7usize),
                        self.line(8usize),
                        self.line(9usize),
                        self.line(10usize),
                        self.line(11usize),
                        self.line(12usize),
                        self.line(13usize),
                        self.line(14usize),
                        self.line(15usize),
                        self.line(16usize),
                        self.line(17usize),
                        self.line(18usize),
                        self.line(19usize),
                        self.line(20usize),
                        self.line(21usize),
                        self.line(22usize),
                        self.line(23usize),
                        self.line(24usize),
                        self.line(25usize),
                        self.line(26usize),
                        self.line(27usize),
                        self.line(28usize),
                        self.line(29usize),
                        self.line(30usize),
                        self.line(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lines {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Lines {
                line: [bool; 32usize],
            }
            let proxy = Lines {
                line: [
                    self.line(0usize),
                    self.line(1usize),
                    self.line(2usize),
                    self.line(3usize),
                    self.line(4usize),
                    self.line(5usize),
                    self.line(6usize),
                    self.line(7usize),
                    self.line(8usize),
                    self.line(9usize),
                    self.line(10usize),
                    self.line(11usize),
                    self.line(12usize),
                    self.line(13usize),
                    self.line(14usize),
                    self.line(15usize),
                    self.line(16usize),
                    self.line(17usize),
                    self.line(18usize),
                    self.line(19usize),
                    self.line(20usize),
                    self.line(21usize),
                    self.line(22usize),
                    self.line(23usize),
                    self.line(24usize),
                    self.line(25usize),
                    self.line(26usize),
                    self.line(27usize),
                    self.line(28usize),
                    self.line(29usize),
                    self.line(30usize),
                    self.line(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EXTI lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lockrg(pub u32);
    impl Lockrg {
        #[doc = "LOCK"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LOCK"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Lockrg {
        #[inline(always)]
        fn default() -> Lockrg {
            Lockrg(0)
        }
    }
    impl core::fmt::Debug for Lockrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lockrg").field("lock", &self.lock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockrg {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Lockrg {
                lock: bool,
            }
            let proxy = Lockrg { lock: self.lock() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Security enable on event input x"]
        #[inline(always)]
        pub const fn priv_(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Security enable on event input x"]
        #[inline(always)]
        pub fn set_priv_(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field(
                    "priv_",
                    &[
                        self.priv_(0usize),
                        self.priv_(1usize),
                        self.priv_(2usize),
                        self.priv_(3usize),
                        self.priv_(4usize),
                        self.priv_(5usize),
                        self.priv_(6usize),
                        self.priv_(7usize),
                        self.priv_(8usize),
                        self.priv_(9usize),
                        self.priv_(10usize),
                        self.priv_(11usize),
                        self.priv_(12usize),
                        self.priv_(13usize),
                        self.priv_(14usize),
                        self.priv_(15usize),
                        self.priv_(16usize),
                        self.priv_(17usize),
                        self.priv_(18usize),
                        self.priv_(19usize),
                        self.priv_(20usize),
                        self.priv_(21usize),
                        self.priv_(22usize),
                        self.priv_(23usize),
                        self.priv_(24usize),
                        self.priv_(25usize),
                        self.priv_(26usize),
                        self.priv_(27usize),
                        self.priv_(28usize),
                        self.priv_(29usize),
                        self.priv_(30usize),
                        self.priv_(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Privcfgr {
                priv_: [bool; 32usize],
            }
            let proxy = Privcfgr {
                priv_: [
                    self.priv_(0usize),
                    self.priv_(1usize),
                    self.priv_(2usize),
                    self.priv_(3usize),
                    self.priv_(4usize),
                    self.priv_(5usize),
                    self.priv_(6usize),
                    self.priv_(7usize),
                    self.priv_(8usize),
                    self.priv_(9usize),
                    self.priv_(10usize),
                    self.priv_(11usize),
                    self.priv_(12usize),
                    self.priv_(13usize),
                    self.priv_(14usize),
                    self.priv_(15usize),
                    self.priv_(16usize),
                    self.priv_(17usize),
                    self.priv_(18usize),
                    self.priv_(19usize),
                    self.priv_(20usize),
                    self.priv_(21usize),
                    self.priv_(22usize),
                    self.priv_(23usize),
                    self.priv_(24usize),
                    self.priv_(25usize),
                    self.priv_(26usize),
                    self.priv_(27usize),
                    self.priv_(28usize),
                    self.priv_(29usize),
                    self.priv_(30usize),
                    self.priv_(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Security configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Security enable on event input x"]
        #[inline(always)]
        pub const fn sec(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Security enable on event input x"]
        #[inline(always)]
        pub fn set_sec(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field(
                    "sec",
                    &[
                        self.sec(0usize),
                        self.sec(1usize),
                        self.sec(2usize),
                        self.sec(3usize),
                        self.sec(4usize),
                        self.sec(5usize),
                        self.sec(6usize),
                        self.sec(7usize),
                        self.sec(8usize),
                        self.sec(9usize),
                        self.sec(10usize),
                        self.sec(11usize),
                        self.sec(12usize),
                        self.sec(13usize),
                        self.sec(14usize),
                        self.sec(15usize),
                        self.sec(16usize),
                        self.sec(17usize),
                        self.sec(18usize),
                        self.sec(19usize),
                        self.sec(20usize),
                        self.sec(21usize),
                        self.sec(22usize),
                        self.sec(23usize),
                        self.sec(24usize),
                        self.sec(25usize),
                        self.sec(26usize),
                        self.sec(27usize),
                        self.sec(28usize),
                        self.sec(29usize),
                        self.sec(30usize),
                        self.sec(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Seccfgr {
                sec: [bool; 32usize],
            }
            let proxy = Seccfgr {
                sec: [
                    self.sec(0usize),
                    self.sec(1usize),
                    self.sec(2usize),
                    self.sec(3usize),
                    self.sec(4usize),
                    self.sec(5usize),
                    self.sec(6usize),
                    self.sec(7usize),
                    self.sec(8usize),
                    self.sec(9usize),
                    self.sec(10usize),
                    self.sec(11usize),
                    self.sec(12usize),
                    self.sec(13usize),
                    self.sec(14usize),
                    self.sec(15usize),
                    self.sec(16usize),
                    self.sec(17usize),
                    self.sec(18usize),
                    self.sec(19usize),
                    self.sec(20usize),
                    self.sec(21usize),
                    self.sec(22usize),
                    self.sec(23usize),
                    self.sec(24usize),
                    self.sec(25usize),
                    self.sec(26usize),
                    self.sec(27usize),
                    self.sec(28usize),
                    self.sec(29usize),
                    self.sec(30usize),
                    self.sec(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
