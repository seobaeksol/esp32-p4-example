#[doc = "Register `OUT_DSCR_BF0` reader"]
pub type R = crate::R<OutDscrBf0Spec>;
#[doc = "Field `OUTLINK_DSCR_BF0_CH0` reader - Represents the address of the current transmit descriptor y that has already been fetched."]
pub type OutlinkDscrBf0Ch0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the current transmit descriptor y that has already been fetched."]
    #[inline(always)]
    pub fn outlink_dscr_bf0_ch0(&self) -> OutlinkDscrBf0Ch0R {
        OutlinkDscrBf0Ch0R::new(self.bits)
    }
}
#[doc = "The last transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutDscrBf0Spec;
impl crate::RegisterSpec for OutDscrBf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_dscr_bf0::R`](R) reader structure"]
impl crate::Readable for OutDscrBf0Spec {}
#[doc = "`reset()` method sets OUT_DSCR_BF0 to value 0"]
impl crate::Resettable for OutDscrBf0Spec {}
