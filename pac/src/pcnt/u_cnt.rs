#[doc = "Register `U%s_CNT` reader"]
pub type R = crate::R<UCntSpec>;
#[doc = "Field `CNT` reader - This register stores the current pulse count value for unit %s."]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit %s."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Counter value for unit %s\n\nYou can [`read`](crate::Reg::read) this register and get [`u_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCntSpec;
impl crate::RegisterSpec for UCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_cnt::R`](R) reader structure"]
impl crate::Readable for UCntSpec {}
#[doc = "`reset()` method sets U%s_CNT to value 0"]
impl crate::Resettable for UCntSpec {}
