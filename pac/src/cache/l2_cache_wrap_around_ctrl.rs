#[doc = "Register `L2_CACHE_WRAP_AROUND_CTRL` reader"]
pub type R = crate::R<L2CacheWrapAroundCtrlSpec>;
#[doc = "Register `L2_CACHE_WRAP_AROUND_CTRL` writer"]
pub type W = crate::W<L2CacheWrapAroundCtrlSpec>;
#[doc = "Field `L2_CACHE_WRAP` reader - Set this bit as 1 to enable L2-Cache wrap around mode."]
pub type L2CacheWrapR = crate::BitReader;
#[doc = "Field `L2_CACHE_WRAP` writer - Set this bit as 1 to enable L2-Cache wrap around mode."]
pub type L2CacheWrapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Set this bit as 1 to enable L2-Cache wrap around mode."]
    #[inline(always)]
    pub fn l2_cache_wrap(&self) -> L2CacheWrapR {
        L2CacheWrapR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Set this bit as 1 to enable L2-Cache wrap around mode."]
    #[inline(always)]
    pub fn l2_cache_wrap(&mut self) -> L2CacheWrapW<'_, L2CacheWrapAroundCtrlSpec> {
        L2CacheWrapW::new(self, 5)
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_wrap_around_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_wrap_around_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheWrapAroundCtrlSpec;
impl crate::RegisterSpec for L2CacheWrapAroundCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_wrap_around_ctrl::R`](R) reader structure"]
impl crate::Readable for L2CacheWrapAroundCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_wrap_around_ctrl::W`](W) writer structure"]
impl crate::Writable for L2CacheWrapAroundCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_WRAP_AROUND_CTRL to value 0"]
impl crate::Resettable for L2CacheWrapAroundCtrlSpec {}
