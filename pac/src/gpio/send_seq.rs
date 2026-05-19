#[doc = "Register `SEND_SEQ` reader"]
pub type R = crate::R<SendSeqSpec>;
#[doc = "Register `SEND_SEQ` writer"]
pub type W = crate::W<SendSeqSpec>;
#[doc = "Field `SEND_SEQ` reader - High speed sdio pad bist send sequence"]
pub type SendSeqR = crate::FieldReader<u32>;
#[doc = "Field `SEND_SEQ` writer - High speed sdio pad bist send sequence"]
pub type SendSeqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - High speed sdio pad bist send sequence"]
    #[inline(always)]
    pub fn send_seq(&self) -> SendSeqR {
        SendSeqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High speed sdio pad bist send sequence"]
    #[inline(always)]
    pub fn send_seq(&mut self) -> SendSeqW<'_, SendSeqSpec> {
        SendSeqW::new(self, 0)
    }
}
#[doc = "High speed sdio pad bist send sequence\n\nYou can [`read`](crate::Reg::read) this register and get [`send_seq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`send_seq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SendSeqSpec;
impl crate::RegisterSpec for SendSeqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`send_seq::R`](R) reader structure"]
impl crate::Readable for SendSeqSpec {}
#[doc = "`write(|w| ..)` method takes [`send_seq::W`](W) writer structure"]
impl crate::Writable for SendSeqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEND_SEQ to value 0x1234_5678"]
impl crate::Resettable for SendSeqSpec {
    const RESET_VALUE: u32 = 0x1234_5678;
}
