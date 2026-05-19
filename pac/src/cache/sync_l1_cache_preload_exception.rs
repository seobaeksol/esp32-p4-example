#[doc = "Register `SYNC_L1_CACHE_PRELOAD_EXCEPTION` reader"]
pub type R = crate::R<SyncL1CachePreloadExceptionSpec>;
#[doc = "Field `L1_ICACHE0_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L1-ICache0."]
pub type L1Icache0PldErrCodeR = crate::FieldReader;
#[doc = "Field `L1_ICACHE1_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L1-ICache1."]
pub type L1Icache1PldErrCodeR = crate::FieldReader;
#[doc = "Field `L1_ICACHE2_PLD_ERR_CODE` reader - Reserved"]
pub type L1Icache2PldErrCodeR = crate::FieldReader;
#[doc = "Field `L1_ICACHE3_PLD_ERR_CODE` reader - Reserved"]
pub type L1Icache3PldErrCodeR = crate::FieldReader;
#[doc = "Field `L1_DCACHE_PLD_ERR_CODE` reader - The value 2 is Only available which means preload size is error in L1-DCache."]
pub type L1DcachePldErrCodeR = crate::FieldReader;
#[doc = "Field `SYNC_ERR_CODE` reader - The values 0-2 are available which means sync map, command conflict and size are error in Cache System."]
pub type SyncErrCodeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - The value 2 is Only available which means preload size is error in L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_pld_err_code(&self) -> L1Icache0PldErrCodeR {
        L1Icache0PldErrCodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - The value 2 is Only available which means preload size is error in L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_pld_err_code(&self) -> L1Icache1PldErrCodeR {
        L1Icache1PldErrCodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_err_code(&self) -> L1Icache2PldErrCodeR {
        L1Icache2PldErrCodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_err_code(&self) -> L1Icache3PldErrCodeR {
        L1Icache3PldErrCodeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - The value 2 is Only available which means preload size is error in L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_pld_err_code(&self) -> L1DcachePldErrCodeR {
        L1DcachePldErrCodeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - The values 0-2 are available which means sync map, command conflict and size are error in Cache System."]
    #[inline(always)]
    pub fn sync_err_code(&self) -> SyncErrCodeR {
        SyncErrCodeR::new(((self.bits >> 12) & 3) as u8)
    }
}
#[doc = "Cache Sync/Preload Operation exception register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_l1_cache_preload_exception::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncL1CachePreloadExceptionSpec;
impl crate::RegisterSpec for SyncL1CachePreloadExceptionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_l1_cache_preload_exception::R`](R) reader structure"]
impl crate::Readable for SyncL1CachePreloadExceptionSpec {}
#[doc = "`reset()` method sets SYNC_L1_CACHE_PRELOAD_EXCEPTION to value 0"]
impl crate::Resettable for SyncL1CachePreloadExceptionSpec {}
