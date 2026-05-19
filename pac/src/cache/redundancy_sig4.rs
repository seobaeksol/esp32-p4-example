#[doc = "Register `REDUNDANCY_SIG4` reader"]
pub type R = crate::R<RedundancySig4Spec>;
#[doc = "Field `REDCY_SIG4` reader - Those bits are prepared for ECO."]
pub type RedcySig4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn redcy_sig4(&self) -> RedcySig4R {
        RedcySig4R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Cache redundancy signal 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`redundancy_sig4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RedundancySig4Spec;
impl crate::RegisterSpec for RedundancySig4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redundancy_sig4::R`](R) reader structure"]
impl crate::Readable for RedundancySig4Spec {}
#[doc = "`reset()` method sets REDUNDANCY_SIG4 to value 0"]
impl crate::Resettable for RedundancySig4Spec {}
