#[doc = "Register `DMA_RX_RESET` writer"]
pub type W = crate::W<DmaRxResetSpec>;
#[doc = "Field `DMA_RX_RESET` writer - Write 1 to reset DMA RX FIFO"]
pub type DmaRxResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to reset DMA RX FIFO"]
    #[inline(always)]
    pub fn dma_rx_reset(&mut self) -> DmaRxResetW<'_, DmaRxResetSpec> {
        DmaRxResetW::new(self, 0)
    }
}
#[doc = "DMA RX FIFO Reset Signal\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rx_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaRxResetSpec;
impl crate::RegisterSpec for DmaRxResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_rx_reset::W`](W) writer structure"]
impl crate::Writable for DmaRxResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_RX_RESET to value 0"]
impl crate::Resettable for DmaRxResetSpec {}
