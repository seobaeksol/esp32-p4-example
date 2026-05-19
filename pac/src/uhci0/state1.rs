#[doc = "Register `STATE1` reader"]
pub type R = crate::R<State1Spec>;
#[doc = "Field `ENCODE_STATE` reader - Indicates UHCI encoder status."]
pub type EncodeStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Indicates UHCI encoder status."]
    #[inline(always)]
    pub fn encode_state(&self) -> EncodeStateR {
        EncodeStateR::new((self.bits & 7) as u8)
    }
}
#[doc = "UHCI Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`state1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct State1Spec;
impl crate::RegisterSpec for State1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state1::R`](R) reader structure"]
impl crate::Readable for State1Spec {}
#[doc = "`reset()` method sets STATE1 to value 0"]
impl crate::Resettable for State1Spec {}
