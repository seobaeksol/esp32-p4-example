#[doc = "Register `TX_TAILING_BITS` reader"]
pub type R = crate::R<TxTailingBitsSpec>;
#[doc = "Register `TX_TAILING_BITS` writer"]
pub type W = crate::W<TxTailingBitsSpec>;
#[doc = "Field `TX_TAILING_BITS` reader - write this bits to specify the extra data bit length after getting EOF"]
pub type TxTailingBitsR = crate::FieldReader<u16>;
#[doc = "Field `TX_TAILING_BITS` writer - write this bits to specify the extra data bit length after getting EOF"]
pub type TxTailingBitsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - write this bits to specify the extra data bit length after getting EOF"]
    #[inline(always)]
    pub fn tx_tailing_bits(&self) -> TxTailingBitsR {
        TxTailingBitsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - write this bits to specify the extra data bit length after getting EOF"]
    #[inline(always)]
    pub fn tx_tailing_bits(&mut self) -> TxTailingBitsW<'_, TxTailingBitsSpec> {
        TxTailingBitsW::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_tailing_bits::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_tailing_bits::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxTailingBitsSpec;
impl crate::RegisterSpec for TxTailingBitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_tailing_bits::R`](R) reader structure"]
impl crate::Readable for TxTailingBitsSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_tailing_bits::W`](W) writer structure"]
impl crate::Writable for TxTailingBitsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_TAILING_BITS to value 0"]
impl crate::Resettable for TxTailingBitsSpec {}
