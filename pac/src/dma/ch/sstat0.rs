#[doc = "Register `SSTAT0` reader"]
pub type R = crate::R<Sstat0Spec>;
#[doc = "Field `CH1_SSTAT` reader - NA"]
pub type Ch1SstatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_sstat(&self) -> Ch1SstatR {
        Ch1SstatR::new(self.bits)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sstat0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sstat0Spec;
impl crate::RegisterSpec for Sstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstat0::R`](R) reader structure"]
impl crate::Readable for Sstat0Spec {}
#[doc = "`reset()` method sets SSTAT0 to value 0"]
impl crate::Resettable for Sstat0Spec {}
