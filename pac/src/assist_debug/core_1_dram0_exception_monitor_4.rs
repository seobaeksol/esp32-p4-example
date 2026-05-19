#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_4` reader"]
pub type R = crate::R<Core1Dram0ExceptionMonitor4Spec>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_ADDR_1` reader - reg_core_1_dram0_recording_addr_1"]
pub type Core1Dram0RecordingAddr1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - reg_core_1_dram0_recording_addr_1"]
    #[inline(always)]
    pub fn core_1_dram0_recording_addr_1(&self) -> Core1Dram0RecordingAddr1R {
        Core1Dram0RecordingAddr1R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "exception monitor status register6\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_dram0_exception_monitor_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1Dram0ExceptionMonitor4Spec;
impl crate::RegisterSpec for Core1Dram0ExceptionMonitor4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_exception_monitor_4::R`](R) reader structure"]
impl crate::Readable for Core1Dram0ExceptionMonitor4Spec {}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_4 to value 0"]
impl crate::Resettable for Core1Dram0ExceptionMonitor4Spec {}
