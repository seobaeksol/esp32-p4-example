#[doc = "Register `MMU_ITEM_INDEX` reader"]
pub type R = crate::R<MmuItemIndexSpec>;
#[doc = "Register `MMU_ITEM_INDEX` writer"]
pub type W = crate::W<MmuItemIndexSpec>;
#[doc = "Field `SPI_MMU_ITEM_INDEX` reader - MSPI-MMU item index"]
pub type SpiMmuItemIndexR = crate::FieldReader<u32>;
#[doc = "Field `SPI_MMU_ITEM_INDEX` writer - MSPI-MMU item index"]
pub type SpiMmuItemIndexW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MSPI-MMU item index"]
    #[inline(always)]
    pub fn spi_mmu_item_index(&self) -> SpiMmuItemIndexR {
        SpiMmuItemIndexR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MSPI-MMU item index"]
    #[inline(always)]
    pub fn spi_mmu_item_index(&mut self) -> SpiMmuItemIndexW<'_, MmuItemIndexSpec> {
        SpiMmuItemIndexW::new(self, 0)
    }
}
#[doc = "MSPI-MMU item index register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmuItemIndexSpec;
impl crate::RegisterSpec for MmuItemIndexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_item_index::R`](R) reader structure"]
impl crate::Readable for MmuItemIndexSpec {}
#[doc = "`write(|w| ..)` method takes [`mmu_item_index::W`](W) writer structure"]
impl crate::Writable for MmuItemIndexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMU_ITEM_INDEX to value 0"]
impl crate::Resettable for MmuItemIndexSpec {}
