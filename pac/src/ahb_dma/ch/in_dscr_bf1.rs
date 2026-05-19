#[doc = "Register `IN_DSCR_BF1` reader"]
pub type R = crate::R<InDscrBf1Spec>;
#[doc = "Field `INLINK_DSCR_BF1_CH0` reader - Represents the address of the previous receive descriptor x-1 that has already been fetched."]
pub type InlinkDscrBf1Ch0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the previous receive descriptor x-1 that has already been fetched."]
    #[inline(always)]
    pub fn inlink_dscr_bf1_ch0(&self) -> InlinkDscrBf1Ch0R {
        InlinkDscrBf1Ch0R::new(self.bits)
    }
}
#[doc = "The second-to-last receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InDscrBf1Spec;
impl crate::RegisterSpec for InDscrBf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dscr_bf1::R`](R) reader structure"]
impl crate::Readable for InDscrBf1Spec {}
#[doc = "`reset()` method sets IN_DSCR_BF1 to value 0"]
impl crate::Resettable for InDscrBf1Spec {}
