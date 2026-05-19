#[doc = "Register `VAD_OB7` reader"]
pub type R = crate::R<VadOb7Spec>;
#[doc = "Field `ENERGY_LOW_OB` reader - Reg energy bit 31~0 observe signal"]
pub type EnergyLowObR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reg energy bit 31~0 observe signal"]
    #[inline(always)]
    pub fn energy_low_ob(&self) -> EnergyLowObR {
        EnergyLowObR::new(self.bits)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`vad_ob7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VadOb7Spec;
impl crate::RegisterSpec for VadOb7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob7::R`](R) reader structure"]
impl crate::Readable for VadOb7Spec {}
#[doc = "`reset()` method sets VAD_OB7 to value 0"]
impl crate::Resettable for VadOb7Spec {}
