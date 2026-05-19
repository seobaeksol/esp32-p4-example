#[doc = "Register `RECIVE_SEQ` reader"]
pub type R = crate::R<ReciveSeqSpec>;
#[doc = "Field `RECIVE_SEQ` reader - High speed sdio pad bist recive sequence"]
pub type ReciveSeqR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - High speed sdio pad bist recive sequence"]
    #[inline(always)]
    pub fn recive_seq(&self) -> ReciveSeqR {
        ReciveSeqR::new(self.bits)
    }
}
#[doc = "High speed sdio pad bist recive sequence\n\nYou can [`read`](crate::Reg::read) this register and get [`recive_seq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReciveSeqSpec;
impl crate::RegisterSpec for ReciveSeqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`recive_seq::R`](R) reader structure"]
impl crate::Readable for ReciveSeqSpec {}
#[doc = "`reset()` method sets RECIVE_SEQ to value 0"]
impl crate::Resettable for ReciveSeqSpec {}
