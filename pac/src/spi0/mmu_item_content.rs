#[doc = "Register `MMU_ITEM_CONTENT` reader"]
pub type R = crate::R<MmuItemContentSpec>;
#[doc = "Register `MMU_ITEM_CONTENT` writer"]
pub type W = crate::W<MmuItemContentSpec>;
#[doc = "Field `SPI_MMU_ITEM_CONTENT` reader - MSPI-MMU item content"]
pub type SpiMmuItemContentR = crate::FieldReader<u32>;
#[doc = "Field `SPI_MMU_ITEM_CONTENT` writer - MSPI-MMU item content"]
pub type SpiMmuItemContentW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MSPI-MMU item content"]
    #[inline(always)]
    pub fn spi_mmu_item_content(&self) -> SpiMmuItemContentR {
        SpiMmuItemContentR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MSPI-MMU item content"]
    #[inline(always)]
    pub fn spi_mmu_item_content(&mut self) -> SpiMmuItemContentW<'_, MmuItemContentSpec> {
        SpiMmuItemContentW::new(self, 0)
    }
}
#[doc = "MSPI-MMU item content register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_content::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_content::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuItemContentSpec;
impl crate::RegisterSpec for MmuItemContentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_item_content::R`](R) reader structure"]
impl crate::Readable for MmuItemContentSpec {}
#[doc = "`write(|w| ..)` method takes [`mmu_item_content::W`](W) writer structure"]
impl crate::Writable for MmuItemContentSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMU_ITEM_CONTENT to value 0x037c"]
impl crate::Resettable for MmuItemContentSpec {
    const RESET_VALUE: u32 = 0x037c;
}
