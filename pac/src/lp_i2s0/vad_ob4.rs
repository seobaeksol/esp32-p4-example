#[doc = "Register `VAD_OB4` reader"]
pub type R = crate::R<VadOb4Spec>;
#[doc = "Field `NOISE_STD_OB` reader - Reg noise_std observe signal"]
pub type NoiseStdObR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg noise_std observe signal"]
    #[inline(always)]
    pub fn noise_std_ob(&self) -> NoiseStdObR {
        NoiseStdObR::new(self.bits)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadOb4Spec;
impl crate::RegisterSpec for VadOb4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob4::R`](R) reader structure"]
impl crate::Readable for VadOb4Spec {}
#[doc = "`reset()` method sets VAD_OB4 to value 0"]
impl crate::Resettable for VadOb4Spec {}
