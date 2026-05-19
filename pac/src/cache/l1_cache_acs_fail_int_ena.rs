#[doc = "Register `L1_CACHE_ACS_FAIL_INT_ENA` reader"]
pub type R = crate::R<L1CacheAcsFailIntEnaSpec>;
#[doc = "Register `L1_CACHE_ACS_FAIL_INT_ENA` writer"]
pub type W = crate::W<L1CacheAcsFailIntEnaSpec>;
#[doc = "Field `L1_ICACHE0_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
pub type L1Icache0FailIntEnaR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_FAIL_INT_ENA` writer - The bit is used to enable interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
pub type L1Icache0FailIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
pub type L1Icache1FailIntEnaR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_FAIL_INT_ENA` writer - The bit is used to enable interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
pub type L1Icache1FailIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_FAIL_INT_ENA` reader - Reserved"]
pub type L1Icache2FailIntEnaR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_FAIL_INT_ENA` reader - Reserved"]
pub type L1Icache3FailIntEnaR = crate::BitReader;
#[doc = "Field `L1_DCACHE_FAIL_INT_ENA` reader - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type L1DcacheFailIntEnaR = crate::BitReader;
#[doc = "Field `L1_DCACHE_FAIL_INT_ENA` writer - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type L1DcacheFailIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_ena(&self) -> L1Icache0FailIntEnaR {
        L1Icache0FailIntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_ena(&self) -> L1Icache1FailIntEnaR {
        L1Icache1FailIntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_fail_int_ena(&self) -> L1Icache2FailIntEnaR {
        L1Icache2FailIntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_fail_int_ena(&self) -> L1Icache3FailIntEnaR {
        L1Icache3FailIntEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_int_ena(&self) -> L1DcacheFailIntEnaR {
        L1DcacheFailIntEnaR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_ena(
        &mut self,
    ) -> L1Icache0FailIntEnaW<'_, L1CacheAcsFailIntEnaSpec> {
        L1Icache0FailIntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_ena(
        &mut self,
    ) -> L1Icache1FailIntEnaW<'_, L1CacheAcsFailIntEnaSpec> {
        L1Icache1FailIntEnaW::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_int_ena(&mut self) -> L1DcacheFailIntEnaW<'_, L1CacheAcsFailIntEnaSpec> {
        L1DcacheFailIntEnaW::new(self, 4)
    }
}
#[doc = "Cache Access Fail Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_fail_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAcsFailIntEnaSpec;
impl crate::RegisterSpec for L1CacheAcsFailIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_fail_int_ena::R`](R) reader structure"]
impl crate::Readable for L1CacheAcsFailIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_fail_int_ena::W`](W) writer structure"]
impl crate::Writable for L1CacheAcsFailIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_INT_ENA to value 0"]
impl crate::Resettable for L1CacheAcsFailIntEnaSpec {}
