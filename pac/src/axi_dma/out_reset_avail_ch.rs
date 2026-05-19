#[doc = "Register `OUT_RESET_AVAIL_CH%s` reader"]
pub type R = crate::R<OutResetAvailChSpec>;
#[doc = "Field `OUT_RESET_AVAIL` reader - tx chan0 reset valid reg."]
pub type OutResetAvailR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - tx chan0 reset valid reg."]
    #[inline(always)]
    pub fn out_reset_avail(&self) -> OutResetAvailR {
        OutResetAvailR::new((self.bits & 1) != 0)
    }
}
#[doc = "The tx channel %s reset valid_flag register.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_reset_avail_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutResetAvailChSpec;
impl crate::RegisterSpec for OutResetAvailChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_reset_avail_ch::R`](R) reader structure"]
impl crate::Readable for OutResetAvailChSpec {}
#[doc = "`reset()` method sets OUT_RESET_AVAIL_CH%s to value 0x01"]
impl crate::Resettable for OutResetAvailChSpec {
    const RESET_VALUE: u32 = 0x01;
}
