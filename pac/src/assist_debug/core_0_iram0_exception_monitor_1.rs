#[doc = "Register `CORE_0_IRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<Core0Iram0ExceptionMonitor1Spec>;
#[doc = "Field `CORE_0_IRAM0_RECORDING_ADDR_1` reader - reg_core_0_iram0_recording_addr_1"]
pub type Core0Iram0RecordingAddr1R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_IRAM0_RECORDING_WR_1` reader - reg_core_0_iram0_recording_wr_1"]
pub type Core0Iram0RecordingWr1R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_RECORDING_LOADSTORE_1` reader - reg_core_0_iram0_recording_loadstore_1"]
pub type Core0Iram0RecordingLoadstore1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - reg_core_0_iram0_recording_addr_1"]
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_1(&self) -> Core0Iram0RecordingAddr1R {
        Core0Iram0RecordingAddr1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - reg_core_0_iram0_recording_wr_1"]
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_1(&self) -> Core0Iram0RecordingWr1R {
        Core0Iram0RecordingWr1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reg_core_0_iram0_recording_loadstore_1"]
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_1(&self) -> Core0Iram0RecordingLoadstore1R {
        Core0Iram0RecordingLoadstore1R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "exception monitor status register1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_iram0_exception_monitor_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0Iram0ExceptionMonitor1Spec;
impl crate::RegisterSpec for Core0Iram0ExceptionMonitor1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_iram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for Core0Iram0ExceptionMonitor1Spec {}
#[doc = "`reset()` method sets CORE_0_IRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for Core0Iram0ExceptionMonitor1Spec {}
