#[doc = "Register `GMAC_CTRL1` reader"]
pub type R = crate::R<GmacCtrl1Spec>;
#[doc = "Field `PTP_TIMESTAMP_L` reader - N/A"]
pub type PtpTimestampLR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn ptp_timestamp_l(&self) -> PtpTimestampLR {
        PtpTimestampLR::new(self.bits)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacCtrl1Spec;
impl crate::RegisterSpec for GmacCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_ctrl1::R`](R) reader structure"]
impl crate::Readable for GmacCtrl1Spec {}
#[doc = "`reset()` method sets GMAC_CTRL1 to value 0"]
impl crate::Resettable for GmacCtrl1Spec {}
