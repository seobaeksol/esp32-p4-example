#[doc = "Register `IN_RESET_AVAIL_CH%s` reader"]
pub type R = crate::R<InResetAvailChSpec>;
#[doc = "Field `IN_RESET_AVAIL` reader - rx chan0 reset valid reg."]
pub type InResetAvailR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - rx chan0 reset valid reg."]
    #[inline(always)]
    pub fn in_reset_avail(&self) -> InResetAvailR {
        InResetAvailR::new((self.bits & 1) != 0)
    }
}
#[doc = "The rx channel %s reset valid_flag register.\n\nYou can [`read`](crate::Reg::read) this register and get [`in_reset_avail_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InResetAvailChSpec;
impl crate::RegisterSpec for InResetAvailChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_reset_avail_ch::R`](R) reader structure"]
impl crate::Readable for InResetAvailChSpec {}
#[doc = "`reset()` method sets IN_RESET_AVAIL_CH%s to value 0x01"]
impl crate::Resettable for InResetAvailChSpec {
    const RESET_VALUE: u32 = 0x01;
}
