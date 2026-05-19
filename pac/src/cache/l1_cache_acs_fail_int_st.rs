#[doc = "Register `L1_CACHE_ACS_FAIL_INT_ST` reader"]
pub type R = crate::R<L1CacheAcsFailIntStSpec>;
#[doc = "Field `L1_ICACHE0_FAIL_INT_ST` reader - The bit indicates the interrupt status of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache."]
pub type L1Icache0FailIntStR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_FAIL_INT_ST` reader - The bit indicates the interrupt status of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache."]
pub type L1Icache1FailIntStR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_FAIL_INT_ST` reader - Reserved"]
pub type L1Icache2FailIntStR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_FAIL_INT_ST` reader - Reserved"]
pub type L1Icache3FailIntStR = crate::BitReader;
#[doc = "Field `L1_DCACHE_FAIL_INT_ST` reader - The bit indicates the interrupt status of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
pub type L1DcacheFailIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit indicates the interrupt status of access fail that occurs in L1-ICache0 due to cpu accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache0_fail_int_st(&self) -> L1Icache0FailIntStR {
        L1Icache0FailIntStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit indicates the interrupt status of access fail that occurs in L1-ICache1 due to cpu accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache1_fail_int_st(&self) -> L1Icache1FailIntStR {
        L1Icache1FailIntStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_fail_int_st(&self) -> L1Icache2FailIntStR {
        L1Icache2FailIntStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_fail_int_st(&self) -> L1Icache3FailIntStR {
        L1Icache3FailIntStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit indicates the interrupt status of access fail that occurs in L1-DCache due to cpu accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_int_st(&self) -> L1DcacheFailIntStR {
        L1DcacheFailIntStR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_fail_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAcsFailIntStSpec;
impl crate::RegisterSpec for L1CacheAcsFailIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_fail_int_st::R`](R) reader structure"]
impl crate::Readable for L1CacheAcsFailIntStSpec {}
#[doc = "`reset()` method sets L1_CACHE_ACS_FAIL_INT_ST to value 0"]
impl crate::Resettable for L1CacheAcsFailIntStSpec {}
