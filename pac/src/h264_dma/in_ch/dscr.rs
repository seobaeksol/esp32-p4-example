#[doc = "Register `DSCR` reader"]
pub type R = crate::R<DscrSpec>;
#[doc = "Field `INLINK_DSCR` reader - The address of the next inlink descriptor address x."]
pub type InlinkDscrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the next inlink descriptor address x."]
    #[inline(always)]
    pub fn inlink_dscr(&self) -> InlinkDscrR {
        InlinkDscrR::new(self.bits)
    }
}
#[doc = "RX CHx next dscr addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscrSpec;
impl crate::RegisterSpec for DscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr::R`](R) reader structure"]
impl crate::Readable for DscrSpec {}
#[doc = "`reset()` method sets DSCR to value 0"]
impl crate::Resettable for DscrSpec {}
