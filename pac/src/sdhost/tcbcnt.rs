#[doc = "Register `TCBCNT` reader"]
pub type R = crate::R<TcbcntSpec>;
#[doc = "Field `TCBCNT` reader - Number of bytes transferred by CIU unit to card."]
pub type TcbcntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn tcbcnt(&self) -> TcbcntR {
        TcbcntR::new(self.bits)
    }
}
#[doc = "Transferred byte count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcbcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcbcntSpec;
impl crate::RegisterSpec for TcbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcbcnt::R`](R) reader structure"]
impl crate::Readable for TcbcntSpec {}
#[doc = "`reset()` method sets TCBCNT to value 0"]
impl crate::Resettable for TcbcntSpec {}
