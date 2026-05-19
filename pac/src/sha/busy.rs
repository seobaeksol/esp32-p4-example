#[doc = "Register `BUSY` reader"]
pub type R = crate::R<BusySpec>;
#[doc = "Field `STATE` reader - Represents the states of SHA accelerator. \\\\ 0: idle\\\\ 1: busy\\\\"]
pub type StateR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents the states of SHA accelerator. \\\\ 0: idle\\\\ 1: busy\\\\"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 1) != 0)
    }
}
#[doc = "Represents if SHA Accelerator is busy or not\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusySpec;
impl crate::RegisterSpec for BusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busy::R`](R) reader structure"]
impl crate::Readable for BusySpec {}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BusySpec {}
