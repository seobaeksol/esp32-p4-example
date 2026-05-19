#[doc = "Register `L1_CACHE_ACS_FAIL_INT_CLR` reader"]
pub type R = crate::R<L1CacheAcsFailIntClrSpec>;
#[doc = "Register `L1_CACHE_ACS_FAIL_INT_CLR` writer"]
pub type W = crate::W<L1CacheAcsFailIntClrSpec>;
#[doc = "Field `L1_ICACHE0_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
pub type L1Icache0FailIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
pub type L1Icache1FailIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_FAIL_INT_CLR` reader - Reserved"]
pub type L1Icache2FailIntClrR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_FAIL_INT_CLR` reader - Reserved"]
pub type L1Icache3FailIntClrR = crate::BitReader;
#[doc = "Field `L1_DCACHE_FAIL_INT_CLR` writer - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type L1DcacheFailIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_fail_int_clr(&self) -> L1Icache2FailIntClrR {
        L1Icache2FailIntClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_fail_int_clr(&self) -> L1Icache3FailIntClrR {
        L1Icache3FailIntClrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to clear interrupt of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_clr(
        &mut self,
    ) -> L1Icache0FailIntClrW<'_, L1CacheAcsFailIntClrSpec> {
        L1Icache0FailIntClrW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to clear interrupt of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_clr(
        &mut self,
    ) -> L1Icache1FailIntClrW<'_, L1CacheAcsFailIntClrSpec> {
        L1Icache1FailIntClrW::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to clear interrupt of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_int_clr(&mut self) -> L1DcacheFailIntClrW<'_, L1CacheAcsFailIntClrSpec> {
        L1DcacheFailIntClrW::new(self, 4)
    }
}
#[doc = "L1-Cache Access Fail Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_fail_int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAcsFailIntClrSpec;
impl crate::RegisterSpec for L1CacheAcsFailIntClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_fail_int_clr::R`](R) reader structure"]
impl crate::Readable for L1CacheAcsFailIntClrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_fail_int_clr::W`](W) writer structure"]
impl crate::Writable for L1CacheAcsFailIntClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_INT_CLR to value 0"]
impl crate::Resettable for L1CacheAcsFailIntClrSpec {}
