#[doc = "Register `DMA_BLOCK_NUM` reader"]
pub type R = crate::R<DmaBlockNumSpec>;
#[doc = "Register `DMA_BLOCK_NUM` writer"]
pub type W = crate::W<DmaBlockNumSpec>;
#[doc = "Field `DMA_BLOCK_NUM` reader - Configures the DMA-SHA block number."]
pub type DmaBlockNumR = crate::FieldReader<u16>;
#[doc = "Field `DMA_BLOCK_NUM` writer - Configures the DMA-SHA block number."]
pub type DmaBlockNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the DMA-SHA block number."]
    #[inline(always)]
    pub fn dma_block_num(&self) -> DmaBlockNumR {
        DmaBlockNumR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the DMA-SHA block number."]
    #[inline(always)]
    pub fn dma_block_num(&mut self) -> DmaBlockNumW<'_, DmaBlockNumSpec> {
        DmaBlockNumW::new(self, 0)
    }
}
#[doc = "Block number register (only effective for DMA-SHA)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_block_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_block_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaBlockNumSpec;
impl crate::RegisterSpec for DmaBlockNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_block_num::R`](R) reader structure"]
impl crate::Readable for DmaBlockNumSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_block_num::W`](W) writer structure"]
impl crate::Writable for DmaBlockNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_BLOCK_NUM to value 0"]
impl crate::Resettable for DmaBlockNumSpec {}
