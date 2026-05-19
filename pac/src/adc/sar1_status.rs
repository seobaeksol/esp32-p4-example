#[doc = "Register `SAR1_STATUS` reader"]
pub type R = crate::R<Sar1StatusSpec>;
#[doc = "Field `SAR1_STATUS` reader - "]
pub type Sar1StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar1_status(&self) -> Sar1StatusR {
        Sar1StatusR::new(self.bits)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar1StatusSpec;
impl crate::RegisterSpec for Sar1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_status::R`](R) reader structure"]
impl crate::Readable for Sar1StatusSpec {}
#[doc = "`reset()` method sets SAR1_STATUS to value 0"]
impl crate::Resettable for Sar1StatusSpec {}
