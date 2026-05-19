#[doc = "Register `IN_DSCR` reader"]
pub type R = crate::R<InDscrSpec>;
#[doc = "Field `INLINK_DSCR_CH0` reader - Represents the address of the next receive descriptor x+1 pointed by the current receive descriptor that has already been fetched."]
pub type InlinkDscrCh0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the next receive descriptor x+1 pointed by the current receive descriptor that has already been fetched."]
    #[inline(always)]
    pub fn inlink_dscr_ch0(&self) -> InlinkDscrCh0R {
        InlinkDscrCh0R::new(self.bits)
    }
}
#[doc = "Current receive descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InDscrSpec;
impl crate::RegisterSpec for InDscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dscr::R`](R) reader structure"]
impl crate::Readable for InDscrSpec {}
#[doc = "`reset()` method sets IN_DSCR to value 0"]
impl crate::Resettable for InDscrSpec {}
