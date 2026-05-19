#[doc = "Register `L1_DCACHE_PRELOAD_SIZE` reader"]
pub type R = crate::R<L1DcachePreloadSizeSpec>;
#[doc = "Register `L1_DCACHE_PRELOAD_SIZE` writer"]
pub type W = crate::W<L1DcachePreloadSizeSpec>;
#[doc = "Field `L1_DCACHE_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOAD_ADDR_REG"]
pub type L1DcachePreloadSizeR = crate::FieldReader<u16>;
#[doc = "Field `L1_DCACHE_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOAD_ADDR_REG"]
pub type L1DcachePreloadSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l1_dcache_preload_size(&self) -> L1DcachePreloadSizeR {
        L1DcachePreloadSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-DCache, which should be used together with L1_DCACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l1_dcache_preload_size(&mut self) -> L1DcachePreloadSizeW<'_, L1DcachePreloadSizeSpec> {
        L1DcachePreloadSizeW::new(self, 0)
    }
}
#[doc = "L1 data Cache preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_preload_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_dcache_preload_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcachePreloadSizeSpec;
impl crate::RegisterSpec for L1DcachePreloadSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_preload_size::R`](R) reader structure"]
impl crate::Readable for L1DcachePreloadSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_preload_size::W`](W) writer structure"]
impl crate::Writable for L1DcachePreloadSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_DCACHE_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L1DcachePreloadSizeSpec {}
