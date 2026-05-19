#[doc = "Register `L2_CACHE_PRELOAD_SIZE` reader"]
pub type R = crate::R<L2CachePreloadSizeSpec>;
#[doc = "Register `L2_CACHE_PRELOAD_SIZE` writer"]
pub type W = crate::W<L2CachePreloadSizeSpec>;
#[doc = "Field `L2_CACHE_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOAD_ADDR_REG"]
pub type L2CachePreloadSizeR = crate::FieldReader<u16>;
#[doc = "Field `L2_CACHE_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOAD_ADDR_REG"]
pub type L2CachePreloadSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l2_cache_preload_size(&self) -> L2CachePreloadSizeR {
        L2CachePreloadSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn l2_cache_preload_size(&mut self) -> L2CachePreloadSizeW<'_, L2CachePreloadSizeSpec> {
        L2CachePreloadSizeW::new(self, 0)
    }
}
#[doc = "L2 Cache preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_preload_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_preload_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CachePreloadSizeSpec;
impl crate::RegisterSpec for L2CachePreloadSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_preload_size::R`](R) reader structure"]
impl crate::Readable for L2CachePreloadSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_preload_size::W`](W) writer structure"]
impl crate::Writable for L2CachePreloadSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L2CachePreloadSizeSpec {}
