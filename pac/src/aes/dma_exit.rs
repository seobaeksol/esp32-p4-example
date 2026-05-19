#[doc = "Register `DMA_EXIT` writer"]
pub type W = crate::W<DmaExitSpec>;
#[doc = "Field `DMA_EXIT` writer - Configures whether or not to exit AES operation. \\\\ 0: No effect\\\\ 1: Exit\\\\ Only valid for DMA-AES operation."]
pub type DmaExitW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Configures whether or not to exit AES operation. \\\\ 0: No effect\\\\ 1: Exit\\\\ Only valid for DMA-AES operation."]
    #[inline(always)]
    pub fn dma_exit(&mut self) -> DmaExitW<'_, DmaExitSpec> {
        DmaExitW::new(self, 0)
    }
}
#[doc = "Operation exit controlling register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_exit::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaExitSpec;
impl crate::RegisterSpec for DmaExitSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_exit::W`](W) writer structure"]
impl crate::Writable for DmaExitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_EXIT to value 0"]
impl crate::Resettable for DmaExitSpec {}
