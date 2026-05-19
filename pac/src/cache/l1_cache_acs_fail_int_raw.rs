#[doc = "Register `L1_CACHE_ACS_FAIL_INT_RAW` reader"]
pub type R = crate::R<L1CacheAcsFailIntRawSpec>;
#[doc = "Register `L1_CACHE_ACS_FAIL_INT_RAW` writer"]
pub type W = crate::W<L1CacheAcsFailIntRawSpec>;
#[doc = "Field `L1_ICACHE0_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
pub type L1Icache0FailIntRawR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
pub type L1Icache0FailIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
pub type L1Icache1FailIntRawR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
pub type L1Icache1FailIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
pub type L1Icache2FailIntRawR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
pub type L1Icache2FailIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE3_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
pub type L1Icache3FailIntRawR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
pub type L1Icache3FailIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_FAIL_INT_RAW` reader - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
pub type L1DcacheFailIntRawR = crate::BitReader;
#[doc = "Field `L1_DCACHE_FAIL_INT_RAW` writer - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
pub type L1DcacheFailIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_raw(&self) -> L1Icache0FailIntRawR {
        L1Icache0FailIntRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_raw(&self) -> L1Icache1FailIntRawR {
        L1Icache1FailIntRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
    #[inline(always)]
    pub fn l1_icache2_fail_int_raw(&self) -> L1Icache2FailIntRawR {
        L1Icache2FailIntRawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
    #[inline(always)]
    pub fn l1_icache3_fail_int_raw(&self) -> L1Icache3FailIntRawR {
        L1Icache3FailIntRawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_int_raw(&self) -> L1DcacheFailIntRawR {
        L1DcacheFailIntRawR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw bit of the interrupt of access fail that occurs in L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_raw(
        &mut self,
    ) -> L1Icache0FailIntRawW<'_, L1CacheAcsFailIntRawSpec> {
        L1Icache0FailIntRawW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw bit of the interrupt of access fail that occurs in L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_raw(
        &mut self,
    ) -> L1Icache1FailIntRawW<'_, L1CacheAcsFailIntRawSpec> {
        L1Icache1FailIntRawW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw bit of the interrupt of access fail that occurs in L1-ICache2."]
    #[inline(always)]
    pub fn l1_icache2_fail_int_raw(
        &mut self,
    ) -> L1Icache2FailIntRawW<'_, L1CacheAcsFailIntRawSpec> {
        L1Icache2FailIntRawW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit of the interrupt of access fail that occurs in L1-ICache3."]
    #[inline(always)]
    pub fn l1_icache3_fail_int_raw(
        &mut self,
    ) -> L1Icache3FailIntRawW<'_, L1CacheAcsFailIntRawSpec> {
        L1Icache3FailIntRawW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit of the interrupt of access fail that occurs in L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_int_raw(&mut self) -> L1DcacheFailIntRawW<'_, L1CacheAcsFailIntRawSpec> {
        L1DcacheFailIntRawW::new(self, 4)
    }
}
#[doc = "Cache Access Fail Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_fail_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAcsFailIntRawSpec;
impl crate::RegisterSpec for L1CacheAcsFailIntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_fail_int_raw::R`](R) reader structure"]
impl crate::Readable for L1CacheAcsFailIntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_fail_int_raw::W`](W) writer structure"]
impl crate::Writable for L1CacheAcsFailIntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_INT_RAW to value 0"]
impl crate::Resettable for L1CacheAcsFailIntRawSpec {}
