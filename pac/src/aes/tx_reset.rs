#[doc = "Register `TX_RESET` writer"]
pub type W = crate::W<TxResetSpec>;
#[doc = "Field `TX_RESET` writer - Set this bit to reset tx_fifo under dma_aes working mode."]
pub type TxResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to reset tx_fifo under dma_aes working mode."]
    #[inline(always)]
    pub fn tx_reset(&mut self) -> TxResetW<'_, TxResetSpec> {
        TxResetW::new(self, 0)
    }
}
#[doc = "AES-DMA reset tx-fifo register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxResetSpec;
impl crate::RegisterSpec for TxResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_reset::W`](W) writer structure"]
impl crate::Writable for TxResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_RESET to value 0"]
impl crate::Resettable for TxResetSpec {}
