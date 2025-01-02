#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FPU interrupt mask register"]
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::Fpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CPU non-secure lock register"]
    #[inline(always)]
    pub const fn cnslckr(self) -> crate::common::Reg<regs::Cnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CPU secure lock register"]
    #[inline(always)]
    pub const fn cslockr(self) -> crate::common::Reg<regs::Cslockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "memory erase status register"]
    #[inline(always)]
    pub const fn mesr(self) -> crate::common::Reg<regs::Mesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "compensation cell control/status register"]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "compensation cell value register"]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "compensation cell code register"]
    #[inline(always)]
    pub const fn cccr(self) -> crate::common::Reg<regs::Cccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RSS command register"]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "compensation cell code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccr(pub u32);
    impl Cccr {
        #[doc = "NMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub const fn ncc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub fn set_ncc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub const fn pcc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code of the I/Os supplied by V<sub>DD</sub> These bits are written by software to define an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is set."]
        #[inline(always)]
        pub fn set_pcc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cccr {
        #[inline(always)]
        fn default() -> Cccr {
            Cccr(0)
        }
    }
    impl core::fmt::Debug for Cccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccr")
                .field("ncc1", &self.ncc1())
                .field("pcc1", &self.pcc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cccr {
                ncc1: u8,
                pcc1: u8,
            }
            let proxy = Cccr {
                ncc1: self.ncc1(),
                pcc1: self.pcc1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "compensation cell control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/Os compensation cell enable This bit enables the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub const fn cs1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/Os code selection This bit selects the code to be applied for the compensation cell of the I/Os supplied by V<sub>DD</sub>."]
        #[inline(always)]
        pub fn set_cs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VDD I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by V<sub>DD</sub>. Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY1) is not set if the HSI clock is not enabled (HSION)."]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VDD I/Os compensation cell ready flag This bit provides the compensation cell status of the I/Os supplied by V<sub>DD</sub>. Note: The HSI clock is required for the compensation cell to work properly. The compensation cell ready bit (RDY1) is not set if the HSI clock is not enabled (HSION)."]
        #[inline(always)]
        pub fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cccsr {
        #[inline(always)]
        fn default() -> Cccsr {
            Cccsr(0)
        }
    }
    impl core::fmt::Debug for Cccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccsr")
                .field("en1", &self.en1())
                .field("cs1", &self.cs1())
                .field("rdy1", &self.rdy1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccsr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cccsr {
                en1: bool,
                cs1: bool,
                rdy1: bool,
            }
            let proxy = Cccsr {
                en1: self.en1(),
                cs1: self.cs1(),
                rdy1: self.rdy1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "compensation cell value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvr(pub u32);
    impl Ccvr {
        #[doc = "NMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub const fn ncv1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for NMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub fn set_ncv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub const fn pcv1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation value of the I/Os supplied by V<sub>DD</sub> This value is provided by the cell and can be used by the CPU to compute an I/Os compensation cell code for PMOS transistors. This code is applied to the I/Os compensation cell when the CS1 bit of the CCCSR is reset."]
        #[inline(always)]
        pub fn set_pcv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Ccvr {
        #[inline(always)]
        fn default() -> Ccvr {
            Ccvr(0)
        }
    }
    impl core::fmt::Debug for Ccvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccvr")
                .field("ncv1", &self.ncv1())
                .field("pcv1", &self.pcv1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccvr {
                ncv1: u8,
                pcv1: u8,
            }
            let proxy = Ccvr {
                ncv1: self.ncv1(),
                pcv1: self.pcv1(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub const fn anaswvdd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table�121 for setting."]
        #[inline(always)]
        pub fn set_anaswvdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6."]
        #[inline(always)]
        pub const fn pa6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6."]
        #[inline(always)]
        pub fn set_pa6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7."]
        #[inline(always)]
        pub const fn pa7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7."]
        #[inline(always)]
        pub fn set_pa7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15."]
        #[inline(always)]
        pub const fn pa15_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15."]
        #[inline(always)]
        pub fn set_pa15_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3."]
        #[inline(always)]
        pub const fn pb3_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3."]
        #[inline(always)]
        pub fn set_pb3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("boosten", &self.boosten())
                .field("anaswvdd", &self.anaswvdd())
                .field("pa6_fmp", &self.pa6_fmp())
                .field("pa7_fmp", &self.pa7_fmp())
                .field("pa15_fmp", &self.pa15_fmp())
                .field("pb3_fmp", &self.pb3_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr1 {
                boosten: bool,
                anaswvdd: bool,
                pa6_fmp: bool,
                pa7_fmp: bool,
                pa15_fmp: bool,
                pb3_fmp: bool,
            }
            let proxy = Cfgr1 {
                boosten: self.boosten(),
                anaswvdd: self.anaswvdd(),
                pa6_fmp: self.pa6_fmp(),
                pa7_fmp: self.pa7_fmp(),
                pa15_fmp: self.pa15_fmp(),
                pb3_fmp: self.pb3_fmp(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex-M33 LOCKUP (hardfault) output enable This bit is set by software and cleared only by a system reset. It can be used to enable and lock the connection of Cortex-M33 LOCKUP (hardfault) output to TIM1/16/17 break input."]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M33 LOCKUP (hardfault) output enable This bit is set by software and cleared only by a system reset. It can be used to enable and lock the connection of Cortex-M33 LOCKUP (hardfault) output to TIM1/16/17 break input."]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 parity lock bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/16/17 break inputs."]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity lock bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/16/17 break inputs."]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection to TIM1/16/17 break input, as well as the PVDE and PVDLS\\[2:0\\]
in the PWR register."]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection to TIM1/16/17 break input, as well as the PVDE and PVDLS\\[2:0\\]
in the PWR register."]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the Flash ECC double error signal connection to TIM1/16/17 break input."]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the Flash ECC double error signal connection to TIM1/16/17 break input."]
        #[inline(always)]
        pub fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("cll", &self.cll())
                .field("spl", &self.spl())
                .field("pvdl", &self.pvdl())
                .field("eccl", &self.eccl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr2 {
                cll: bool,
                spl: bool,
                pvdl: bool,
                eccl: bool,
            }
            let proxy = Cfgr2 {
                cll: self.cll(),
                spl: self.spl(),
                pvdl: self.pvdl(),
                eccl: self.eccl(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CPU non-secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnslckr(pub u32);
    impl Cnslckr {
        #[doc = "VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_NS register lock This bit is set by software and cleared only by a system reset."]
        #[inline(always)]
        pub fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers."]
        #[inline(always)]
        pub fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cnslckr {
        #[inline(always)]
        fn default() -> Cnslckr {
            Cnslckr(0)
        }
    }
    impl core::fmt::Debug for Cnslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnslckr")
                .field("locknsvtor", &self.locknsvtor())
                .field("locknsmpu", &self.locknsmpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnslckr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cnslckr {
                locknsvtor: bool,
                locknsmpu: bool,
            }
            let proxy = Cnslckr {
                locknsvtor: self.locknsvtor(),
                locknsmpu: self.locknsmpu(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "CPU secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cslockr(pub u32);
    impl Cslockr {
        #[doc = "VTOR_S register and AIRCR register bits lock This bit is set by software and cleared only by a system reset. When set, it disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_S register and AIRCR register bits lock This bit is set by software and cleared only by a system reset. When set, it disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register."]
        #[inline(always)]
        pub fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers."]
        #[inline(always)]
        pub fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SAU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SAU registers lock This bit is set by software and cleared only by a system reset. When set, it disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers."]
        #[inline(always)]
        pub fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cslockr {
        #[inline(always)]
        fn default() -> Cslockr {
            Cslockr(0)
        }
    }
    impl core::fmt::Debug for Cslockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cslockr")
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locksmpu", &self.locksmpu())
                .field("locksau", &self.locksau())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cslockr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cslockr {
                locksvtaircr: bool,
                locksmpu: bool,
                locksau: bool,
            }
            let proxy = Cslockr {
                locksvtaircr: self.locksvtaircr(),
                locksmpu: self.locksmpu(),
                locksau: self.locksau(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "FPU interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuimr(pub u32);
    impl Fpuimr {
        #[doc = "Floating point unit interrupts enable bits FPU_IE\\[5\\]: Inexact interrupt enable (interrupt disable at reset) FPU_IE\\[4\\]: Input abnormal interrupt enable FPU_IE\\[3\\]: Overflow interrupt enable FPU_IE\\[2\\]: Underflow interrupt enable FPU_IE\\[1\\]: Divide-by-zero interrupt enable FPU_IE\\[0\\]: Invalid operation Interrupt enable"]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Floating point unit interrupts enable bits FPU_IE\\[5\\]: Inexact interrupt enable (interrupt disable at reset) FPU_IE\\[4\\]: Input abnormal interrupt enable FPU_IE\\[3\\]: Overflow interrupt enable FPU_IE\\[2\\]: Underflow interrupt enable FPU_IE\\[1\\]: Divide-by-zero interrupt enable FPU_IE\\[0\\]: Invalid operation Interrupt enable"]
        #[inline(always)]
        pub fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Fpuimr {
        #[inline(always)]
        fn default() -> Fpuimr {
            Fpuimr(0)
        }
    }
    impl core::fmt::Debug for Fpuimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpuimr").field("fpu_ie", &self.fpu_ie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpuimr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fpuimr {
                fpu_ie: u8,
            }
            let proxy = Fpuimr { fpu_ie: self.fpu_ie() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "memory erase status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mesr(pub u32);
    impl Mesr {
        #[doc = "Device memories erase status This bit is set by hardware when SRAM2, ICACHE, PKA SRAM erase is completed after power-on reset or tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is not reset by system reset and is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub const fn mclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Device memories erase status This bit is set by hardware when SRAM2, ICACHE, PKA SRAM erase is completed after power-on reset or tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is not reset by system reset and is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub fn set_mclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ICACHE and PKA SRAM erase status This bit is set by hardware when ICACHE and PKA SRAM erase is completed after potential tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub const fn ipmee(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE and PKA SRAM erase status This bit is set by hardware when ICACHE and PKA SRAM erase is completed after potential tamper detection (refer to Section�75: Tamper and backup registers (TAMP) for more details). This bit is cleared by software by writing 1 to it."]
        #[inline(always)]
        pub fn set_ipmee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mesr {
        #[inline(always)]
        fn default() -> Mesr {
            Mesr(0)
        }
    }
    impl core::fmt::Debug for Mesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mesr")
                .field("mclr", &self.mclr())
                .field("ipmee", &self.ipmee())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mesr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Mesr {
                mclr: bool,
                ipmee: bool,
            }
            let proxy = Mesr {
                mclr: self.mclr(),
                ipmee: self.ipmee(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RSS command register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS commands This field defines a command to be executed by the RSS."]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RSS commands This field defines a command to be executed by the RSS."]
        #[inline(always)]
        pub fn set_rsscmd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rsscmdr {
        #[inline(always)]
        fn default() -> Rsscmdr {
            Rsscmdr(0)
        }
    }
    impl core::fmt::Debug for Rsscmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsscmdr").field("rsscmd", &self.rsscmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rsscmdr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rsscmdr {
                rsscmd: u16,
            }
            let proxy = Rsscmdr { rsscmd: self.rsscmd() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "clock control, memory erase status and compensation cell registers security"]
        #[inline(always)]
        pub const fn syscfgsec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clock control, memory erase status and compensation cell registers security"]
        #[inline(always)]
        pub fn set_syscfgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Class B security"]
        #[inline(always)]
        pub const fn classbsec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Class B security"]
        #[inline(always)]
        pub fn set_classbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FPU security"]
        #[inline(always)]
        pub const fn fpusec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FPU security"]
        #[inline(always)]
        pub fn set_fpusec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("syscfgsec", &self.syscfgsec())
                .field("classbsec", &self.classbsec())
                .field("fpusec", &self.fpusec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Seccfgr {
                syscfgsec: bool,
                classbsec: bool,
                fpusec: bool,
            }
            let proxy = Seccfgr {
                syscfgsec: self.syscfgsec(),
                classbsec: self.classbsec(),
                fpusec: self.fpusec(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
