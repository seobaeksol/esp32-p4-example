#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_5` reader"]
pub type R = crate::R<Core1Dram0ExceptionMonitor5Spec>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_PC_1` reader - reg_core_1_dram0_recording_pc_1"]
pub type Core1Dram0RecordingPc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_1_dram0_recording_pc_1"]
    #[inline(always)]
    pub fn core_1_dram0_recording_pc_1(&self) -> Core1Dram0RecordingPc1R {
        Core1Dram0RecordingPc1R::new(self.bits)
    }
}
#[doc = "exception monitor status register7\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_dram0_exception_monitor_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1Dram0ExceptionMonitor5Spec;
impl crate::RegisterSpec for Core1Dram0ExceptionMonitor5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_dram0_exception_monitor_5::R`](R) reader structure"]
impl crate::Readable for Core1Dram0ExceptionMonitor5Spec {}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_5 to value 0"]
impl crate::Resettable for Core1Dram0ExceptionMonitor5Spec {}
