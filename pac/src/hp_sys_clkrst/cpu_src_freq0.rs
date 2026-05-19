#[doc = "Register `CPU_SRC_FREQ0` reader"]
pub type R = crate::R<CpuSrcFreq0Spec>;
#[doc = "Field `CPU_SRC_FREQ` reader - cpu source clock frequency, step by 0.25MHz"]
pub type CpuSrcFreqR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - cpu source clock frequency, step by 0.25MHz"]
    #[inline(always)]
    pub fn cpu_src_freq(&self) -> CpuSrcFreqR {
        CpuSrcFreqR::new(self.bits)
    }
}
#[doc = "CPU Source Frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_src_freq0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuSrcFreq0Spec;
impl crate::RegisterSpec for CpuSrcFreq0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_src_freq0::R`](R) reader structure"]
impl crate::Readable for CpuSrcFreq0Spec {}
#[doc = "`reset()` method sets CPU_SRC_FREQ0 to value 0"]
impl crate::Resettable for CpuSrcFreq0Spec {}
