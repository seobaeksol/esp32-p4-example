#[doc = "Register `CORE_0_IRAM0_EXCEPTION_MONITOR_0` reader"]
pub type R = crate::R<Core0Iram0ExceptionMonitor0Spec>;
#[doc = "Field `CORE_0_IRAM0_RECORDING_ADDR_0` reader - reg_core_0_iram0_recording_addr_0"]
pub type Core0Iram0RecordingAddr0R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_IRAM0_RECORDING_WR_0` reader - reg_core_0_iram0_recording_wr_0"]
pub type Core0Iram0RecordingWr0R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_RECORDING_LOADSTORE_0` reader - reg_core_0_iram0_recording_loadstore_0"]
pub type Core0Iram0RecordingLoadstore0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - reg_core_0_iram0_recording_addr_0"]
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_0(&self) -> Core0Iram0RecordingAddr0R {
        Core0Iram0RecordingAddr0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - reg_core_0_iram0_recording_wr_0"]
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_0(&self) -> Core0Iram0RecordingWr0R {
        Core0Iram0RecordingWr0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reg_core_0_iram0_recording_loadstore_0"]
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_0(&self) -> Core0Iram0RecordingLoadstore0R {
        Core0Iram0RecordingLoadstore0R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "exception monitor status register0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_iram0_exception_monitor_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0Iram0ExceptionMonitor0Spec;
impl crate::RegisterSpec for Core0Iram0ExceptionMonitor0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_iram0_exception_monitor_0::R`](R) reader structure"]
impl crate::Readable for Core0Iram0ExceptionMonitor0Spec {}
#[doc = "`reset()` method sets CORE_0_IRAM0_EXCEPTION_MONITOR_0 to value 0"]
impl crate::Resettable for Core0Iram0ExceptionMonitor0Spec {}
