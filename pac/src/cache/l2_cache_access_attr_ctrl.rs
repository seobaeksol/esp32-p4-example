#[doc = "Register `L2_CACHE_ACCESS_ATTR_CTRL` reader"]
pub type R = crate::R<L2CacheAccessAttrCtrlSpec>;
#[doc = "Register `L2_CACHE_ACCESS_ATTR_CTRL` writer"]
pub type W = crate::W<L2CacheAccessAttrCtrlSpec>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_CC` reader - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
pub type L2CacheAccessForceCcR = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_CC` writer - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
pub type L2CacheAccessForceCcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WB` reader - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
pub type L2CacheAccessForceWbR = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WB` writer - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
pub type L2CacheAccessForceWbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WMA` reader - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
pub type L2CacheAccessForceWmaR = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WMA` writer - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
pub type L2CacheAccessForceWmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_RMA` reader - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
pub type L2CacheAccessForceRmaR = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_RMA` writer - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
pub type L2CacheAccessForceRmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
    #[inline(always)]
    pub fn l2_cache_access_force_cc(&self) -> L2CacheAccessForceCcR {
        L2CacheAccessForceCcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
    #[inline(always)]
    pub fn l2_cache_access_force_wb(&self) -> L2CacheAccessForceWbR {
        L2CacheAccessForceWbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_wma(&self) -> L2CacheAccessForceWmaR {
        L2CacheAccessForceWmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_rma(&self) -> L2CacheAccessForceRmaR {
        L2CacheAccessForceRmaR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
    #[inline(always)]
    pub fn l2_cache_access_force_cc(
        &mut self,
    ) -> L2CacheAccessForceCcW<'_, L2CacheAccessAttrCtrlSpec> {
        L2CacheAccessForceCcW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
    #[inline(always)]
    pub fn l2_cache_access_force_wb(
        &mut self,
    ) -> L2CacheAccessForceWbW<'_, L2CacheAccessAttrCtrlSpec> {
        L2CacheAccessForceWbW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_wma(
        &mut self,
    ) -> L2CacheAccessForceWmaW<'_, L2CacheAccessAttrCtrlSpec> {
        L2CacheAccessForceWmaW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_rma(
        &mut self,
    ) -> L2CacheAccessForceRmaW<'_, L2CacheAccessAttrCtrlSpec> {
        L2CacheAccessForceRmaW::new(self, 3)
    }
}
#[doc = "L2 cache access attribute control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_access_attr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_access_attr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheAccessAttrCtrlSpec;
impl crate::RegisterSpec for L2CacheAccessAttrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_access_attr_ctrl::R`](R) reader structure"]
impl crate::Readable for L2CacheAccessAttrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_access_attr_ctrl::W`](W) writer structure"]
impl crate::Writable for L2CacheAccessAttrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_ACCESS_ATTR_CTRL to value 0x0f"]
impl crate::Resettable for L2CacheAccessAttrCtrlSpec {
    const RESET_VALUE: u32 = 0x0f;
}
