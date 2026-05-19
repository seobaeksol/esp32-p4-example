#[doc = "Register `VAD_OB3` reader"]
pub type R = crate::R<VadOb3Spec>;
#[doc = "Field `NOISE_MEAN_OB` reader - Reg noise_mean observe signal"]
pub type NoiseMeanObR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg noise_mean observe signal"]
    #[inline(always)]
    pub fn noise_mean_ob(&self) -> NoiseMeanObR {
        NoiseMeanObR::new(self.bits)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadOb3Spec;
impl crate::RegisterSpec for VadOb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob3::R`](R) reader structure"]
impl crate::Readable for VadOb3Spec {}
#[doc = "`reset()` method sets VAD_OB3 to value 0"]
impl crate::Resettable for VadOb3Spec {}
