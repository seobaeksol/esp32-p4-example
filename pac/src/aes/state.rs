#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Field `STATE` reader - Represents the working status of the AES accelerator. \\\\ In Typical AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: No effect\\\\ 3: No effect\\\\ In DMA-AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: DONE\\\\ 3: No effect\\\\"]
pub type StateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the working status of the AES accelerator. \\\\ In Typical AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: No effect\\\\ 3: No effect\\\\ In DMA-AES working mode:\\\\ 0: IDLE\\\\ 1: WORK\\\\ 2: DONE\\\\ 3: No effect\\\\"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 3) as u8)
    }
}
#[doc = "Operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for StateSpec {}
