#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_3` reader"]
pub type R = crate::R<Core0Dram0ExceptionMonitor3Spec>;
#[doc = "Field `CORE_0_DRAM0_RECORDING_WR_1` reader - reg_core_0_dram0_recording_wr_1"]
pub type Core0Dram0RecordingWr1R = crate::BitReader;
#[doc = "Field `CORE_0_DRAM0_RECORDING_BYTEEN_1` reader - reg_core_0_dram0_recording_byteen_1"]
pub type Core0Dram0RecordingByteen1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - reg_core_0_dram0_recording_wr_1"]
    #[inline(always)]
    pub fn core_0_dram0_recording_wr_1(&self) -> Core0Dram0RecordingWr1R {
        Core0Dram0RecordingWr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - reg_core_0_dram0_recording_byteen_1"]
    #[inline(always)]
    pub fn core_0_dram0_recording_byteen_1(&self) -> Core0Dram0RecordingByteen1R {
        Core0Dram0RecordingByteen1R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[doc = "exception monitor status register5\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core0Dram0ExceptionMonitor3Spec;
impl crate::RegisterSpec for Core0Dram0ExceptionMonitor3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_dram0_exception_monitor_3::R`](R) reader structure"]
impl crate::Readable for Core0Dram0ExceptionMonitor3Spec {}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_3 to value 0"]
impl crate::Resettable for Core0Dram0ExceptionMonitor3Spec {}
