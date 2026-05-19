#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_2` reader"]
pub type R = crate::R<Core1Dram0ExceptionMonitor2Spec>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_PC_0` reader - reg_core_1_dram0_recording_pc_0"]
pub type Core1Dram0RecordingPc0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_1_dram0_recording_pc_0"]
    #[inline(always)]
    pub fn core_1_dram0_recording_pc_0(&self) -> Core1Dram0RecordingPc0R {
        Core1Dram0RecordingPc0R::new(self.bits)
    }
}
#[doc = "exception monitor status register4\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_dram0_exception_monitor_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1Dram0ExceptionMonitor2Spec;
impl crate::RegisterSpec for Core1Dram0ExceptionMonitor2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_exception_monitor_2::R`](R) reader structure"]
impl crate::Readable for Core1Dram0ExceptionMonitor2Spec {}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_2 to value 0"]
impl crate::Resettable for Core1Dram0ExceptionMonitor2Spec {}
