#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Field `BUSY` reader - The status bits of ECDSA Accelerator. ECDSA is at 0: IDLE, 1: LOAD, 2: GET, 3: BUSY state."]
pub type BusyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - The status bits of ECDSA Accelerator. ECDSA is at 0: IDLE, 1: LOAD, 2: GET, 3: BUSY state."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 3) as u8)
    }
}
#[doc = "ECDSA status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for StateSpec {}
