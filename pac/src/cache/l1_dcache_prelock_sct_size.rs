#[doc = "Register `L1_DCACHE_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<L1DcachePrelockSctSizeSpec>;
#[doc = "Register `L1_DCACHE_PRELOCK_SCT_SIZE` writer"]
pub type W = crate::W<L1DcachePrelockSctSizeSpec>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT0_ADDR_REG"]
pub type L1DcachePrelockSct0SizeR = crate::FieldReader<u16>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT0_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT0_ADDR_REG"]
pub type L1DcachePrelockSct0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT1_ADDR_REG"]
pub type L1DcachePrelockSct1SizeR = crate::FieldReader<u16>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT1_SIZE` writer - Those bits are used to configure the size of the second section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT1_ADDR_REG"]
pub type L1DcachePrelockSct1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct0_size(&self) -> L1DcachePrelockSct0SizeR {
        L1DcachePrelockSct0SizeR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Those bits are used to configure the size of the second section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct1_size(&self) -> L1DcachePrelockSct1SizeR {
        L1DcachePrelockSct1SizeR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct0_size(
        &mut self,
    ) -> L1DcachePrelockSct0SizeW<'_, L1DcachePrelockSctSizeSpec> {
        L1DcachePrelockSct0SizeW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Those bits are used to configure the size of the second section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct1_size(
        &mut self,
    ) -> L1DcachePrelockSct1SizeW<'_, L1DcachePrelockSctSizeSpec> {
        L1DcachePrelockSct1SizeW::new(self, 16)
    }
}
#[doc = "L1 data Cache prelock section size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_prelock_sct_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_prelock_sct_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcachePrelockSctSizeSpec;
impl crate::RegisterSpec for L1DcachePrelockSctSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for L1DcachePrelockSctSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_prelock_sct_size::W`](W) writer structure"]
impl crate::Writable for L1DcachePrelockSctSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_PRELOCK_SCT_SIZE to value 0x3fff_3fff"]
impl crate::Resettable for L1DcachePrelockSctSizeSpec {
    const RESET_VALUE: u32 = 0x3fff_3fff;
}
