#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Reset and clock control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcc {
    ptr: *mut u8,
}
unsafe impl Send for Rcc {}
unsafe impl Sync for Rcc {}
impl Rcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn icscr(self) -> crate::common::Reg<regs::Icscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PLLSAI1 configuration register"]
    #[inline(always)]
    pub const fn pllsai1cfgr(self) -> crate::common::Reg<regs::Pllsaicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PLLSAI2 configuration register"]
    #[inline(always)]
    pub const fn pllsai2cfgr(self) -> crate::common::Reg<regs::Pllsaicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "APB1ENR1"]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "APB2ENR"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb1smenr(self) -> crate::common::Reg<regs::Ahb1smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb2smenr(self) -> crate::common::Reg<regs::Ahb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb3smenr(self) -> crate::common::Reg<regs::Ahb3smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "APB1SMENR1"]
    #[inline(always)]
    pub const fn apb1smenr1(self) -> crate::common::Reg<regs::Apb1smenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn apb1smenr2(self) -> crate::common::Reg<regs::Apb1smenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "APB2SMENR"]
    #[inline(always)]
    pub const fn apb2smenr(self) -> crate::common::Reg<regs::Apb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "CCIPR"]
    #[inline(always)]
    pub const fn ccipr(self) -> crate::common::Reg<regs::Ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "BDCR"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "CSR"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash memory interface clock enable"]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface clock enable"]
        #[inline(always)]
        pub fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing Controller clock enable"]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing Controller clock enable"]
        #[inline(always)]
        pub fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA2D clock enable"]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clock enable"]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    impl core::fmt::Debug for Ahb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1enr")
                .field("dma1en", &self.dma1en())
                .field("dma2en", &self.dma2en())
                .field("flashen", &self.flashen())
                .field("crcen", &self.crcen())
                .field("tscen", &self.tscen())
                .field("dma2den", &self.dma2den())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1enr {
                dma1en: bool,
                dma2en: bool,
                flashen: bool,
                crcen: bool,
                tscen: bool,
                dma2den: bool,
            }
            let proxy = Ahb1enr {
                dma1en: self.dma1en(),
                dma2en: self.dma2en(),
                flashen: self.flashen(),
                crcen: self.crcen(),
                tscen: self.tscen(),
                dma2den: self.dma2den(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "DMA1 reset"]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 reset"]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash memory interface reset"]
        #[inline(always)]
        pub const fn flashrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface reset"]
        #[inline(always)]
        pub fn set_flashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing Controller reset"]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing Controller reset"]
        #[inline(always)]
        pub fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA2D reset"]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D reset"]
        #[inline(always)]
        pub fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb1rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1rstr")
                .field("dma1rst", &self.dma1rst())
                .field("dma2rst", &self.dma2rst())
                .field("flashrst", &self.flashrst())
                .field("crcrst", &self.crcrst())
                .field("tscrst", &self.tscrst())
                .field("dma2drst", &self.dma2drst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1rstr {
                dma1rst: bool,
                dma2rst: bool,
                flashrst: bool,
                crcrst: bool,
                tscrst: bool,
                dma2drst: bool,
            }
            let proxy = Ahb1rstr {
                dma1rst: self.dma1rst(),
                dma2rst: self.dma2rst(),
                flashrst: self.flashrst(),
                crcrst: self.crcrst(),
                tscrst: self.tscrst(),
                dma2drst: self.dma2drst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1smenr(pub u32);
    impl Ahb1smenr {
        #[doc = "DMA1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dma1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dma2smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dma2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash memory interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn flashsmen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_flashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sram1smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sram1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CRCSMEN"]
        #[inline(always)]
        pub const fn crcsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRCSMEN"]
        #[inline(always)]
        pub fn set_crcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing Controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tscsmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing Controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tscsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA2D clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dma2dsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dma2dsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb1smenr {
        #[inline(always)]
        fn default() -> Ahb1smenr {
            Ahb1smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb1smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1smenr")
                .field("dma1smen", &self.dma1smen())
                .field("dma2smen", &self.dma2smen())
                .field("flashsmen", &self.flashsmen())
                .field("sram1smen", &self.sram1smen())
                .field("crcsmen", &self.crcsmen())
                .field("tscsmen", &self.tscsmen())
                .field("dma2dsmen", &self.dma2dsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1smenr {
                dma1smen: bool,
                dma2smen: bool,
                flashsmen: bool,
                sram1smen: bool,
                crcsmen: bool,
                tscsmen: bool,
                dma2dsmen: bool,
            }
            let proxy = Ahb1smenr {
                dma1smen: self.dma1smen(),
                dma2smen: self.dma2smen(),
                flashsmen: self.flashsmen(),
                sram1smen: self.sram1smen(),
                crcsmen: self.crcsmen(),
                tscsmen: self.tscsmen(),
                dma2dsmen: self.dma2dsmen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clock enable"]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clock enable"]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OTG full speed clock enable"]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OTG full speed clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ADC clock enable"]
        #[inline(always)]
        pub const fn adcen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clock enable"]
        #[inline(always)]
        pub fn set_adcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DCMI clock enable"]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI clock enable"]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AES accelerator clock enable"]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES accelerator clock enable"]
        #[inline(always)]
        pub fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random Number Generator clock enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator clock enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ahb2enr {
        #[inline(always)]
        fn default() -> Ahb2enr {
            Ahb2enr(0)
        }
    }
    impl core::fmt::Debug for Ahb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr")
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("gpioien", &self.gpioien())
                .field("usb_otg_fsen", &self.usb_otg_fsen())
                .field("adcen", &self.adcen())
                .field("dcmien", &self.dcmien())
                .field("aesen", &self.aesen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2enr {
                gpioaen: bool,
                gpioben: bool,
                gpiocen: bool,
                gpioden: bool,
                gpioeen: bool,
                gpiofen: bool,
                gpiogen: bool,
                gpiohen: bool,
                gpioien: bool,
                usb_otg_fsen: bool,
                adcen: bool,
                dcmien: bool,
                aesen: bool,
                hashen: bool,
                rngen: bool,
            }
            let proxy = Ahb2enr {
                gpioaen: self.gpioaen(),
                gpioben: self.gpioben(),
                gpiocen: self.gpiocen(),
                gpioden: self.gpioden(),
                gpioeen: self.gpioeen(),
                gpiofen: self.gpiofen(),
                gpiogen: self.gpiogen(),
                gpiohen: self.gpiohen(),
                gpioien: self.gpioien(),
                usb_otg_fsen: self.usb_otg_fsen(),
                adcen: self.adcen(),
                dcmien: self.dcmien(),
                aesen: self.aesen(),
                hashen: self.hashen(),
                rngen: self.rngen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I reset"]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I reset"]
        #[inline(always)]
        pub fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USB OTG FS reset"]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG FS reset"]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ADC reset"]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC reset"]
        #[inline(always)]
        pub fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Digital Camera Interface reset"]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Digital Camera Interface reset"]
        #[inline(always)]
        pub fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AES hardware accelerator reset"]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES hardware accelerator reset"]
        #[inline(always)]
        pub fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hash reset"]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hash reset"]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator reset"]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator reset"]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ahb2rstr {
        #[inline(always)]
        fn default() -> Ahb2rstr {
            Ahb2rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr")
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("gpioirst", &self.gpioirst())
                .field("usb_otg_fsrst", &self.usb_otg_fsrst())
                .field("adcrst", &self.adcrst())
                .field("dcmirst", &self.dcmirst())
                .field("aesrst", &self.aesrst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2rstr {
                gpioarst: bool,
                gpiobrst: bool,
                gpiocrst: bool,
                gpiodrst: bool,
                gpioerst: bool,
                gpiofrst: bool,
                gpiogrst: bool,
                gpiohrst: bool,
                gpioirst: bool,
                usb_otg_fsrst: bool,
                adcrst: bool,
                dcmirst: bool,
                aesrst: bool,
                hashrst: bool,
                rngrst: bool,
            }
            let proxy = Ahb2rstr {
                gpioarst: self.gpioarst(),
                gpiobrst: self.gpiobrst(),
                gpiocrst: self.gpiocrst(),
                gpiodrst: self.gpiodrst(),
                gpioerst: self.gpioerst(),
                gpiofrst: self.gpiofrst(),
                gpiogrst: self.gpiogrst(),
                gpiohrst: self.gpiohrst(),
                gpioirst: self.gpioirst(),
                usb_otg_fsrst: self.usb_otg_fsrst(),
                adcrst: self.adcrst(),
                dcmirst: self.dcmirst(),
                aesrst: self.aesrst(),
                hashrst: self.hashrst(),
                rngrst: self.rngrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2smenr(pub u32);
    impl Ahb2smenr {
        #[doc = "IO port A clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpioasmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpioasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiobsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiobsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiocsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiocsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiodsmen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiodsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpioesmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpioesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiofsmen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiofsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiogsmen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiogsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiohsmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiohsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpioismen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpioismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sram2smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sram2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OTG full speed clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usb_otg_fssmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OTG full speed clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usb_otg_fssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ADC clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn adcfssmen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_adcfssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DCMI clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dcmismen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dcmismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AES accelerator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn aessmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES accelerator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_aessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn hashsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_hashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random Number Generator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn rngsmen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_rngsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Ahb2smenr {
        #[inline(always)]
        fn default() -> Ahb2smenr {
            Ahb2smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb2smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2smenr")
                .field("gpioasmen", &self.gpioasmen())
                .field("gpiobsmen", &self.gpiobsmen())
                .field("gpiocsmen", &self.gpiocsmen())
                .field("gpiodsmen", &self.gpiodsmen())
                .field("gpioesmen", &self.gpioesmen())
                .field("gpiofsmen", &self.gpiofsmen())
                .field("gpiogsmen", &self.gpiogsmen())
                .field("gpiohsmen", &self.gpiohsmen())
                .field("gpioismen", &self.gpioismen())
                .field("sram2smen", &self.sram2smen())
                .field("usb_otg_fssmen", &self.usb_otg_fssmen())
                .field("adcfssmen", &self.adcfssmen())
                .field("dcmismen", &self.dcmismen())
                .field("aessmen", &self.aessmen())
                .field("hashsmen", &self.hashsmen())
                .field("rngsmen", &self.rngsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2smenr {
                gpioasmen: bool,
                gpiobsmen: bool,
                gpiocsmen: bool,
                gpiodsmen: bool,
                gpioesmen: bool,
                gpiofsmen: bool,
                gpiogsmen: bool,
                gpiohsmen: bool,
                gpioismen: bool,
                sram2smen: bool,
                usb_otg_fssmen: bool,
                adcfssmen: bool,
                dcmismen: bool,
                aessmen: bool,
                hashsmen: bool,
                rngsmen: bool,
            }
            let proxy = Ahb2smenr {
                gpioasmen: self.gpioasmen(),
                gpiobsmen: self.gpiobsmen(),
                gpiocsmen: self.gpiocsmen(),
                gpiodsmen: self.gpiodsmen(),
                gpioesmen: self.gpioesmen(),
                gpiofsmen: self.gpiofsmen(),
                gpiogsmen: self.gpiogsmen(),
                gpiohsmen: self.gpiohsmen(),
                gpioismen: self.gpioismen(),
                sram2smen: self.sram2smen(),
                usb_otg_fssmen: self.usb_otg_fssmen(),
                adcfssmen: self.adcfssmen(),
                dcmismen: self.dcmismen(),
                aessmen: self.aessmen(),
                hashsmen: self.hashsmen(),
                rngsmen: self.rngsmen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "Flexible memory controller clock enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller clock enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "QUADSPIEN"]
        #[inline(always)]
        pub const fn quadspien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPIEN"]
        #[inline(always)]
        pub fn set_quadspien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    impl core::fmt::Debug for Ahb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3enr")
                .field("fmcen", &self.fmcen())
                .field("quadspien", &self.quadspien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3enr {
                fmcen: bool,
                quadspien: bool,
            }
            let proxy = Ahb3enr {
                fmcen: self.fmcen(),
                quadspien: self.quadspien(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "Flexible memory controller reset"]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller reset"]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Quad SPI memory interface reset"]
        #[inline(always)]
        pub const fn quadspirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Quad SPI memory interface reset"]
        #[inline(always)]
        pub fn set_quadspirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ahb3rstr {
        #[inline(always)]
        fn default() -> Ahb3rstr {
            Ahb3rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb3rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3rstr")
                .field("fmcrst", &self.fmcrst())
                .field("quadspirst", &self.quadspirst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3rstr {
                fmcrst: bool,
                quadspirst: bool,
            }
            let proxy = Ahb3rstr {
                fmcrst: self.fmcrst(),
                quadspirst: self.quadspirst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3smenr(pub u32);
    impl Ahb3smenr {
        #[doc = "Flexible memory controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn fmcsmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_fmcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "QUADSPISMEN"]
        #[inline(always)]
        pub const fn quadspismen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPISMEN"]
        #[inline(always)]
        pub fn set_quadspismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ahb3smenr {
        #[inline(always)]
        fn default() -> Ahb3smenr {
            Ahb3smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb3smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3smenr")
                .field("fmcsmen", &self.fmcsmen())
                .field("quadspismen", &self.quadspismen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3smenr {
                fmcsmen: bool,
                quadspismen: bool,
            }
            let proxy = Ahb3smenr {
                fmcsmen: self.fmcsmen(),
                quadspismen: self.quadspismen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1ENR1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIM2 timer clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 timer clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 timer clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 timer clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 timer clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 timer clock enable"]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 timer clock enable"]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 timer clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 timer clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 timer clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 timer clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LCD clock enable"]
        #[inline(always)]
        pub const fn lcden(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LCD clock enable"]
        #[inline(always)]
        pub fn set_lcden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RTC APB clock enable"]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable"]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable"]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable"]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clock Recovery System clock enable"]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System clock enable"]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN1 clock enable"]
        #[inline(always)]
        pub const fn can1en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 clock enable"]
        #[inline(always)]
        pub fn set_can1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2 clock enable"]
        #[inline(always)]
        pub const fn can2en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 clock enable"]
        #[inline(always)]
        pub fn set_can2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB FS clock enable"]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB FS clock enable"]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC1 interface clock enable"]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 interface clock enable"]
        #[inline(always)]
        pub fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPAMP interface clock enable"]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP interface clock enable"]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low power timer 1 clock enable"]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer 1 clock enable"]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1enr1 {
        #[inline(always)]
        fn default() -> Apb1enr1 {
            Apb1enr1(0)
        }
    }
    impl core::fmt::Debug for Apb1enr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr1")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("lcden", &self.lcden())
                .field("rtcapben", &self.rtcapben())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i2c3en", &self.i2c3en())
                .field("crsen", &self.crsen())
                .field("can1en", &self.can1en())
                .field("can2en", &self.can2en())
                .field("usben", &self.usben())
                .field("pwren", &self.pwren())
                .field("dac1en", &self.dac1en())
                .field("opampen", &self.opampen())
                .field("lptim1en", &self.lptim1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1enr1 {
                tim2en: bool,
                tim3en: bool,
                tim4en: bool,
                tim5en: bool,
                tim6en: bool,
                tim7en: bool,
                lcden: bool,
                rtcapben: bool,
                wwdgen: bool,
                spi2en: bool,
                spi3en: bool,
                usart2en: bool,
                usart3en: bool,
                uart4en: bool,
                uart5en: bool,
                i2c1en: bool,
                i2c2en: bool,
                i2c3en: bool,
                crsen: bool,
                can1en: bool,
                can2en: bool,
                usben: bool,
                pwren: bool,
                dac1en: bool,
                opampen: bool,
                lptim1en: bool,
            }
            let proxy = Apb1enr1 {
                tim2en: self.tim2en(),
                tim3en: self.tim3en(),
                tim4en: self.tim4en(),
                tim5en: self.tim5en(),
                tim6en: self.tim6en(),
                tim7en: self.tim7en(),
                lcden: self.lcden(),
                rtcapben: self.rtcapben(),
                wwdgen: self.wwdgen(),
                spi2en: self.spi2en(),
                spi3en: self.spi3en(),
                usart2en: self.usart2en(),
                usart3en: self.usart3en(),
                uart4en: self.uart4en(),
                uart5en: self.uart5en(),
                i2c1en: self.i2c1en(),
                i2c2en: self.i2c2en(),
                i2c3en: self.i2c3en(),
                crsen: self.crsen(),
                can1en: self.can1en(),
                can2en: self.can2en(),
                usben: self.usben(),
                pwren: self.pwren(),
                dac1en: self.dac1en(),
                opampen: self.opampen(),
                lptim1en: self.lptim1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1 peripheral clock enable register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "Low power UART 1 clock enable"]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low power UART 1 clock enable"]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4 clock enable"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Single wire protocol clock enable"]
        #[inline(always)]
        pub const fn swpmi1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Single wire protocol clock enable"]
        #[inline(always)]
        pub fn set_swpmi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LPTIM2EN"]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2EN"]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Apb1enr2 {
        #[inline(always)]
        fn default() -> Apb1enr2 {
            Apb1enr2(0)
        }
    }
    impl core::fmt::Debug for Apb1enr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr2")
                .field("lpuart1en", &self.lpuart1en())
                .field("i2c4en", &self.i2c4en())
                .field("swpmi1en", &self.swpmi1en())
                .field("lptim2en", &self.lptim2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1enr2 {
                lpuart1en: bool,
                i2c4en: bool,
                swpmi1en: bool,
                lptim2en: bool,
            }
            let proxy = Apb1enr2 {
                lpuart1en: self.lpuart1en(),
                i2c4en: self.i2c4en(),
                swpmi1en: self.swpmi1en(),
                lptim2en: self.lptim2en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1 peripheral reset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIM2 timer reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 timer reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 timer reset"]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 timer reset"]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 timer reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 timer reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 timer reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 timer reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LCD interface reset"]
        #[inline(always)]
        pub const fn lcdrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LCD interface reset"]
        #[inline(always)]
        pub fn set_lcdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 reset"]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset"]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 reset"]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 reset"]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 reset"]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 reset"]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART4 reset."]
        #[inline(always)]
        pub const fn usart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 reset."]
        #[inline(always)]
        pub fn set_usart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 reset"]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 reset"]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS reset"]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS reset"]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN1 reset"]
        #[inline(always)]
        pub const fn can1rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 reset"]
        #[inline(always)]
        pub fn set_can1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2 reset"]
        #[inline(always)]
        pub const fn can2rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 reset"]
        #[inline(always)]
        pub fn set_can2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB FS reset"]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB FS reset"]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub const fn pwrrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub fn set_pwrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC1 interface reset"]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 interface reset"]
        #[inline(always)]
        pub fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPAMP interface reset"]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP interface reset"]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low Power Timer 1 reset"]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low Power Timer 1 reset"]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1rstr1 {
        #[inline(always)]
        fn default() -> Apb1rstr1 {
            Apb1rstr1(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr1")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim4rst", &self.tim4rst())
                .field("tim5rst", &self.tim5rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("lcdrst", &self.lcdrst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("usart4rst", &self.usart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("crsrst", &self.crsrst())
                .field("can1rst", &self.can1rst())
                .field("can2rst", &self.can2rst())
                .field("usbrst", &self.usbrst())
                .field("pwrrst", &self.pwrrst())
                .field("dac1rst", &self.dac1rst())
                .field("opamprst", &self.opamprst())
                .field("lptim1rst", &self.lptim1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1rstr1 {
                tim2rst: bool,
                tim3rst: bool,
                tim4rst: bool,
                tim5rst: bool,
                tim6rst: bool,
                tim7rst: bool,
                lcdrst: bool,
                spi2rst: bool,
                spi3rst: bool,
                usart2rst: bool,
                usart3rst: bool,
                uart4rst: bool,
                usart4rst: bool,
                uart5rst: bool,
                i2c1rst: bool,
                i2c2rst: bool,
                i2c3rst: bool,
                crsrst: bool,
                can1rst: bool,
                can2rst: bool,
                usbrst: bool,
                pwrrst: bool,
                dac1rst: bool,
                opamprst: bool,
                lptim1rst: bool,
            }
            let proxy = Apb1rstr1 {
                tim2rst: self.tim2rst(),
                tim3rst: self.tim3rst(),
                tim4rst: self.tim4rst(),
                tim5rst: self.tim5rst(),
                tim6rst: self.tim6rst(),
                tim7rst: self.tim7rst(),
                lcdrst: self.lcdrst(),
                spi2rst: self.spi2rst(),
                spi3rst: self.spi3rst(),
                usart2rst: self.usart2rst(),
                usart3rst: self.usart3rst(),
                uart4rst: self.uart4rst(),
                usart4rst: self.usart4rst(),
                uart5rst: self.uart5rst(),
                i2c1rst: self.i2c1rst(),
                i2c2rst: self.i2c2rst(),
                i2c3rst: self.i2c3rst(),
                crsrst: self.crsrst(),
                can1rst: self.can1rst(),
                can2rst: self.can2rst(),
                usbrst: self.usbrst(),
                pwrrst: self.pwrrst(),
                dac1rst: self.dac1rst(),
                opamprst: self.opamprst(),
                lptim1rst: self.lptim1rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1 peripheral reset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "Low-power UART 1 reset"]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power UART 1 reset"]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4 reset"]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 reset"]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Single wire protocol reset"]
        #[inline(always)]
        pub const fn swpmi1rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Single wire protocol reset"]
        #[inline(always)]
        pub fn set_swpmi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Low-power timer 2 reset"]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power timer 2 reset"]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Apb1rstr2 {
        #[inline(always)]
        fn default() -> Apb1rstr2 {
            Apb1rstr2(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr2")
                .field("lpuart1rst", &self.lpuart1rst())
                .field("i2c4rst", &self.i2c4rst())
                .field("swpmi1rst", &self.swpmi1rst())
                .field("lptim2rst", &self.lptim2rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1rstr2 {
                lpuart1rst: bool,
                i2c4rst: bool,
                swpmi1rst: bool,
                lptim2rst: bool,
            }
            let proxy = Apb1rstr2 {
                lpuart1rst: self.lpuart1rst(),
                i2c4rst: self.i2c4rst(),
                swpmi1rst: self.swpmi1rst(),
                lptim2rst: self.lptim2rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1SMENR1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr1(pub u32);
    impl Apb1smenr1 {
        #[doc = "TIM2 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim2smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim3smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim4smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim5smen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim6smen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim7smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim7smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LCD clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn lcdsmen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LCD clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_lcdsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RTC APB clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn rtcapbsmen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_rtcapbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn wwdgsmen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_wwdgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn spi2smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_spi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sp3smen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sp3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usart2smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usart2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usart3smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usart3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn uart4smen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_uart4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn uart5smen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_uart5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c3smen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn crssmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_crssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn can1smen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_can1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn can2smen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_can2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB FS clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usbsmen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB FS clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Power interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn pwrsmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_pwrsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dac1smen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dac1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPAMP interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn opampsmen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_opampsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low power timer 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn lptim1smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_lptim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1smenr1 {
        #[inline(always)]
        fn default() -> Apb1smenr1 {
            Apb1smenr1(0)
        }
    }
    impl core::fmt::Debug for Apb1smenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1smenr1")
                .field("tim2smen", &self.tim2smen())
                .field("tim3smen", &self.tim3smen())
                .field("tim4smen", &self.tim4smen())
                .field("tim5smen", &self.tim5smen())
                .field("tim6smen", &self.tim6smen())
                .field("tim7smen", &self.tim7smen())
                .field("lcdsmen", &self.lcdsmen())
                .field("rtcapbsmen", &self.rtcapbsmen())
                .field("wwdgsmen", &self.wwdgsmen())
                .field("spi2smen", &self.spi2smen())
                .field("sp3smen", &self.sp3smen())
                .field("usart2smen", &self.usart2smen())
                .field("usart3smen", &self.usart3smen())
                .field("uart4smen", &self.uart4smen())
                .field("uart5smen", &self.uart5smen())
                .field("i2c1smen", &self.i2c1smen())
                .field("i2c2smen", &self.i2c2smen())
                .field("i2c3smen", &self.i2c3smen())
                .field("crssmen", &self.crssmen())
                .field("can1smen", &self.can1smen())
                .field("can2smen", &self.can2smen())
                .field("usbsmen", &self.usbsmen())
                .field("pwrsmen", &self.pwrsmen())
                .field("dac1smen", &self.dac1smen())
                .field("opampsmen", &self.opampsmen())
                .field("lptim1smen", &self.lptim1smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1smenr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1smenr1 {
                tim2smen: bool,
                tim3smen: bool,
                tim4smen: bool,
                tim5smen: bool,
                tim6smen: bool,
                tim7smen: bool,
                lcdsmen: bool,
                rtcapbsmen: bool,
                wwdgsmen: bool,
                spi2smen: bool,
                sp3smen: bool,
                usart2smen: bool,
                usart3smen: bool,
                uart4smen: bool,
                uart5smen: bool,
                i2c1smen: bool,
                i2c2smen: bool,
                i2c3smen: bool,
                crssmen: bool,
                can1smen: bool,
                can2smen: bool,
                usbsmen: bool,
                pwrsmen: bool,
                dac1smen: bool,
                opampsmen: bool,
                lptim1smen: bool,
            }
            let proxy = Apb1smenr1 {
                tim2smen: self.tim2smen(),
                tim3smen: self.tim3smen(),
                tim4smen: self.tim4smen(),
                tim5smen: self.tim5smen(),
                tim6smen: self.tim6smen(),
                tim7smen: self.tim7smen(),
                lcdsmen: self.lcdsmen(),
                rtcapbsmen: self.rtcapbsmen(),
                wwdgsmen: self.wwdgsmen(),
                spi2smen: self.spi2smen(),
                sp3smen: self.sp3smen(),
                usart2smen: self.usart2smen(),
                usart3smen: self.usart3smen(),
                uart4smen: self.uart4smen(),
                uart5smen: self.uart5smen(),
                i2c1smen: self.i2c1smen(),
                i2c2smen: self.i2c2smen(),
                i2c3smen: self.i2c3smen(),
                crssmen: self.crssmen(),
                can1smen: self.can1smen(),
                can2smen: self.can2smen(),
                usbsmen: self.usbsmen(),
                pwrsmen: self.pwrsmen(),
                dac1smen: self.dac1smen(),
                opampsmen: self.opampsmen(),
                lptim1smen: self.lptim1smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr2(pub u32);
    impl Apb1smenr2 {
        #[doc = "Low power UART 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn lpuart1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low power UART 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_lpuart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c4smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Single wire protocol clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn swpmi1smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Single wire protocol clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_swpmi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LPTIM2SMEN"]
        #[inline(always)]
        pub const fn lptim2smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2SMEN"]
        #[inline(always)]
        pub fn set_lptim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Apb1smenr2 {
        #[inline(always)]
        fn default() -> Apb1smenr2 {
            Apb1smenr2(0)
        }
    }
    impl core::fmt::Debug for Apb1smenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1smenr2")
                .field("lpuart1smen", &self.lpuart1smen())
                .field("i2c4smen", &self.i2c4smen())
                .field("swpmi1smen", &self.swpmi1smen())
                .field("lptim2smen", &self.lptim2smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1smenr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1smenr2 {
                lpuart1smen: bool,
                i2c4smen: bool,
                swpmi1smen: bool,
                lptim2smen: bool,
            }
            let proxy = Apb1smenr2 {
                lpuart1smen: self.lpuart1smen(),
                i2c4smen: self.i2c4smen(),
                swpmi1smen: self.swpmi1smen(),
                lptim2smen: self.lptim2smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB2ENR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "SYSCFG clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Firewall clock enable"]
        #[inline(always)]
        pub const fn fwen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Firewall clock enable"]
        #[inline(always)]
        pub fn set_fwen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SDMMC clock enable"]
        #[inline(always)]
        pub const fn sdmmcen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC clock enable"]
        #[inline(always)]
        pub fn set_sdmmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 timer clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clock enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clock enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 timer clock enable"]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 timer clock enable"]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 timer clock enable"]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 timer clock enable"]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 timer clock enable"]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 timer clock enable"]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clock enable"]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable"]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clock enable"]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clock enable"]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DFSDMEN enable"]
        #[inline(always)]
        pub const fn dfsdmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDMEN enable"]
        #[inline(always)]
        pub fn set_dfsdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    impl core::fmt::Debug for Apb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2enr")
                .field("syscfgen", &self.syscfgen())
                .field("fwen", &self.fwen())
                .field("sdmmcen", &self.sdmmcen())
                .field("tim1en", &self.tim1en())
                .field("spi1en", &self.spi1en())
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("sai1en", &self.sai1en())
                .field("sai2en", &self.sai2en())
                .field("dfsdmen", &self.dfsdmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2enr {
                syscfgen: bool,
                fwen: bool,
                sdmmcen: bool,
                tim1en: bool,
                spi1en: bool,
                tim8en: bool,
                usart1en: bool,
                tim15en: bool,
                tim16en: bool,
                tim17en: bool,
                sai1en: bool,
                sai2en: bool,
                dfsdmen: bool,
            }
            let proxy = Apb2enr {
                syscfgen: self.syscfgen(),
                fwen: self.fwen(),
                sdmmcen: self.sdmmcen(),
                tim1en: self.tim1en(),
                spi1en: self.spi1en(),
                tim8en: self.tim8en(),
                usart1en: self.usart1en(),
                tim15en: self.tim15en(),
                tim16en: self.tim16en(),
                tim17en: self.tim17en(),
                sai1en: self.sai1en(),
                sai2en: self.sai2en(),
                dfsdmen: self.dfsdmen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "System configuration (SYSCFG) reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration (SYSCFG) reset"]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SDMMC reset"]
        #[inline(always)]
        pub const fn sdmmcrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC reset"]
        #[inline(always)]
        pub fn set_sdmmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 timer reset"]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer reset"]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset"]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset"]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer reset"]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer reset"]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 timer reset"]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 timer reset"]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 timer reset"]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 timer reset"]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 timer reset"]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 timer reset"]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Serial audio interface 1 (SAI1) reset"]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Serial audio interface 1 (SAI1) reset"]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Serial audio interface 2 (SAI2) reset"]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Serial audio interface 2 (SAI2) reset"]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DFSDM filter reset"]
        #[inline(always)]
        pub const fn dfsdmrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM filter reset"]
        #[inline(always)]
        pub fn set_dfsdmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    impl core::fmt::Debug for Apb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2rstr")
                .field("syscfgrst", &self.syscfgrst())
                .field("sdmmcrst", &self.sdmmcrst())
                .field("tim1rst", &self.tim1rst())
                .field("spi1rst", &self.spi1rst())
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("sai1rst", &self.sai1rst())
                .field("sai2rst", &self.sai2rst())
                .field("dfsdmrst", &self.dfsdmrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2rstr {
                syscfgrst: bool,
                sdmmcrst: bool,
                tim1rst: bool,
                spi1rst: bool,
                tim8rst: bool,
                usart1rst: bool,
                tim15rst: bool,
                tim16rst: bool,
                tim17rst: bool,
                sai1rst: bool,
                sai2rst: bool,
                dfsdmrst: bool,
            }
            let proxy = Apb2rstr {
                syscfgrst: self.syscfgrst(),
                sdmmcrst: self.sdmmcrst(),
                tim1rst: self.tim1rst(),
                spi1rst: self.spi1rst(),
                tim8rst: self.tim8rst(),
                usart1rst: self.usart1rst(),
                tim15rst: self.tim15rst(),
                tim16rst: self.tim16rst(),
                tim17rst: self.tim17rst(),
                sai1rst: self.sai1rst(),
                sai2rst: self.sai2rst(),
                dfsdmrst: self.dfsdmrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB2SMENR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2smenr(pub u32);
    impl Apb2smenr {
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn syscfgsmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_syscfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SDMMC clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sdmmcsmen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sdmmcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn spi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_spi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim8smen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim8smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usart1smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim15smen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim15smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim16smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim16smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim17smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim17smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sai1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sai1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sai2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sai2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DFSDM timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dfsdmsmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dfsdmsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Apb2smenr {
        #[inline(always)]
        fn default() -> Apb2smenr {
            Apb2smenr(0)
        }
    }
    impl core::fmt::Debug for Apb2smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2smenr")
                .field("syscfgsmen", &self.syscfgsmen())
                .field("sdmmcsmen", &self.sdmmcsmen())
                .field("tim1smen", &self.tim1smen())
                .field("spi1smen", &self.spi1smen())
                .field("tim8smen", &self.tim8smen())
                .field("usart1smen", &self.usart1smen())
                .field("tim15smen", &self.tim15smen())
                .field("tim16smen", &self.tim16smen())
                .field("tim17smen", &self.tim17smen())
                .field("sai1smen", &self.sai1smen())
                .field("sai2smen", &self.sai2smen())
                .field("dfsdmsmen", &self.dfsdmsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2smenr {
                syscfgsmen: bool,
                sdmmcsmen: bool,
                tim1smen: bool,
                spi1smen: bool,
                tim8smen: bool,
                usart1smen: bool,
                tim15smen: bool,
                tim16smen: bool,
                tim17smen: bool,
                sai1smen: bool,
                sai2smen: bool,
                dfsdmsmen: bool,
            }
            let proxy = Apb2smenr {
                syscfgsmen: self.syscfgsmen(),
                sdmmcsmen: self.sdmmcsmen(),
                tim1smen: self.tim1smen(),
                spi1smen: self.spi1smen(),
                tim8smen: self.tim8smen(),
                usart1smen: self.usart1smen(),
                tim15smen: self.tim15smen(),
                tim16smen: self.tim16smen(),
                tim17smen: self.tim17smen(),
                sai1smen: self.sai1smen(),
                sai2smen: self.sai2smen(),
                dfsdmsmen: self.dfsdmsmen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "BDCR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enable"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enable"]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready"]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready"]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass"]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SE oscillator drive capability"]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "SE oscillator drive capability"]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSECSSON"]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSECSSON"]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSECSSD"]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSECSSD"]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain software reset"]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset"]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Low speed clock output enable"]
        #[inline(always)]
        pub const fn lscoen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low speed clock output enable"]
        #[inline(always)]
        pub fn set_lscoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Low speed clock output selection"]
        #[inline(always)]
        pub const fn lscosel(&self) -> super::vals::Lscosel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Lscosel::from_bits(val as u8)
        }
        #[doc = "Low speed clock output selection"]
        #[inline(always)]
        pub fn set_lscosel(&mut self, val: super::vals::Lscosel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    impl core::fmt::Debug for Bdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr")
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("lsedrv", &self.lsedrv())
                .field("lsecsson", &self.lsecsson())
                .field("lsecssd", &self.lsecssd())
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("bdrst", &self.bdrst())
                .field("lscoen", &self.lscoen())
                .field("lscosel", &self.lscosel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Bdcr {
                lseon: bool,
                lserdy: bool,
                lsebyp: bool,
                lsedrv: super::vals::Lsedrv,
                lsecsson: bool,
                lsecssd: bool,
                rtcsel: super::vals::Rtcsel,
                rtcen: bool,
                bdrst: bool,
                lscoen: bool,
                lscosel: super::vals::Lscosel,
            }
            let proxy = Bdcr {
                lseon: self.lseon(),
                lserdy: self.lserdy(),
                lsebyp: self.lsebyp(),
                lsedrv: self.lsedrv(),
                lsecsson: self.lsecsson(),
                lsecssd: self.lsecssd(),
                rtcsel: self.rtcsel(),
                rtcen: self.rtcen(),
                bdrst: self.bdrst(),
                lscoen: self.lscoen(),
                lscosel: self.lscosel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CCIPR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr(pub u32);
    impl Ccipr {
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub fn set_usart2sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USART3 clock source selection"]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART3 clock source selection"]
        #[inline(always)]
        pub fn set_usart3sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "UART4 clock source selection"]
        #[inline(always)]
        pub const fn uart4sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "UART4 clock source selection"]
        #[inline(always)]
        pub fn set_uart4sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "UART5 clock source selection"]
        #[inline(always)]
        pub const fn uart5sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "UART5 clock source selection"]
        #[inline(always)]
        pub fn set_uart5sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LPUART1 clock source selection"]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuart1sel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Lpuart1sel::from_bits(val as u8)
        }
        #[doc = "LPUART1 clock source selection"]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuart1sel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2c1sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2c1sel::from_bits(val as u8)
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: super::vals::I2c1sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::I2c2sel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::I2c2sel::from_bits(val as u8)
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub fn set_i2c2sel(&mut self, val: super::vals::I2c2sel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::I2c3sel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::I2c3sel::from_bits(val as u8)
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub fn set_i2c3sel(&mut self, val: super::vals::I2c3sel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Low power timer 1 clock source selection"]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "Low power timer 1 clock source selection"]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Low power timer 2 clock source selection"]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "Low power timer 2 clock source selection"]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "SAI1 clock source selection"]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Sai1sel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Sai1sel::from_bits(val as u8)
        }
        #[doc = "SAI1 clock source selection"]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Sai1sel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "SAI2 clock source selection"]
        #[inline(always)]
        pub const fn sai2sel(&self) -> super::vals::Sai2sel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Sai2sel::from_bits(val as u8)
        }
        #[doc = "SAI2 clock source selection"]
        #[inline(always)]
        pub fn set_sai2sel(&mut self, val: super::vals::Sai2sel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "48 MHz clock source selection"]
        #[inline(always)]
        pub const fn clk48sel(&self) -> super::vals::Clk48sel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Clk48sel::from_bits(val as u8)
        }
        #[doc = "48 MHz clock source selection"]
        #[inline(always)]
        pub fn set_clk48sel(&mut self, val: super::vals::Clk48sel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "ADCs clock source selection"]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "ADCs clock source selection"]
        #[inline(always)]
        pub fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "SWPMI1 clock source selection"]
        #[inline(always)]
        pub const fn swpmi1sel(&self) -> super::vals::Swpmi1sel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Swpmi1sel::from_bits(val as u8)
        }
        #[doc = "SWPMI1 clock source selection"]
        #[inline(always)]
        pub fn set_swpmi1sel(&mut self, val: super::vals::Swpmi1sel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "DFSDM clock source selection"]
        #[inline(always)]
        pub const fn dfsdmsel(&self) -> super::vals::Dfsdmsel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Dfsdmsel::from_bits(val as u8)
        }
        #[doc = "DFSDM clock source selection"]
        #[inline(always)]
        pub fn set_dfsdmsel(&mut self, val: super::vals::Dfsdmsel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ccipr {
        #[inline(always)]
        fn default() -> Ccipr {
            Ccipr(0)
        }
    }
    impl core::fmt::Debug for Ccipr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr")
                .field("usart1sel", &self.usart1sel())
                .field("usart2sel", &self.usart2sel())
                .field("usart3sel", &self.usart3sel())
                .field("uart4sel", &self.uart4sel())
                .field("uart5sel", &self.uart5sel())
                .field("lpuart1sel", &self.lpuart1sel())
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c2sel", &self.i2c2sel())
                .field("i2c3sel", &self.i2c3sel())
                .field("lptim1sel", &self.lptim1sel())
                .field("lptim2sel", &self.lptim2sel())
                .field("sai1sel", &self.sai1sel())
                .field("sai2sel", &self.sai2sel())
                .field("clk48sel", &self.clk48sel())
                .field("adcsel", &self.adcsel())
                .field("swpmi1sel", &self.swpmi1sel())
                .field("dfsdmsel", &self.dfsdmsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr {
                usart1sel: super::vals::Usart1sel,
                usart2sel: super::vals::Usartsel,
                usart3sel: super::vals::Usartsel,
                uart4sel: super::vals::Usartsel,
                uart5sel: super::vals::Usartsel,
                lpuart1sel: super::vals::Lpuart1sel,
                i2c1sel: super::vals::I2c1sel,
                i2c2sel: super::vals::I2c2sel,
                i2c3sel: super::vals::I2c3sel,
                lptim1sel: super::vals::Lptim1sel,
                lptim2sel: super::vals::Lptim2sel,
                sai1sel: super::vals::Sai1sel,
                sai2sel: super::vals::Sai2sel,
                clk48sel: super::vals::Clk48sel,
                adcsel: super::vals::Adcsel,
                swpmi1sel: super::vals::Swpmi1sel,
                dfsdmsel: super::vals::Dfsdmsel,
            }
            let proxy = Ccipr {
                usart1sel: self.usart1sel(),
                usart2sel: self.usart2sel(),
                usart3sel: self.usart3sel(),
                uart4sel: self.uart4sel(),
                uart5sel: self.uart5sel(),
                lpuart1sel: self.lpuart1sel(),
                i2c1sel: self.i2c1sel(),
                i2c2sel: self.i2c2sel(),
                i2c3sel: self.i2c3sel(),
                lptim1sel: self.lptim1sel(),
                lptim2sel: self.lptim2sel(),
                sai1sel: self.sai1sel(),
                sai2sel: self.sai2sel(),
                clk48sel: self.clk48sel(),
                adcsel: self.adcsel(),
                swpmi1sel: self.swpmi1sel(),
                dfsdmsel: self.dfsdmsel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 11usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
        }
        #[doc = "Wakeup from Stop and CSS backup clock selection"]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "Wakeup from Stop and CSS backup clock selection"]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Microcontroller clock output selection"]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output selection"]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Microcontroller clock output prescaler"]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output prescaler"]
        #[inline(always)]
        pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("hpre", &self.hpre())
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("stopwuck", &self.stopwuck())
                .field("mcosel", &self.mcosel())
                .field("mcopre", &self.mcopre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr {
                sw: super::vals::Sw,
                sws: super::vals::Sw,
                hpre: super::vals::Hpre,
                ppre1: super::vals::Ppre,
                ppre2: super::vals::Ppre,
                stopwuck: super::vals::Stopwuck,
                mcosel: super::vals::Mcosel,
                mcopre: super::vals::Mcopre,
            }
            let proxy = Cfgr {
                sw: self.sw(),
                sws: self.sws(),
                hpre: self.hpre(),
                ppre1: self.ppre1(),
                ppre2: self.ppre2(),
                stopwuck: self.stopwuck(),
                mcosel: self.mcosel(),
                mcopre: self.mcopre(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI ready interrupt clear"]
        #[inline(always)]
        pub const fn msirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_msirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL ready interrupt clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLLSAI1 ready interrupt clear"]
        #[inline(always)]
        pub const fn pllsai1rdyc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1 ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllsai1rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSAI2 ready interrupt clear"]
        #[inline(always)]
        pub const fn pllsai2rdyc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2 ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllsai2rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Clock security system interrupt clear"]
        #[inline(always)]
        pub fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 oscillator ready interrupt clear"]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 oscillator ready interrupt clear"]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cicr {
        #[inline(always)]
        fn default() -> Cicr {
            Cicr(0)
        }
    }
    impl core::fmt::Debug for Cicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cicr")
                .field("lsirdyc", &self.lsirdyc())
                .field("lserdyc", &self.lserdyc())
                .field("msirdyc", &self.msirdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("pllrdyc", &self.pllrdyc())
                .field("pllsai1rdyc", &self.pllsai1rdyc())
                .field("pllsai2rdyc", &self.pllsai2rdyc())
                .field("cssc", &self.cssc())
                .field("lsecssc", &self.lsecssc())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cicr {
                lsirdyc: bool,
                lserdyc: bool,
                msirdyc: bool,
                hsirdyc: bool,
                hserdyc: bool,
                pllrdyc: bool,
                pllsai1rdyc: bool,
                pllsai2rdyc: bool,
                cssc: bool,
                lsecssc: bool,
                hsi48rdyc: bool,
            }
            let proxy = Cicr {
                lsirdyc: self.lsirdyc(),
                lserdyc: self.lserdyc(),
                msirdyc: self.msirdyc(),
                hsirdyc: self.hsirdyc(),
                hserdyc: self.hserdyc(),
                pllrdyc: self.pllrdyc(),
                pllsai1rdyc: self.pllsai1rdyc(),
                pllsai2rdyc: self.pllsai2rdyc(),
                cssc: self.cssc(),
                lsecssc: self.lsecssc(),
                hsi48rdyc: self.hsi48rdyc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI ready interrupt enable"]
        #[inline(always)]
        pub const fn msirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_msirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL ready interrupt enable"]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLLSAI1 ready interrupt enable"]
        #[inline(always)]
        pub const fn pllsai1rdyie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1 ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllsai1rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSAI2 ready interrupt enable"]
        #[inline(always)]
        pub const fn pllsai2rdyie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2 ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllsai2rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSE clock security system interrupt enable"]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt enable"]
        #[inline(always)]
        pub fn set_lsecssie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 ready interrupt enable"]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable"]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cier {
        #[inline(always)]
        fn default() -> Cier {
            Cier(0)
        }
    }
    impl core::fmt::Debug for Cier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cier")
                .field("lsirdyie", &self.lsirdyie())
                .field("lserdyie", &self.lserdyie())
                .field("msirdyie", &self.msirdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("pllrdyie", &self.pllrdyie())
                .field("pllsai1rdyie", &self.pllsai1rdyie())
                .field("pllsai2rdyie", &self.pllsai2rdyie())
                .field("lsecssie", &self.lsecssie())
                .field("hsi48rdyie", &self.hsi48rdyie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cier {
                lsirdyie: bool,
                lserdyie: bool,
                msirdyie: bool,
                hsirdyie: bool,
                hserdyie: bool,
                pllrdyie: bool,
                pllsai1rdyie: bool,
                pllsai2rdyie: bool,
                lsecssie: bool,
                hsi48rdyie: bool,
            }
            let proxy = Cier {
                lsirdyie: self.lsirdyie(),
                lserdyie: self.lserdyie(),
                msirdyie: self.msirdyie(),
                hsirdyie: self.hsirdyie(),
                hserdyie: self.hserdyie(),
                pllrdyie: self.pllrdyie(),
                pllsai1rdyie: self.pllsai1rdyie(),
                pllsai2rdyie: self.pllsai2rdyie(),
                lsecssie: self.lsecssie(),
                hsi48rdyie: self.hsi48rdyie(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub const fn msirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_msirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLLSAI1 ready interrupt flag"]
        #[inline(always)]
        pub const fn pllsai1rdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1 ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllsai1rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSAI2 ready interrupt flag"]
        #[inline(always)]
        pub const fn pllsai2rdyf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2 ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllsai2rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE Clock security system interrupt flag"]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Clock security system interrupt flag"]
        #[inline(always)]
        pub fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 ready interrupt flag"]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag"]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cifr {
        #[inline(always)]
        fn default() -> Cifr {
            Cifr(0)
        }
    }
    impl core::fmt::Debug for Cifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cifr")
                .field("lsirdyf", &self.lsirdyf())
                .field("lserdyf", &self.lserdyf())
                .field("msirdyf", &self.msirdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("pllrdyf", &self.pllrdyf())
                .field("pllsai1rdyf", &self.pllsai1rdyf())
                .field("pllsai2rdyf", &self.pllsai2rdyf())
                .field("cssf", &self.cssf())
                .field("lsecssf", &self.lsecssf())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cifr {
                lsirdyf: bool,
                lserdyf: bool,
                msirdyf: bool,
                hsirdyf: bool,
                hserdyf: bool,
                pllrdyf: bool,
                pllsai1rdyf: bool,
                pllsai2rdyf: bool,
                cssf: bool,
                lsecssf: bool,
                hsi48rdyf: bool,
            }
            let proxy = Cifr {
                lsirdyf: self.lsirdyf(),
                lserdyf: self.lserdyf(),
                msirdyf: self.msirdyf(),
                hsirdyf: self.hsirdyf(),
                hserdyf: self.hserdyf(),
                pllrdyf: self.pllrdyf(),
                pllsai1rdyf: self.pllsai1rdyf(),
                pllsai2rdyf: self.pllsai2rdyf(),
                cssf: self.cssf(),
                lsecssf: self.lsecssf(),
                hsi48rdyf: self.hsi48rdyf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "MSI clock enable"]
        #[inline(always)]
        pub const fn msion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock enable"]
        #[inline(always)]
        pub fn set_msion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MSI clock ready flag"]
        #[inline(always)]
        pub const fn msirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock ready flag"]
        #[inline(always)]
        pub fn set_msirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI clock PLL enable"]
        #[inline(always)]
        pub const fn msipllen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock PLL enable"]
        #[inline(always)]
        pub fn set_msipllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MSI clock range selection"]
        #[inline(always)]
        pub const fn msirgsel(&self) -> super::vals::Msirgsel {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Msirgsel::from_bits(val as u8)
        }
        #[doc = "MSI clock range selection"]
        #[inline(always)]
        pub fn set_msirgsel(&mut self, val: super::vals::Msirgsel) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "MSI clock ranges"]
        #[inline(always)]
        pub const fn msirange(&self) -> super::vals::Msirange {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Msirange::from_bits(val as u8)
        }
        #[doc = "MSI clock ranges"]
        #[inline(always)]
        pub fn set_msirange(&mut self, val: super::vals::Msirange) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "HSI clock enable"]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable"]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSI always enable for peripheral kernels"]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HSI always enable for peripheral kernels"]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI automatic start from Stop"]
        #[inline(always)]
        pub const fn hsiasfs(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSI automatic start from Stop"]
        #[inline(always)]
        pub fn set_hsiasfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE crystal oscillator bypass"]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE crystal oscillator bypass"]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Main PLL enable"]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL enable"]
        #[inline(always)]
        pub fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Main PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SAI1 PLL enable"]
        #[inline(always)]
        pub const fn pllsai1on(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 PLL enable"]
        #[inline(always)]
        pub fn set_pllsai1on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SAI1 PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllsai1rdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllsai1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SAI2 PLL enable"]
        #[inline(always)]
        pub const fn pllsai2on(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 PLL enable"]
        #[inline(always)]
        pub fn set_pllsai2on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SAI2 PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllsai2rdy(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllsai2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("msion", &self.msion())
                .field("msirdy", &self.msirdy())
                .field("msipllen", &self.msipllen())
                .field("msirgsel", &self.msirgsel())
                .field("msirange", &self.msirange())
                .field("hsion", &self.hsion())
                .field("hsikeron", &self.hsikeron())
                .field("hsirdy", &self.hsirdy())
                .field("hsiasfs", &self.hsiasfs())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("csson", &self.csson())
                .field("pllon", &self.pllon())
                .field("pllrdy", &self.pllrdy())
                .field("pllsai1on", &self.pllsai1on())
                .field("pllsai1rdy", &self.pllsai1rdy())
                .field("pllsai2on", &self.pllsai2on())
                .field("pllsai2rdy", &self.pllsai2rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr {
                msion: bool,
                msirdy: bool,
                msipllen: bool,
                msirgsel: super::vals::Msirgsel,
                msirange: super::vals::Msirange,
                hsion: bool,
                hsikeron: bool,
                hsirdy: bool,
                hsiasfs: bool,
                hseon: bool,
                hserdy: bool,
                hsebyp: bool,
                csson: bool,
                pllon: bool,
                pllrdy: bool,
                pllsai1on: bool,
                pllsai1rdy: bool,
                pllsai2on: bool,
                pllsai2rdy: bool,
            }
            let proxy = Cr {
                msion: self.msion(),
                msirdy: self.msirdy(),
                msipllen: self.msipllen(),
                msirgsel: self.msirgsel(),
                msirange: self.msirange(),
                hsion: self.hsion(),
                hsikeron: self.hsikeron(),
                hsirdy: self.hsirdy(),
                hsiasfs: self.hsiasfs(),
                hseon: self.hseon(),
                hserdy: self.hserdy(),
                hsebyp: self.hsebyp(),
                csson: self.csson(),
                pllon: self.pllon(),
                pllrdy: self.pllrdy(),
                pllsai1on: self.pllsai1on(),
                pllsai1rdy: self.pllsai1rdy(),
                pllsai2on: self.pllsai2on(),
                pllsai2rdy: self.pllsai2rdy(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock recovery RC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "HSI48 clock enable"]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable"]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSI48 clock ready flag"]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag"]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI48 clock calibration"]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 7usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSI48 clock calibration"]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
        }
    }
    impl Default for Crrcr {
        #[inline(always)]
        fn default() -> Crrcr {
            Crrcr(0)
        }
    }
    impl core::fmt::Debug for Crrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crrcr")
                .field("hsi48on", &self.hsi48on())
                .field("hsi48rdy", &self.hsi48rdy())
                .field("hsi48cal", &self.hsi48cal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crrcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Crrcr {
                hsi48on: bool,
                hsi48rdy: bool,
                hsi48cal: u16,
            }
            let proxy = Crrcr {
                hsi48on: self.hsi48on(),
                hsi48rdy: self.hsi48rdy(),
                hsi48cal: self.hsi48cal(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CSR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSI oscillator ready"]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready"]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SI range after Standby mode"]
        #[inline(always)]
        pub const fn msisrange(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "SI range after Standby mode"]
        #[inline(always)]
        pub fn set_msisrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Firewall reset flag"]
        #[inline(always)]
        pub const fn fwrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Firewall reset flag"]
        #[inline(always)]
        pub fn set_fwrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Option byte loader reset flag"]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loader reset flag"]
        #[inline(always)]
        pub fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Pin reset flag"]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag"]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BOR flag"]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BOR flag"]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent window watchdog reset flag"]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent window watchdog reset flag"]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("msisrange", &self.msisrange())
                .field("rmvf", &self.rmvf())
                .field("fwrstf", &self.fwrstf())
                .field("oblrstf", &self.oblrstf())
                .field("pinrstf", &self.pinrstf())
                .field("borrstf", &self.borrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdgrstf", &self.iwdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Csr {
                lsion: bool,
                lsirdy: bool,
                msisrange: u8,
                rmvf: bool,
                fwrstf: bool,
                oblrstf: bool,
                pinrstf: bool,
                borrstf: bool,
                sftrstf: bool,
                iwdgrstf: bool,
                wwdgrstf: bool,
                lpwrrstf: bool,
            }
            let proxy = Csr {
                lsion: self.lsion(),
                lsirdy: self.lsirdy(),
                msisrange: self.msisrange(),
                rmvf: self.rmvf(),
                fwrstf: self.fwrstf(),
                oblrstf: self.oblrstf(),
                pinrstf: self.pinrstf(),
                borrstf: self.borrstf(),
                sftrstf: self.sftrstf(),
                iwdgrstf: self.iwdgrstf(),
                wwdgrstf: self.wwdgrstf(),
                lpwrrstf: self.lpwrrstf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Internal clock sources calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr(pub u32);
    impl Icscr {
        #[doc = "MSI clock calibration"]
        #[inline(always)]
        pub const fn msical(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MSI clock calibration"]
        #[inline(always)]
        pub fn set_msical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "MSI clock trimming"]
        #[inline(always)]
        pub const fn msitrim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "MSI clock trimming"]
        #[inline(always)]
        pub fn set_msitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Icscr {
        #[inline(always)]
        fn default() -> Icscr {
            Icscr(0)
        }
    }
    impl core::fmt::Debug for Icscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr")
                .field("msical", &self.msical())
                .field("msitrim", &self.msitrim())
                .field("hsical", &self.hsical())
                .field("hsitrim", &self.hsitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Icscr {
                msical: u8,
                msitrim: u8,
                hsical: u8,
                hsitrim: u8,
            }
            let proxy = Icscr {
                msical: self.msical(),
                msitrim: self.msitrim(),
                hsical: self.hsical(),
                hsitrim: self.hsitrim(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Main PLL multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::Plln::from_bits(val as u8)
        }
        #[doc = "Main PLL multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "Main PLL PLLSAI3CLK output enable"]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL PLLSAI3CLK output enable"]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
        #[inline(always)]
        pub const fn pllpbit(&self) -> super::vals::Pllpbit {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pllpbit::from_bits(val as u8)
        }
        #[doc = "Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
        #[inline(always)]
        pub fn set_pllpbit(&mut self, val: super::vals::Pllpbit) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Main PLL PLLUSB1CLK output enable"]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL PLLUSB1CLK output enable"]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "Main PLL division factor for PLLUSB1CLK(48 MHz clock)"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "Main PLL PLLCLK output enable"]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL PLLCLK output enable"]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Main PLL division factor for PLLCLK (system clock)"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "Main PLL division factor for PLLCLK (system clock)"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "Main PLL division factor for PLLSAI2CLK"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 27usize) & 0x1f;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "Main PLL division factor for PLLSAI2CLK"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val.to_bits() as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    impl core::fmt::Debug for Pllcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllcfgr")
                .field("pllsrc", &self.pllsrc())
                .field("pllm", &self.pllm())
                .field("plln", &self.plln())
                .field("pllpen", &self.pllpen())
                .field("pllpbit", &self.pllpbit())
                .field("pllqen", &self.pllqen())
                .field("pllq", &self.pllq())
                .field("pllren", &self.pllren())
                .field("pllr", &self.pllr())
                .field("pllp", &self.pllp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pllcfgr {
                pllsrc: super::vals::Pllsrc,
                pllm: super::vals::Pllm,
                plln: super::vals::Plln,
                pllpen: bool,
                pllpbit: super::vals::Pllpbit,
                pllqen: bool,
                pllq: super::vals::Pllq,
                pllren: bool,
                pllr: super::vals::Pllr,
                pllp: super::vals::Pllp,
            }
            let proxy = Pllcfgr {
                pllsrc: self.pllsrc(),
                pllm: self.pllm(),
                plln: self.plln(),
                pllpen: self.pllpen(),
                pllpbit: self.pllpbit(),
                pllqen: self.pllqen(),
                pllq: self.pllq(),
                pllren: self.pllren(),
                pllr: self.pllr(),
                pllp: self.pllp(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PLLSAI configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllsaicfgr(pub u32);
    impl Pllsaicfgr {
        #[doc = "SAI1PLL multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::Plln::from_bits(val as u8)
        }
        #[doc = "SAI1PLL multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "SAI1PLL PLLSAICLK output enable"]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1PLL PLLSAICLK output enable"]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI1PLL division factor for PLLSAICLK"]
        #[inline(always)]
        pub const fn pllpbit(&self) -> super::vals::Pllpbit {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pllpbit::from_bits(val as u8)
        }
        #[doc = "SAI1PLL division factor for PLLSAICLK"]
        #[inline(always)]
        pub fn set_pllpbit(&mut self, val: super::vals::Pllpbit) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "SAI1PLL PLLUSB2CLK output enable"]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1PLL PLLUSB2CLK output enable"]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1PLL division factor for PLLUSB2CLK"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "SAI1PLL division factor for PLLUSB2CLK"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "PLLSAI PLLADC1CLK output enable"]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI PLLADC1CLK output enable"]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLLSAI division factor for PLLADC1CLK"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLLSAI division factor for PLLADC1CLK"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "PLLSAI division factor for PLLSAICLK"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 27usize) & 0x1f;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "PLLSAI division factor for PLLSAICLK"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val.to_bits() as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for Pllsaicfgr {
        #[inline(always)]
        fn default() -> Pllsaicfgr {
            Pllsaicfgr(0)
        }
    }
    impl core::fmt::Debug for Pllsaicfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllsaicfgr")
                .field("plln", &self.plln())
                .field("pllpen", &self.pllpen())
                .field("pllpbit", &self.pllpbit())
                .field("pllqen", &self.pllqen())
                .field("pllq", &self.pllq())
                .field("pllren", &self.pllren())
                .field("pllr", &self.pllr())
                .field("pllp", &self.pllp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllsaicfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pllsaicfgr {
                plln: super::vals::Plln,
                pllpen: bool,
                pllpbit: super::vals::Pllpbit,
                pllqen: bool,
                pllq: super::vals::Pllq,
                pllren: bool,
                pllr: super::vals::Pllr,
                pllp: super::vals::Pllp,
            }
            let proxy = Pllsaicfgr {
                plln: self.plln(),
                pllpen: self.pllpen(),
                pllpbit: self.pllpbit(),
                pllqen: self.pllqen(),
                pllq: self.pllq(),
                pllren: self.pllren(),
                pllr: self.pllr(),
                pllp: self.pllp(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcsel {
        #[doc = "No clock selected"]
        DISABLE = 0x0,
        #[doc = "PLLADC1CLK clock selected"]
        PLL1_Q = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x03,
    }
    impl Adcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcsel {
        #[inline(always)]
        fn from(val: u8) -> Adcsel {
            Adcsel::from_bits(val)
        }
    }
    impl From<Adcsel> for u8 {
        #[inline(always)]
        fn from(val: Adcsel) -> u8 {
            Adcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clk48sel {
        #[doc = "HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)"]
        HSI48 = 0x0,
        #[doc = "PLLSAI1_Q aka PLL48M1CLK clock selected"]
        PLLSAI1_Q = 0x01,
        #[doc = "PLL_Q aka PLL48M2CLK clock selected"]
        PLL1_Q = 0x02,
        #[doc = "MSI clock selected"]
        MSI = 0x03,
    }
    impl Clk48sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clk48sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clk48sel {
        #[inline(always)]
        fn from(val: u8) -> Clk48sel {
            Clk48sel::from_bits(val)
        }
    }
    impl From<Clk48sel> for u8 {
        #[inline(always)]
        fn from(val: Clk48sel) -> u8 {
            Clk48sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dfsdmsel {
        #[doc = "APB2 clock (PCLK2) selected as DFSDM kernel clock"]
        PCLK2 = 0x0,
        #[doc = "System clock selected as DFSDM kernel clock"]
        SYS = 0x01,
    }
    impl Dfsdmsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dfsdmsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dfsdmsel {
        #[inline(always)]
        fn from(val: u8) -> Dfsdmsel {
            Dfsdmsel::from_bits(val)
        }
    }
    impl From<Dfsdmsel> for u8 {
        #[inline(always)]
        fn from(val: Dfsdmsel) -> u8 {
            Dfsdmsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "system clock not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "system clock divided by 2"]
        DIV2 = 0x08,
        #[doc = "system clock divided by 4"]
        DIV4 = 0x09,
        #[doc = "system clock divided by 8"]
        DIV8 = 0x0a,
        #[doc = "system clock divided by 16"]
        DIV16 = 0x0b,
        #[doc = "system clock divided by 64"]
        DIV64 = 0x0c,
        #[doc = "system clock divided by 128"]
        DIV128 = 0x0d,
        #[doc = "system clock divided by 256"]
        DIV256 = 0x0e,
        #[doc = "system clock divided by 512"]
        DIV512 = 0x0f,
    }
    impl Hpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpre {
        #[inline(always)]
        fn from(val: u8) -> Hpre {
            Hpre::from_bits(val)
        }
    }
    impl From<Hpre> for u8 {
        #[inline(always)]
        fn from(val: Hpre) -> u8 {
            Hpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c1sel {
        #[doc = "PCLK clock selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2c1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1sel {
        #[inline(always)]
        fn from(val: u8) -> I2c1sel {
            I2c1sel::from_bits(val)
        }
    }
    impl From<I2c1sel> for u8 {
        #[inline(always)]
        fn from(val: I2c1sel) -> u8 {
            I2c1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c2sel {
        #[doc = "PCLK clock selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2c2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c2sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c2sel {
        #[inline(always)]
        fn from(val: u8) -> I2c2sel {
            I2c2sel::from_bits(val)
        }
    }
    impl From<I2c2sel> for u8 {
        #[inline(always)]
        fn from(val: I2c2sel) -> u8 {
            I2c2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c3sel {
        #[doc = "PCLK clock selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2c3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c3sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c3sel {
        #[inline(always)]
        fn from(val: u8) -> I2c3sel {
            I2c3sel::from_bits(val)
        }
    }
    impl From<I2c3sel> for u8 {
        #[inline(always)]
        fn from(val: I2c3sel) -> u8 {
            I2c3sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim1sel {
        #[doc = "PCLK clock selected"]
        PCLK1 = 0x0,
        #[doc = "LSI clock selected"]
        LSI = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        #[doc = "LSE clock selected"]
        LSE = 0x03,
    }
    impl Lptim1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim1sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim1sel {
            Lptim1sel::from_bits(val)
        }
    }
    impl From<Lptim1sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim1sel) -> u8 {
            Lptim1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim2sel {
        #[doc = "PCLK clock selected"]
        PCLK1 = 0x0,
        #[doc = "LSI clock selected"]
        LSI = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        #[doc = "LSE clock selected"]
        LSE = 0x03,
    }
    impl Lptim2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim2sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim2sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim2sel {
            Lptim2sel::from_bits(val)
        }
    }
    impl From<Lptim2sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim2sel) -> u8 {
            Lptim2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpuart1sel {
        #[doc = "PCLK clock selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        #[doc = "LSE clock selected"]
        LSE = 0x03,
    }
    impl Lpuart1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuart1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpuart1sel {
        #[inline(always)]
        fn from(val: u8) -> Lpuart1sel {
            Lpuart1sel::from_bits(val)
        }
    }
    impl From<Lpuart1sel> for u8 {
        #[inline(always)]
        fn from(val: Lpuart1sel) -> u8 {
            Lpuart1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lscosel {
        #[doc = "LSI clock selected"]
        LSI = 0x0,
        #[doc = "LSE clock selected"]
        LSE = 0x01,
    }
    impl Lscosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lscosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lscosel {
        #[inline(always)]
        fn from(val: u8) -> Lscosel {
            Lscosel::from_bits(val)
        }
    }
    impl From<Lscosel> for u8 {
        #[inline(always)]
        fn from(val: Lscosel) -> u8 {
            Lscosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0x0,
        #[doc = "Medium low driving capability"]
        MEDIUM_LOW = 0x01,
        #[doc = "Medium high driving capability"]
        MEDIUM_HIGH = 0x02,
        #[doc = "High driving capability"]
        HIGH = 0x03,
    }
    impl Lsedrv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsedrv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsedrv {
        #[inline(always)]
        fn from(val: u8) -> Lsedrv {
            Lsedrv::from_bits(val)
        }
    }
    impl From<Lsedrv> for u8 {
        #[inline(always)]
        fn from(val: Lsedrv) -> u8 {
            Lsedrv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcopre {
        #[doc = "No division"]
        DIV1 = 0x0,
        #[doc = "Division by 2"]
        DIV2 = 0x01,
        #[doc = "Division by 4"]
        DIV4 = 0x02,
        #[doc = "Division by 8"]
        DIV8 = 0x03,
        #[doc = "Division by 16"]
        DIV16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mcopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcopre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcopre {
        #[inline(always)]
        fn from(val: u8) -> Mcopre {
            Mcopre::from_bits(val)
        }
    }
    impl From<Mcopre> for u8 {
        #[inline(always)]
        fn from(val: Mcopre) -> u8 {
            Mcopre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcosel {
        #[doc = "No clock"]
        DISABLE = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "MSI oscillator clock selected"]
        MSI = 0x02,
        #[doc = "HSI oscillator clock selected"]
        HSI = 0x03,
        #[doc = "HSE oscillator clock selected"]
        HSE = 0x04,
        #[doc = "PLL clock selected"]
        PLL = 0x05,
        #[doc = "LSI oscillator clock selected"]
        LSI = 0x06,
        #[doc = "LSE oscillator clock selected"]
        LSE = 0x07,
        #[doc = "HSI48 oscillator clock selected"]
        HSI48 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcosel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcosel {
        #[inline(always)]
        fn from(val: u8) -> Mcosel {
            Mcosel::from_bits(val)
        }
    }
    impl From<Mcosel> for u8 {
        #[inline(always)]
        fn from(val: Mcosel) -> u8 {
            Mcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msirange {
        #[doc = "range 0 around 100 kHz"]
        RANGE100K = 0x0,
        #[doc = "range 1 around 200 kHz"]
        RANGE200K = 0x01,
        #[doc = "range 2 around 400 kHz"]
        RANGE400K = 0x02,
        #[doc = "range 3 around 800 kHz"]
        RANGE800K = 0x03,
        #[doc = "range 4 around 1 MHz"]
        RANGE1M = 0x04,
        #[doc = "range 5 around 2 MHz"]
        RANGE2M = 0x05,
        #[doc = "range 6 around 4 MHz"]
        RANGE4M = 0x06,
        #[doc = "range 7 around 8 MHz"]
        RANGE8M = 0x07,
        #[doc = "range 8 around 16 MHz"]
        RANGE16M = 0x08,
        #[doc = "range 9 around 24 MHz"]
        RANGE24M = 0x09,
        #[doc = "range 10 around 32 MHz"]
        RANGE32M = 0x0a,
        #[doc = "range 11 around 48 MHz"]
        RANGE48M = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Msirange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirange {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirange {
        #[inline(always)]
        fn from(val: u8) -> Msirange {
            Msirange::from_bits(val)
        }
    }
    impl From<Msirange> for u8 {
        #[inline(always)]
        fn from(val: Msirange) -> u8 {
            Msirange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msirgsel {
        #[doc = "MSI Range is provided by MSISRANGE\\[3:0\\]
in RCC_CSR register"]
        CSR = 0x0,
        #[doc = "MSI Range is provided by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
        CR = 0x01,
    }
    impl Msirgsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirgsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirgsel {
        #[inline(always)]
        fn from(val: u8) -> Msirgsel {
            Msirgsel::from_bits(val)
        }
    }
    impl From<Msirgsel> for u8 {
        #[inline(always)]
        fn from(val: Msirgsel) -> u8 {
            Msirgsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllm {
        DIV1 = 0x0,
        DIV2 = 0x01,
        DIV3 = 0x02,
        DIV4 = 0x03,
        DIV5 = 0x04,
        DIV6 = 0x05,
        DIV7 = 0x06,
        DIV8 = 0x07,
    }
    impl Pllm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllm {
        #[inline(always)]
        fn from(val: u8) -> Pllm {
            Pllm::from_bits(val)
        }
    }
    impl From<Pllm> for u8 {
        #[inline(always)]
        fn from(val: Pllm) -> u8 {
            Pllm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plln {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        MUL8 = 0x08,
        MUL9 = 0x09,
        MUL10 = 0x0a,
        MUL11 = 0x0b,
        MUL12 = 0x0c,
        MUL13 = 0x0d,
        MUL14 = 0x0e,
        MUL15 = 0x0f,
        MUL16 = 0x10,
        MUL17 = 0x11,
        MUL18 = 0x12,
        MUL19 = 0x13,
        MUL20 = 0x14,
        MUL21 = 0x15,
        MUL22 = 0x16,
        MUL23 = 0x17,
        MUL24 = 0x18,
        MUL25 = 0x19,
        MUL26 = 0x1a,
        MUL27 = 0x1b,
        MUL28 = 0x1c,
        MUL29 = 0x1d,
        MUL30 = 0x1e,
        MUL31 = 0x1f,
        MUL32 = 0x20,
        MUL33 = 0x21,
        MUL34 = 0x22,
        MUL35 = 0x23,
        MUL36 = 0x24,
        MUL37 = 0x25,
        MUL38 = 0x26,
        MUL39 = 0x27,
        MUL40 = 0x28,
        MUL41 = 0x29,
        MUL42 = 0x2a,
        MUL43 = 0x2b,
        MUL44 = 0x2c,
        MUL45 = 0x2d,
        MUL46 = 0x2e,
        MUL47 = 0x2f,
        MUL48 = 0x30,
        MUL49 = 0x31,
        MUL50 = 0x32,
        MUL51 = 0x33,
        MUL52 = 0x34,
        MUL53 = 0x35,
        MUL54 = 0x36,
        MUL55 = 0x37,
        MUL56 = 0x38,
        MUL57 = 0x39,
        MUL58 = 0x3a,
        MUL59 = 0x3b,
        MUL60 = 0x3c,
        MUL61 = 0x3d,
        MUL62 = 0x3e,
        MUL63 = 0x3f,
        MUL64 = 0x40,
        MUL65 = 0x41,
        MUL66 = 0x42,
        MUL67 = 0x43,
        MUL68 = 0x44,
        MUL69 = 0x45,
        MUL70 = 0x46,
        MUL71 = 0x47,
        MUL72 = 0x48,
        MUL73 = 0x49,
        MUL74 = 0x4a,
        MUL75 = 0x4b,
        MUL76 = 0x4c,
        MUL77 = 0x4d,
        MUL78 = 0x4e,
        MUL79 = 0x4f,
        MUL80 = 0x50,
        MUL81 = 0x51,
        MUL82 = 0x52,
        MUL83 = 0x53,
        MUL84 = 0x54,
        MUL85 = 0x55,
        MUL86 = 0x56,
        MUL87 = 0x57,
        MUL88 = 0x58,
        MUL89 = 0x59,
        MUL90 = 0x5a,
        MUL91 = 0x5b,
        MUL92 = 0x5c,
        MUL93 = 0x5d,
        MUL94 = 0x5e,
        MUL95 = 0x5f,
        MUL96 = 0x60,
        MUL97 = 0x61,
        MUL98 = 0x62,
        MUL99 = 0x63,
        MUL100 = 0x64,
        MUL101 = 0x65,
        MUL102 = 0x66,
        MUL103 = 0x67,
        MUL104 = 0x68,
        MUL105 = 0x69,
        MUL106 = 0x6a,
        MUL107 = 0x6b,
        MUL108 = 0x6c,
        MUL109 = 0x6d,
        MUL110 = 0x6e,
        MUL111 = 0x6f,
        MUL112 = 0x70,
        MUL113 = 0x71,
        MUL114 = 0x72,
        MUL115 = 0x73,
        MUL116 = 0x74,
        MUL117 = 0x75,
        MUL118 = 0x76,
        MUL119 = 0x77,
        MUL120 = 0x78,
        MUL121 = 0x79,
        MUL122 = 0x7a,
        MUL123 = 0x7b,
        MUL124 = 0x7c,
        MUL125 = 0x7d,
        MUL126 = 0x7e,
        MUL127 = 0x7f,
    }
    impl Plln {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plln {
            unsafe { core::mem::transmute(val & 0x7f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plln {
        #[inline(always)]
        fn from(val: u8) -> Plln {
            Plln::from_bits(val)
        }
    }
    impl From<Plln> for u8 {
        #[inline(always)]
        fn from(val: Plln) -> u8 {
            Plln::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllp {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
        DIV8 = 0x08,
        DIV9 = 0x09,
        DIV10 = 0x0a,
        DIV11 = 0x0b,
        DIV12 = 0x0c,
        DIV13 = 0x0d,
        DIV14 = 0x0e,
        DIV15 = 0x0f,
        DIV16 = 0x10,
        DIV17 = 0x11,
        DIV18 = 0x12,
        DIV19 = 0x13,
        DIV20 = 0x14,
        DIV21 = 0x15,
        DIV22 = 0x16,
        DIV23 = 0x17,
        DIV24 = 0x18,
        DIV25 = 0x19,
        DIV26 = 0x1a,
        DIV27 = 0x1b,
        DIV28 = 0x1c,
        DIV29 = 0x1d,
        DIV30 = 0x1e,
        DIV31 = 0x1f,
    }
    impl Pllp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllp {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllp {
        #[inline(always)]
        fn from(val: u8) -> Pllp {
            Pllp::from_bits(val)
        }
    }
    impl From<Pllp> for u8 {
        #[inline(always)]
        fn from(val: Pllp) -> u8 {
            Pllp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllpbit {
        DIV7 = 0x0,
        DIV17 = 0x01,
    }
    impl Pllpbit {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllpbit {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllpbit {
        #[inline(always)]
        fn from(val: u8) -> Pllpbit {
            Pllpbit::from_bits(val)
        }
    }
    impl From<Pllpbit> for u8 {
        #[inline(always)]
        fn from(val: Pllpbit) -> u8 {
            Pllpbit::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllq {
        DIV2 = 0x0,
        DIV4 = 0x01,
        DIV6 = 0x02,
        DIV8 = 0x03,
    }
    impl Pllq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllq {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllq {
        #[inline(always)]
        fn from(val: u8) -> Pllq {
            Pllq::from_bits(val)
        }
    }
    impl From<Pllq> for u8 {
        #[inline(always)]
        fn from(val: Pllq) -> u8 {
            Pllq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllr {
        DIV2 = 0x0,
        DIV4 = 0x01,
        DIV6 = 0x02,
        DIV8 = 0x03,
    }
    impl Pllr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllr {
        #[inline(always)]
        fn from(val: u8) -> Pllr {
            Pllr::from_bits(val)
        }
    }
    impl From<Pllr> for u8 {
        #[inline(always)]
        fn from(val: Pllr) -> u8 {
            Pllr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "No clock sent to PLL"]
        DISABLE = 0x0,
        #[doc = "MSI selected as PLL input clock"]
        MSI = 0x01,
        #[doc = "HSI selected as PLL input clock"]
        HSI = 0x02,
        #[doc = "HSE selected as PLL input clock"]
        HSE = 0x03,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsrc {
        #[inline(always)]
        fn from(val: u8) -> Pllsrc {
            Pllsrc::from_bits(val)
        }
    }
    impl From<Pllsrc> for u8 {
        #[inline(always)]
        fn from(val: Pllsrc) -> u8 {
            Pllsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        #[doc = "HCLK not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HCLK divided by 2"]
        DIV2 = 0x04,
        #[doc = "HCLK divided by 4"]
        DIV4 = 0x05,
        #[doc = "HCLK divided by 8"]
        DIV8 = 0x06,
        #[doc = "HCLK divided by 16"]
        DIV16 = 0x07,
    }
    impl Ppre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre {
        #[inline(always)]
        fn from(val: u8) -> Ppre {
            Ppre::from_bits(val)
        }
    }
    impl From<Ppre> for u8 {
        #[inline(always)]
        fn from(val: Ppre) -> u8 {
            Ppre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "No clock"]
        DISABLE = 0x0,
        #[doc = "LSE oscillator clock used as RTC clock"]
        LSE = 0x01,
        #[doc = "LSI oscillator clock used as RTC clock"]
        LSI = 0x02,
        #[doc = "HSE oscillator clock divided by 32 used as the RTC clock"]
        HSE = 0x03,
    }
    impl Rtcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcsel {
        #[inline(always)]
        fn from(val: u8) -> Rtcsel {
            Rtcsel::from_bits(val)
        }
    }
    impl From<Rtcsel> for u8 {
        #[inline(always)]
        fn from(val: Rtcsel) -> u8 {
            Rtcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sai1sel {
        #[doc = "PLLSAI1CLK clock is selected as SAIx clock"]
        PLLSAI1_P = 0x0,
        #[doc = "PLLSAI2CLK clock is selected as SAIx clock"]
        PLLSAI2_P = 0x01,
        #[doc = "PLLSAI3CLK clock is selected as SAIx clock"]
        PLL1_P = 0x02,
        #[doc = "External clock SAIx_EXTCLK clock selected as SAIx clock"]
        SAI1_EXTCLK = 0x03,
    }
    impl Sai1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sai1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sai1sel {
        #[inline(always)]
        fn from(val: u8) -> Sai1sel {
            Sai1sel::from_bits(val)
        }
    }
    impl From<Sai1sel> for u8 {
        #[inline(always)]
        fn from(val: Sai1sel) -> u8 {
            Sai1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sai2sel {
        #[doc = "PLLSAI1CLK clock is selected as SAIx clock"]
        PLLSAI1_P = 0x0,
        #[doc = "PLLSAI2CLK clock is selected as SAIx clock"]
        PLLSAI2_P = 0x01,
        #[doc = "PLLSAI3CLK clock is selected as SAIx clock"]
        PLL1_P = 0x02,
        #[doc = "External clock SAIx_EXTCLK clock selected as SAIx clock"]
        SAI2_EXTCLK = 0x03,
    }
    impl Sai2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sai2sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sai2sel {
        #[inline(always)]
        fn from(val: u8) -> Sai2sel {
            Sai2sel::from_bits(val)
        }
    }
    impl From<Sai2sel> for u8 {
        #[inline(always)]
        fn from(val: Sai2sel) -> u8 {
            Sai2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopwuck {
        #[doc = "MSI oscillator selected as wake-up from Stop clock"]
        MSI = 0x0,
        #[doc = "HSI oscillator selected as wake-up from Stop clock"]
        HSI = 0x01,
    }
    impl Stopwuck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopwuck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopwuck {
        #[inline(always)]
        fn from(val: u8) -> Stopwuck {
            Stopwuck::from_bits(val)
        }
    }
    impl From<Stopwuck> for u8 {
        #[inline(always)]
        fn from(val: Stopwuck) -> u8 {
            Stopwuck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sw {
        #[doc = "MSI selected as system clock"]
        MSI = 0x0,
        #[doc = "HSI selected as system clock"]
        HSI = 0x01,
        #[doc = "HSE selected as system clock"]
        HSE = 0x02,
        #[doc = "PLL selected as system clock"]
        PLL1_R = 0x03,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sw {
        #[inline(always)]
        fn from(val: u8) -> Sw {
            Sw::from_bits(val)
        }
    }
    impl From<Sw> for u8 {
        #[inline(always)]
        fn from(val: Sw) -> u8 {
            Sw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Swpmi1sel {
        PCLK1 = 0x0,
        HSI = 0x01,
    }
    impl Swpmi1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Swpmi1sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Swpmi1sel {
        #[inline(always)]
        fn from(val: u8) -> Swpmi1sel {
            Swpmi1sel::from_bits(val)
        }
    }
    impl From<Swpmi1sel> for u8 {
        #[inline(always)]
        fn from(val: Swpmi1sel) -> u8 {
            Swpmi1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart1sel {
        #[doc = "PCLK clock selected"]
        PCLK2 = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        #[doc = "LSE clock selected"]
        LSE = 0x03,
    }
    impl Usart1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart1sel {
        #[inline(always)]
        fn from(val: u8) -> Usart1sel {
            Usart1sel::from_bits(val)
        }
    }
    impl From<Usart1sel> for u8 {
        #[inline(always)]
        fn from(val: Usart1sel) -> u8 {
            Usart1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usartsel {
        #[doc = "PCLK clock selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x01,
        #[doc = "HSI clock selected"]
        HSI = 0x02,
        #[doc = "LSE clock selected"]
        LSE = 0x03,
    }
    impl Usartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usartsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usartsel {
        #[inline(always)]
        fn from(val: u8) -> Usartsel {
            Usartsel::from_bits(val)
        }
    }
    impl From<Usartsel> for u8 {
        #[inline(always)]
        fn from(val: Usartsel) -> u8 {
            Usartsel::to_bits(val)
        }
    }
}
