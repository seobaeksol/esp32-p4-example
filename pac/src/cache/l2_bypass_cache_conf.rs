#[doc = "Register `L2_BYPASS_CACHE_CONF` reader"]
pub type R = crate::R<L2BypassCacheConfSpec>;
#[doc = "Register `L2_BYPASS_CACHE_CONF` writer"]
pub type W = crate::W<L2BypassCacheConfSpec>;
#[doc = "Field `BYPASS_L2_CACHE_EN` reader - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
pub type BypassL2CacheEnR = crate::BitReader;
#[doc = "Field `BYPASS_L2_CACHE_EN` writer - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
pub type BypassL2CacheEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l2_cache_en(&self) -> BypassL2CacheEnR {
        BypassL2CacheEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l2_cache_en(&mut self) -> BypassL2CacheEnW<'_, L2BypassCacheConfSpec> {
        BypassL2CacheEnW::new(self, 5)
    }
}
#[doc = "Bypass Cache configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_bypass_cache_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_bypass_cache_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2BypassCacheConfSpec;
impl crate::RegisterSpec for L2BypassCacheConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_bypass_cache_conf::R`](R) reader structure"]
impl crate::Readable for L2BypassCacheConfSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_bypass_cache_conf::W`](W) writer structure"]
impl crate::Writable for L2BypassCacheConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_BYPASS_CACHE_CONF to value 0"]
impl crate::Resettable for L2BypassCacheConfSpec {}
