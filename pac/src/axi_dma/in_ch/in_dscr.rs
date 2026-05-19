#[doc = "Register `IN_DSCR` reader"]
pub type R = crate::R<InDscrSpec>;
#[doc = "Field `INLINK_DSCR` reader - The address of the current inlink descriptor x."]
pub type InlinkDscrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the current inlink descriptor x."]
    #[inline(always)]
    pub fn inlink_dscr(&self) -> InlinkDscrR {
        InlinkDscrR::new(self.bits)
    }
}
#[doc = "Current inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InDscrSpec;
impl crate::RegisterSpec for InDscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dscr::R`](R) reader structure"]
impl crate::Readable for InDscrSpec {}
#[doc = "`reset()` method sets IN_DSCR to value 0"]
impl crate::Resettable for InDscrSpec {}
