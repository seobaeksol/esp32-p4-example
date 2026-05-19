#[doc = "Register `LP_CPU_EXC_PC` reader"]
pub type R = crate::R<LpCpuExcPcSpec>;
#[doc = "Field `LP_CPU_EXC_PC` reader - need_des"]
pub type LpCpuExcPcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc_pc(&self) -> LpCpuExcPcR {
        LpCpuExcPcR::new(self.bits)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_exc_pc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpCpuExcPcSpec;
impl crate::RegisterSpec for LpCpuExcPcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_cpu_exc_pc::R`](R) reader structure"]
impl crate::Readable for LpCpuExcPcSpec {}
#[doc = "`reset()` method sets LP_CPU_EXC_PC to value 0"]
impl crate::Resettable for LpCpuExcPcSpec {}
