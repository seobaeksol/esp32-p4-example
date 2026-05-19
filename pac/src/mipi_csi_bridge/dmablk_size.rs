#[doc = "Register `DMABLK_SIZE` reader"]
pub type R = crate::R<DmablkSizeSpec>;
#[doc = "Register `DMABLK_SIZE` writer"]
pub type W = crate::W<DmablkSizeSpec>;
#[doc = "Field `DMABLK_SIZE` reader - the number of reg_dma_burst_len in a block"]
pub type DmablkSizeR = crate::FieldReader<u16>;
#[doc = "Field `DMABLK_SIZE` writer - the number of reg_dma_burst_len in a block"]
pub type DmablkSizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - the number of reg_dma_burst_len in a block"]
    #[inline(always)]
    pub fn dmablk_size(&self) -> DmablkSizeR {
        DmablkSizeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - the number of reg_dma_burst_len in a block"]
    #[inline(always)]
    pub fn dmablk_size(&mut self) -> DmablkSizeW<'_, DmablkSizeSpec> {
        DmablkSizeW::new(self, 0)
    }
}
#[doc = "DMA block size configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmablk_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmablk_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmablkSizeSpec;
impl crate::RegisterSpec for DmablkSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmablk_size::R`](R) reader structure"]
impl crate::Readable for DmablkSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`dmablk_size::W`](W) writer structure"]
impl crate::Writable for DmablkSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMABLK_SIZE to value 0x1fff"]
impl crate::Resettable for DmablkSizeSpec {
    const RESET_VALUE: u32 = 0x1fff;
}
