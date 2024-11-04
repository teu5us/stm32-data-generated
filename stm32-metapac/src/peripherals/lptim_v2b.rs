#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Low power timer with Output Compare"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptim {
    ptr: *mut u8,
}
unsafe impl Send for Lptim {}
unsafe impl Sync for Lptim {}
impl Lptim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LPTIM interrupt and status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "LPTIM interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "LPTIM interrupt enable register."]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "LPTIM configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "LPTIM control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "LPTIM compare register 1."]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr
                    .add(0x14usize + ([0usize, 32usize, 36usize, 40usize][n] as usize)) as _,
            )
        }
    }
    #[doc = "LPTIM autoreload register."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "LPTIM counter register."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "LPTIM configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "LPTIM repetition register."]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "LPTIM capture/compare mode register."]
    #[inline(always)]
    pub const fn ccmr(self, n: usize) -> crate::common::Reg<regs::Ccmr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize + n * 1usize) as _) }
    }
}
pub mod regs {
    #[doc = "LPTIM autoreload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\\[15:0\\]
value."]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto reload value ARR is the autoreload value for the LPTIM. This value must be strictly greater than the CCRx\\[15:0\\]
value."]
        #[inline(always)]
        pub fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Arr {
        #[inline(always)]
        fn default() -> Arr {
            Arr(0)
        }
    }
    #[doc = "LPTIM capture/compare mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr(pub u32);
    impl Ccmr {
        #[doc = "Capture/compare selection. This bitfield defines the direction of the channel input (capture) or output mode."]
        #[inline(always)]
        pub const fn ccsel(&self, n: usize) -> super::vals::Ccsel {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Ccsel::from_bits(val as u8)
        }
        #[doc = "Capture/compare selection. This bitfield defines the direction of the channel input (capture) or output mode."]
        #[inline(always)]
        pub fn set_ccsel(&mut self, n: usize, val: super::vals::Ccsel) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Capture/compare output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/compare output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
        #[inline(always)]
        pub const fn ccp_input(&self, n: usize) -> super::vals::CcpInput {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcpInput::from_bits(val as u8)
        }
        #[doc = "Capture/compare output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
        #[inline(always)]
        pub fn set_ccp_input(&mut self, n: usize, val: super::vals::CcpInput) {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Capture/compare output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
        #[inline(always)]
        pub const fn ccp_output(&self, n: usize) -> super::vals::CcpOutput {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcpOutput::from_bits(val as u8)
        }
        #[doc = "Capture/compare output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
        #[inline(always)]
        pub fn set_ccp_output(&mut self, n: usize, val: super::vals::CcpOutput) {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> super::vals::Filter {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Filter::from_bits(val as u8)
        }
        #[doc = "Input capture prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
        #[inline(always)]
        pub fn set_icpsc(&mut self, n: usize, val: super::vals::Filter) {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::Filter {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Filter::from_bits(val as u8)
        }
        #[doc = "Input capture filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
        #[inline(always)]
        pub fn set_icf(&mut self, n: usize, val: super::vals::Filter) {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Ccmr {
        #[inline(always)]
        fn default() -> Ccmr {
            Ccmr(0)
        }
    }
    #[doc = "LPTIM compare register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed. If LPTIM does not implement any channel: The compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on LPTIM output."]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "LPTIM configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM uses:."]
        #[inline(always)]
        pub const fn cksel(&self) -> super::vals::ClockSource {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::ClockSource::from_bits(val as u8)
        }
        #[doc = "Clock selector The CKSEL bit selects which clock source the LPTIM uses:."]
        #[inline(always)]
        pub fn set_cksel(&mut self, val: super::vals::ClockSource) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
        #[inline(always)]
        pub const fn ckpol(&self) -> super::vals::Ckpol {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Ckpol::from_bits(val as u8)
        }
        #[doc = "Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
        #[inline(always)]
        pub fn set_ckpol(&mut self, val: super::vals::Ckpol) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
        #[inline(always)]
        pub const fn ckflt(&self) -> super::vals::Filter {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Filter::from_bits(val as u8)
        }
        #[doc = "Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
        #[inline(always)]
        pub fn set_ckflt(&mut self, val: super::vals::Filter) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
        #[inline(always)]
        pub const fn trgflt(&self) -> super::vals::Filter {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Filter::from_bits(val as u8)
        }
        #[doc = "Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
        #[inline(always)]
        pub fn set_trgflt(&mut self, val: super::vals::Filter) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:."]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Presc {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::Presc::from_bits(val as u8)
        }
        #[doc = "Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:."]
        #[inline(always)]
        pub fn set_presc(&mut self, val: super::vals::Presc) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
        #[inline(always)]
        pub const fn trigsel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
        #[inline(always)]
        pub fn set_trigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:."]
        #[inline(always)]
        pub const fn trigen(&self) -> super::vals::Trigen {
            let val = (self.0 >> 17usize) & 0x03;
            super::vals::Trigen::from_bits(val as u8)
        }
        #[doc = "Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:."]
        #[inline(always)]
        pub fn set_trigen(&mut self, val: super::vals::Trigen) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
        }
        #[doc = "Timeout enable The TIMOUT bit controls the Timeout feature."]
        #[inline(always)]
        pub const fn timout(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout enable The TIMOUT bit controls the Timeout feature."]
        #[inline(always)]
        pub fn set_timout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Waveform shape The WAVE bit controls the output shape."]
        #[inline(always)]
        pub const fn wave(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Waveform shape The WAVE bit controls the output shape."]
        #[inline(always)]
        pub fn set_wave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality."]
        #[inline(always)]
        pub const fn preload(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality."]
        #[inline(always)]
        pub fn set_preload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:."]
        #[inline(always)]
        pub const fn countmode(&self) -> super::vals::ClockSource {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::ClockSource::from_bits(val as u8)
        }
        #[doc = "counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:."]
        #[inline(always)]
        pub fn set_countmode(&mut self, val: super::vals::ClockSource) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn enc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_enc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "LPTIM configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to."]
        #[inline(always)]
        pub const fn insel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "LPTIM input 1 selection The IN1SEL bits control the LPTIM input 1 multiplexer, which connects LPTIM input 1 to one of the available inputs. For connection details refer to."]
        #[inline(always)]
        pub fn set_insel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to."]
        #[inline(always)]
        pub const fn icsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "LPTIM input capture 1 selection The IC1SEL bits control the LPTIM Input capture 1 multiplexer, which connects LPTIM Input capture 1 to one of the available inputs. For connection details refer to."]
        #[inline(always)]
        pub fn set_icsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "LPTIM counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    #[doc = "LPTIM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "LPTIM enable The ENABLE bit is set and cleared by software."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM enable The ENABLE bit is set and cleared by software."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
        #[inline(always)]
        pub const fn sngstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM stops at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It is automatically reset by hardware."]
        #[inline(always)]
        pub fn set_sngstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
        #[inline(always)]
        pub const fn cntstrt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\\[1:0\\]
= ‘00’), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\\[1:0\\]
different than ‘00’), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer does not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware."]
        #[inline(always)]
        pub fn set_cntstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
        #[inline(always)]
        pub const fn countrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit triggers a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). This bit can be set only when the LPTIM is enabled. It is automatically reset by hardware. COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'."]
        #[inline(always)]
        pub fn set_countrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
        #[inline(always)]
        pub const fn rstare(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register asynchronously resets LPTIM_CNT register content. This bit can be set only when the LPTIM is enabled."]
        #[inline(always)]
        pub fn set_rstare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "LPTIM interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier(pub u32);
    impl Dier {
        #[doc = "Capture/compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt enable."]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match Interrupt Enable."]
        #[inline(always)]
        pub const fn arrmie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match Interrupt Enable."]
        #[inline(always)]
        pub fn set_arrmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger valid edge Interrupt Enable."]
        #[inline(always)]
        pub const fn exttrigie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger valid edge Interrupt Enable."]
        #[inline(always)]
        pub fn set_exttrigie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare register 1 update OK interrupt enable."]
        #[inline(always)]
        pub const fn cmpokie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 16usize, 17usize, 18usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare register 1 update OK interrupt enable."]
        #[inline(always)]
        pub fn set_cmpokie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 16usize, 17usize, 18usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload register update OK Interrupt Enable."]
        #[inline(always)]
        pub const fn arrokie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK Interrupt Enable."]
        #[inline(always)]
        pub fn set_arrokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn upie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_upie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn downie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_downie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update event interrupt enable."]
        #[inline(always)]
        pub const fn ueie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Update event interrupt enable."]
        #[inline(always)]
        pub fn set_ueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK interrupt Enable."]
        #[inline(always)]
        pub const fn repokie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK interrupt Enable."]
        #[inline(always)]
        pub fn set_repokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn ccoie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 over-capture interrupt enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_ccoie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 DMA request enable Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Dier {
        #[inline(always)]
        fn default() -> Dier {
            Dier(0)
        }
    }
    #[doc = "LPTIM interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn cccf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_cccf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn arrmcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_arrmcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn exttrigcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_exttrigcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn cmpokcf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 16usize, 17usize, 18usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_cmpokcf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 16usize, 17usize, 18usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn arrokcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_arrokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn upcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_upcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn downcf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_downcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn uecf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_uecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn repokcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_repokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn ccocf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 over-capture clear flag Writing 1 to this bit clears the CC1OF flag in the LPTIM_ISR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_ccocf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub const fn dierokcf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register."]
        #[inline(always)]
        pub fn set_dierokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    #[doc = "LPTIM interrupt and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Compare 1 interrupt flag The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. The CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 interrupt flag The CC1IF flag is set by hardware to inform application that LPTIM_CNT register value matches the compare register's value. The CC1IF flag can be cleared by writing 1 to the CC1CF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + ([0usize, 9usize, 10usize, 11usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn arrm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT register’s value reached the LPTIM_ARR register’s value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_arrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn cmpok(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 16usize, 17usize, 18usize][n] as usize);
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare register 1 update OK CMP1OK is set by hardware to inform application that the APB bus write operation to the LPTIM_CCR1 register has been successfully completed. CMP1OK flag can be cleared by writing 1 to the CMP1OKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_cmpok(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + ([0usize, 16usize, 17usize, 18usize][n] as usize);
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn arrok(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_arrok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn down(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn ue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM update event occurred UE is set by hardware to inform application that an update event was generated. UE flag can be cleared by writing 1 to the UECF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_ue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn repok(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK REPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_RCR register has been successfully completed. REPOK flag can be cleared by writing 1 to the REPOKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_repok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture 1 over-capture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing 1 to the CC1OCF bit in the LPTIM_ICR register. Note: If LPTIM does not implement at least 1 channel this bit is reserved. Please refer to."]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub const fn dierok(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable register update OK DIEROK is set by hardware to inform application that the APB bus write operation to the LPTIM_DIER register has been successfully completed. DIEROK flag can be cleared by writing 1 to the DIEROKCF bit in the LPTIM_ICR register."]
        #[inline(always)]
        pub fn set_dierok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    #[doc = "LPTIM repetition register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "Repetition register value REP is the repetition value for the LPTIM."]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition register value REP is the repetition value for the LPTIM."]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rcr {
        #[inline(always)]
        fn default() -> Rcr {
            Rcr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcpInput {
        RISING = 0x0,
        FALLING = 0x01,
        _RESERVED_2 = 0x02,
        BOTH = 0x03,
    }
    impl CcpInput {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcpInput {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcpInput {
        #[inline(always)]
        fn from(val: u8) -> CcpInput {
            CcpInput::from_bits(val)
        }
    }
    impl From<CcpInput> for u8 {
        #[inline(always)]
        fn from(val: CcpInput) -> u8 {
            CcpInput::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcpOutput {
        ACTIVEHIGH = 0x0,
        ACTIVELOW = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CcpOutput {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcpOutput {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcpOutput {
        #[inline(always)]
        fn from(val: u8) -> CcpOutput {
            CcpOutput::from_bits(val)
        }
    }
    impl From<CcpOutput> for u8 {
        #[inline(always)]
        fn from(val: CcpOutput) -> u8 {
            CcpOutput::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ccsel {
        #[doc = "channel is configured in output PWM mode"]
        OUTPUTCOMPARE = 0x0,
        #[doc = "channel is configured in input capture mode"]
        INPUTCAPTURE = 0x01,
    }
    impl Ccsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccsel {
        #[inline(always)]
        fn from(val: u8) -> Ccsel {
            Ccsel::from_bits(val)
        }
    }
    impl From<Ccsel> for u8 {
        #[inline(always)]
        fn from(val: Ccsel) -> u8 {
            Ccsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ckpol {
        #[doc = "the rising edge is the active edge used for counting. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active."]
        RISING = 0x0,
        #[doc = "the falling edge is the active edge used for counting. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active."]
        FALLING = 0x01,
        #[doc = "both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active."]
        BOTH = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckpol {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckpol {
        #[inline(always)]
        fn from(val: u8) -> Ckpol {
            Ckpol::from_bits(val)
        }
    }
    impl From<Ckpol> for u8 {
        #[inline(always)]
        fn from(val: Ckpol) -> u8 {
            Ckpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ClockSource {
        #[doc = "clocked by internal clock source (APB clock or any of the embedded oscillators)"]
        INTERNAL = 0x0,
        #[doc = "clocked by an external clock source through the LPTIM external Input1"]
        EXTERNAL = 0x01,
    }
    impl ClockSource {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ClockSource {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ClockSource {
        #[inline(always)]
        fn from(val: u8) -> ClockSource {
            ClockSource::from_bits(val)
        }
    }
    impl From<ClockSource> for u8 {
        #[inline(always)]
        fn from(val: ClockSource) -> u8 {
            ClockSource::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Filter {
        COUNT1 = 0x0,
        COUNT2 = 0x01,
        COUNT4 = 0x02,
        COUNT8 = 0x03,
    }
    impl Filter {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Filter {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Filter {
        #[inline(always)]
        fn from(val: u8) -> Filter {
            Filter::from_bits(val)
        }
    }
    impl From<Filter> for u8 {
        #[inline(always)]
        fn from(val: Filter) -> u8 {
            Filter::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Presc {
        DIV1 = 0x0,
        DIV2 = 0x01,
        DIV4 = 0x02,
        DIV8 = 0x03,
        DIV16 = 0x04,
        DIV32 = 0x05,
        DIV64 = 0x06,
        DIV128 = 0x07,
    }
    impl Presc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Presc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Presc {
        #[inline(always)]
        fn from(val: u8) -> Presc {
            Presc::from_bits(val)
        }
    }
    impl From<Presc> for u8 {
        #[inline(always)]
        fn from(val: Presc) -> u8 {
            Presc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Trigen {
        #[doc = "software trigger (counting start is initiated by software)"]
        SOFTWARE = 0x0,
        #[doc = "rising edge is the active edge"]
        RISINGEDGE = 0x01,
        #[doc = "falling edge is the active edge"]
        FALLINGEDGE = 0x02,
        #[doc = "both edges are active edges"]
        BOTHEDGE = 0x03,
    }
    impl Trigen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trigen {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trigen {
        #[inline(always)]
        fn from(val: u8) -> Trigen {
            Trigen::from_bits(val)
        }
    }
    impl From<Trigen> for u8 {
        #[inline(always)]
        fn from(val: Trigen) -> u8 {
            Trigen::to_bits(val)
        }
    }
}
