#[doc = "Register `VAD_OB2` reader"]
pub type R = crate::R<VadOb2Spec>;
#[doc = "Field `NOISE_AMP_OB` reader - Reg noise_amp observe signal"]
pub type NoiseAmpObR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg noise_amp observe signal"]
    #[inline(always)]
    pub fn noise_amp_ob(&self) -> NoiseAmpObR {
        NoiseAmpObR::new(self.bits)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadOb2Spec;
impl crate::RegisterSpec for VadOb2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob2::R`](R) reader structure"]
impl crate::Readable for VadOb2Spec {}
#[doc = "`reset()` method sets VAD_OB2 to value 0"]
impl crate::Resettable for VadOb2Spec {}
