#[doc = "Register `DSCR_BF1` reader"]
pub type R = crate::R<DscrBf1Spec>;
#[doc = "Field `OUTLINK_DSCR_BF1` reader - The address of the second-to-last outlink descriptor's next address y-2."]
pub type OutlinkDscrBf1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the second-to-last outlink descriptor's next address y-2."]
    #[inline(always)]
    pub fn outlink_dscr_bf1(&self) -> OutlinkDscrBf1R {
        OutlinkDscrBf1R::new(self.bits)
    }
}
#[doc = "TX CHx second-to-last dscr addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr_bf1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscrBf1Spec;
impl crate::RegisterSpec for DscrBf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscr_bf1::R`](R) reader structure"]
impl crate::Readable for DscrBf1Spec {}
#[doc = "`reset()` method sets DSCR_BF1 to value 0"]
impl crate::Resettable for DscrBf1Spec {}
