#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flash"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Flash key register"]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Flash option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Flash status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Flash control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Flash address register"]
    #[inline(always)]
    pub const fn ar(self) -> crate::common::Reg<regs::Ar, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Option byte register"]
    #[inline(always)]
    pub const fn obr(self) -> crate::common::Reg<regs::Obr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Write protection register"]
    #[inline(always)]
    pub const fn wrpr(self) -> crate::common::Reg<regs::Wrpr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Flash access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "LATENCY"]
        #[inline(always)]
        pub const fn latency(&self) -> super::vals::Latency {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Latency::from_bits(val as u8)
        }
        #[doc = "LATENCY"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: super::vals::Latency) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Prefetch buffer enable"]
        #[inline(always)]
        pub const fn prftbe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch buffer enable"]
        #[inline(always)]
        pub fn set_prftbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prefetch buffer status"]
        #[inline(always)]
        pub const fn prftbs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch buffer status"]
        #[inline(always)]
        pub fn set_prftbs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("prftbe", &self.prftbe())
                .field("prftbs", &self.prftbs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Acr {
                latency: super::vals::Latency,
                prftbe: bool,
                prftbs: bool,
            }
            let proxy = Acr {
                latency: self.latency(),
                prftbe: self.prftbe(),
                prftbs: self.prftbs(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Flash address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ar(pub u32);
    impl Ar {
        #[doc = "Flash address"]
        #[inline(always)]
        pub const fn far(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Flash address"]
        #[inline(always)]
        pub fn set_far(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ar {
        #[inline(always)]
        fn default() -> Ar {
            Ar(0)
        }
    }
    impl core::fmt::Debug for Ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ar").field("far", &self.far()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ar {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ar {
                far: u32,
            }
            let proxy = Ar { far: self.far() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Flash control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Mass erase"]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase"]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Option byte programming"]
        #[inline(always)]
        pub const fn optpg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte programming"]
        #[inline(always)]
        pub fn set_optpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Option byte erase"]
        #[inline(always)]
        pub const fn opter(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte erase"]
        #[inline(always)]
        pub fn set_opter(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Lock"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Lock"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option bytes write enable"]
        #[inline(always)]
        pub const fn optwre(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Option bytes write enable"]
        #[inline(always)]
        pub fn set_optwre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Force option byte loading"]
        #[inline(always)]
        pub const fn force_optload(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Force option byte loading"]
        #[inline(always)]
        pub fn set_force_optload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("mer", &self.mer())
                .field("optpg", &self.optpg())
                .field("opter", &self.opter())
                .field("strt", &self.strt())
                .field("lock", &self.lock())
                .field("optwre", &self.optwre())
                .field("errie", &self.errie())
                .field("eopie", &self.eopie())
                .field("force_optload", &self.force_optload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr {
                pg: bool,
                per: bool,
                mer: bool,
                optpg: bool,
                opter: bool,
                strt: bool,
                lock: bool,
                optwre: bool,
                errie: bool,
                eopie: bool,
                force_optload: bool,
            }
            let proxy = Cr {
                pg: self.pg(),
                per: self.per(),
                mer: self.mer(),
                optpg: self.optpg(),
                opter: self.opter(),
                strt: self.strt(),
                lock: self.lock(),
                optwre: self.optwre(),
                errie: self.errie(),
                eopie: self.eopie(),
                force_optload: self.force_optload(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Option byte register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obr(pub u32);
    impl Obr {
        #[doc = "Option byte error"]
        #[inline(always)]
        pub const fn opterr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte error"]
        #[inline(always)]
        pub fn set_opterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read protection level status"]
        #[inline(always)]
        pub const fn rdprt(&self) -> super::vals::Rdprt {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Rdprt::from_bits(val as u8)
        }
        #[doc = "Read protection level status"]
        #[inline(always)]
        pub fn set_rdprt(&mut self, val: super::vals::Rdprt) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "WDG_SW"]
        #[inline(always)]
        pub const fn wdg_sw(&self) -> super::vals::WdgSw {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::WdgSw::from_bits(val as u8)
        }
        #[doc = "WDG_SW"]
        #[inline(always)]
        pub fn set_wdg_sw(&mut self, val: super::vals::WdgSw) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> super::vals::NRstStop {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::NRstStop::from_bits(val as u8)
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: super::vals::NRstStop) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> super::vals::NRstStdby {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::NRstStdby::from_bits(val as u8)
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: super::vals::NRstStdby) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "nBOOT0"]
        #[inline(always)]
        pub const fn n_boot0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "nBOOT0"]
        #[inline(always)]
        pub fn set_n_boot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BOOT1"]
        #[inline(always)]
        pub const fn n_boot1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT1"]
        #[inline(always)]
        pub fn set_n_boot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VDDA power supply supervisor enabled"]
        #[inline(always)]
        pub const fn vdda_monitor(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "VDDA power supply supervisor enabled"]
        #[inline(always)]
        pub fn set_vdda_monitor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "RAM_PARITY_CHECK"]
        #[inline(always)]
        pub const fn ram_parity_check(&self) -> super::vals::RamParityCheck {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::RamParityCheck::from_bits(val as u8)
        }
        #[doc = "RAM_PARITY_CHECK"]
        #[inline(always)]
        pub fn set_ram_parity_check(&mut self, val: super::vals::RamParityCheck) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "BOOT_SEL"]
        #[inline(always)]
        pub const fn boot_sel(&self) -> super::vals::BootSel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::BootSel::from_bits(val as u8)
        }
        #[doc = "BOOT_SEL"]
        #[inline(always)]
        pub fn set_boot_sel(&mut self, val: super::vals::BootSel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Data0"]
        #[inline(always)]
        pub const fn data0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Data0"]
        #[inline(always)]
        pub fn set_data0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Data1"]
        #[inline(always)]
        pub const fn data1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Data1"]
        #[inline(always)]
        pub fn set_data1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Obr {
        #[inline(always)]
        fn default() -> Obr {
            Obr(0)
        }
    }
    impl core::fmt::Debug for Obr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obr")
                .field("opterr", &self.opterr())
                .field("rdprt", &self.rdprt())
                .field("wdg_sw", &self.wdg_sw())
                .field("n_rst_stop", &self.n_rst_stop())
                .field("n_rst_stdby", &self.n_rst_stdby())
                .field("n_boot0", &self.n_boot0())
                .field("n_boot1", &self.n_boot1())
                .field("vdda_monitor", &self.vdda_monitor())
                .field("ram_parity_check", &self.ram_parity_check())
                .field("boot_sel", &self.boot_sel())
                .field("data0", &self.data0())
                .field("data1", &self.data1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Obr {
                opterr: bool,
                rdprt: super::vals::Rdprt,
                wdg_sw: super::vals::WdgSw,
                n_rst_stop: super::vals::NRstStop,
                n_rst_stdby: super::vals::NRstStdby,
                n_boot0: bool,
                n_boot1: bool,
                vdda_monitor: bool,
                ram_parity_check: super::vals::RamParityCheck,
                boot_sel: super::vals::BootSel,
                data0: u8,
                data1: u8,
            }
            let proxy = Obr {
                opterr: self.opterr(),
                rdprt: self.rdprt(),
                wdg_sw: self.wdg_sw(),
                n_rst_stop: self.n_rst_stop(),
                n_rst_stdby: self.n_rst_stdby(),
                n_boot0: self.n_boot0(),
                n_boot1: self.n_boot1(),
                vdda_monitor: self.vdda_monitor(),
                ram_parity_check: self.ram_parity_check(),
                boot_sel: self.boot_sel(),
                data0: self.data0(),
                data1: self.data1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Flash status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Busy"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub const fn pgerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub fn set_pgerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Write protection error"]
        #[inline(always)]
        pub const fn wrprt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error"]
        #[inline(always)]
        pub fn set_wrprt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("bsy", &self.bsy())
                .field("pgerr", &self.pgerr())
                .field("wrprt", &self.wrprt())
                .field("eop", &self.eop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Sr {
                bsy: bool,
                pgerr: bool,
                wrprt: bool,
                eop: bool,
            }
            let proxy = Sr {
                bsy: self.bsy(),
                pgerr: self.pgerr(),
                wrprt: self.wrprt(),
                eop: self.eop(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpr(pub u32);
    impl Wrpr {
        #[doc = "Write protect"]
        #[inline(always)]
        pub const fn wrp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write protect"]
        #[inline(always)]
        pub fn set_wrp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrpr {
        #[inline(always)]
        fn default() -> Wrpr {
            Wrpr(0)
        }
    }
    impl core::fmt::Debug for Wrpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrpr").field("wrp", &self.wrp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrpr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Wrpr {
                wrp: u32,
            }
            let proxy = Wrpr { wrp: self.wrp() };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BootSel {
        #[doc = "BOOT0 signal is defined by nBOOT0 option bit"]
        N_BOOT0 = 0x0,
        #[doc = "BOOT0 signal is defined by BOOT0 pin value (legacy mode)"]
        BOOT0 = 0x01,
    }
    impl BootSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BootSel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BootSel {
        #[inline(always)]
        fn from(val: u8) -> BootSel {
            BootSel::from_bits(val)
        }
    }
    impl From<BootSel> for u8 {
        #[inline(always)]
        fn from(val: BootSel) -> u8 {
            BootSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Latency {
        #[doc = "0 wait states"]
        WS0 = 0x0,
        #[doc = "1 wait state"]
        WS1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Latency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Latency {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Latency {
        #[inline(always)]
        fn from(val: u8) -> Latency {
            Latency::from_bits(val)
        }
    }
    impl From<Latency> for u8 {
        #[inline(always)]
        fn from(val: Latency) -> u8 {
            Latency::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum NRstStdby {
        #[doc = "Reset generated when entering Standby mode"]
        RESET = 0x0,
        #[doc = "No reset generated"]
        NO_RESET = 0x01,
    }
    impl NRstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NRstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NRstStdby {
        #[inline(always)]
        fn from(val: u8) -> NRstStdby {
            NRstStdby::from_bits(val)
        }
    }
    impl From<NRstStdby> for u8 {
        #[inline(always)]
        fn from(val: NRstStdby) -> u8 {
            NRstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum NRstStop {
        #[doc = "Reset generated when entering Stop mode"]
        RESET = 0x0,
        #[doc = "No reset generated"]
        NO_RESET = 0x01,
    }
    impl NRstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NRstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NRstStop {
        #[inline(always)]
        fn from(val: u8) -> NRstStop {
            NRstStop::from_bits(val)
        }
    }
    impl From<NRstStop> for u8 {
        #[inline(always)]
        fn from(val: NRstStop) -> u8 {
            NRstStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RamParityCheck {
        #[doc = "RAM parity check enabled"]
        ENABLED = 0x0,
        #[doc = "RAM parity check disabled"]
        DISABLED = 0x01,
    }
    impl RamParityCheck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RamParityCheck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RamParityCheck {
        #[inline(always)]
        fn from(val: u8) -> RamParityCheck {
            RamParityCheck::from_bits(val)
        }
    }
    impl From<RamParityCheck> for u8 {
        #[inline(always)]
        fn from(val: RamParityCheck) -> u8 {
            RamParityCheck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rdprt {
        #[doc = "Level 0"]
        LEVEL0 = 0x0,
        #[doc = "Level 1"]
        LEVEL1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Level 2"]
        LEVEL2 = 0x03,
    }
    impl Rdprt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rdprt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rdprt {
        #[inline(always)]
        fn from(val: u8) -> Rdprt {
            Rdprt::from_bits(val)
        }
    }
    impl From<Rdprt> for u8 {
        #[inline(always)]
        fn from(val: Rdprt) -> u8 {
            Rdprt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WdgSw {
        #[doc = "Hardware watchdog"]
        HARDWARE = 0x0,
        #[doc = "Software watchdog"]
        SOFTWARE = 0x01,
    }
    impl WdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WdgSw {
        #[inline(always)]
        fn from(val: u8) -> WdgSw {
            WdgSw::from_bits(val)
        }
    }
    impl From<WdgSw> for u8 {
        #[inline(always)]
        fn from(val: WdgSw) -> u8 {
            WdgSw::to_bits(val)
        }
    }
}
