#[doc = "Register `STATUS4` reader"]
pub type R = crate::R<Status4Spec>;
#[doc = "Field `HFM_BITSTREAM` reader - the hufman bitstream during encoding process"]
pub type HfmBitstreamR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - the hufman bitstream during encoding process"]
    #[inline(always)]
    pub fn hfm_bitstream(&self) -> HfmBitstreamR {
        HfmBitstreamR::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`status4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status4Spec;
impl crate::RegisterSpec for Status4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status4::R`](R) reader structure"]
impl crate::Readable for Status4Spec {}
#[doc = "`reset()` method sets STATUS4 to value 0"]
impl crate::Resettable for Status4Spec {}
