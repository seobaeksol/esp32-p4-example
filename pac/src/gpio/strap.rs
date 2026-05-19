#[doc = "Register `STRAP` reader"]
pub type R = crate::R<StrapSpec>;
#[doc = "Field `STRAPPING` reader - pad strapping register"]
pub type StrappingR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - pad strapping register"]
    #[inline(always)]
    pub fn strapping(&self) -> StrappingR {
        StrappingR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "pad strapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`strap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrapSpec;
impl crate::RegisterSpec for StrapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`strap::R`](R) reader structure"]
impl crate::Readable for StrapSpec {}
#[doc = "`reset()` method sets STRAP to value 0"]
impl crate::Resettable for StrapSpec {}
