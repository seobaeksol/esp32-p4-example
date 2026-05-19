#[doc = "Register `DMA_ENABLE` reader"]
pub type R = crate::R<DmaEnableSpec>;
#[doc = "Register `DMA_ENABLE` writer"]
pub type W = crate::W<DmaEnableSpec>;
#[doc = "Field `DMA_ENABLE` reader - Configures the working mode of the AES accelerator. \\\\ 0: Typical AES\\\\ 1: DMA-AES\\\\"]
pub type DmaEnableR = crate::BitReader;
#[doc = "Field `DMA_ENABLE` writer - Configures the working mode of the AES accelerator. \\\\ 0: Typical AES\\\\ 1: DMA-AES\\\\"]
pub type DmaEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the working mode of the AES accelerator. \\\\ 0: Typical AES\\\\ 1: DMA-AES\\\\"]
    #[inline(always)]
    pub fn dma_enable(&self) -> DmaEnableR {
        DmaEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the working mode of the AES accelerator. \\\\ 0: Typical AES\\\\ 1: DMA-AES\\\\"]
    #[inline(always)]
    pub fn dma_enable(&mut self) -> DmaEnableW<'_, DmaEnableSpec> {
        DmaEnableW::new(self, 0)
    }
}
#[doc = "Selects the working mode of the AES accelerator\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaEnableSpec;
impl crate::RegisterSpec for DmaEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_enable::R`](R) reader structure"]
impl crate::Readable for DmaEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_enable::W`](W) writer structure"]
impl crate::Writable for DmaEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_ENABLE to value 0"]
impl crate::Resettable for DmaEnableSpec {}
