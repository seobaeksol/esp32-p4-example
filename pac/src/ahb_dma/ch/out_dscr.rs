#[doc = "Register `OUT_DSCR` reader"]
pub type R = crate::R<OutDscrSpec>;
#[doc = "Field `OUTLINK_DSCR_CH0` reader - Represents the address of the next transmit descriptor y+1 pointed by the current transmit descriptor that has already been fetched."]
pub type OutlinkDscrCh0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the next transmit descriptor y+1 pointed by the current transmit descriptor that has already been fetched."]
    #[inline(always)]
    pub fn outlink_dscr_ch0(&self) -> OutlinkDscrCh0R {
        OutlinkDscrCh0R::new(self.bits)
    }
}
#[doc = "Current transmit descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutDscrSpec;
impl crate::RegisterSpec for OutDscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_dscr::R`](R) reader structure"]
impl crate::Readable for OutDscrSpec {}
#[doc = "`reset()` method sets OUT_DSCR to value 0"]
impl crate::Resettable for OutDscrSpec {}
