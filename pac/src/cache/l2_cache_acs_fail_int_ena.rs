#[doc = "Register `L2_CACHE_ACS_FAIL_INT_ENA` reader"]
pub type R = crate::R<L2CacheAcsFailIntEnaSpec>;
#[doc = "Register `L2_CACHE_ACS_FAIL_INT_ENA` writer"]
pub type W = crate::W<L2CacheAcsFailIntEnaSpec>;
#[doc = "Field `L2_CACHE_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
pub type L2CacheFailIntEnaR = crate::BitReader;
#[doc = "Field `L2_CACHE_FAIL_INT_ENA` writer - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
pub type L2CacheFailIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_int_ena(&self) -> L2CacheFailIntEnaR {
        L2CacheFailIntEnaR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - The bit is used to enable interrupt of access fail that occurs in L2-Cache due to l1 cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_int_ena(&mut self) -> L2CacheFailIntEnaW<'_, L2CacheAcsFailIntEnaSpec> {
        L2CacheFailIntEnaW::new(self, 5)
    }
}
#[doc = "Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_acs_fail_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_acs_fail_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheAcsFailIntEnaSpec;
impl crate::RegisterSpec for L2CacheAcsFailIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_fail_int_ena::R`](R) reader structure"]
impl crate::Readable for L2CacheAcsFailIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_acs_fail_int_ena::W`](W) writer structure"]
impl crate::Writable for L2CacheAcsFailIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_INT_ENA to value 0"]
impl crate::Resettable for L2CacheAcsFailIntEnaSpec {}
