#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_ST` reader"]
pub type R = crate::R<L2CacheSyncPreloadIntStSpec>;
#[doc = "Field `L2_CACHE_PLD_DONE_INT_ST` reader - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done."]
pub type L2CachePldDoneIntStR = crate::BitReader;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_ST` reader - The bit indicates the status of the interrupt of L2-Cache preload-operation error."]
pub type L2CachePldErrIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The bit indicates the status of the interrupt that occurs only when L2-Cache preload-operation is done."]
    #[inline(always)]
    pub fn l2_cache_pld_done_int_st(&self) -> L2CachePldDoneIntStR {
        L2CachePldDoneIntStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit indicates the status of the interrupt of L2-Cache preload-operation error."]
    #[inline(always)]
    pub fn l2_cache_pld_err_int_st(&self) -> L2CachePldErrIntStR {
        L2CachePldErrIntStR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "L1-Cache Access Fail Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_sync_preload_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheSyncPreloadIntStSpec;
impl crate::RegisterSpec for L2CacheSyncPreloadIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_sync_preload_int_st::R`](R) reader structure"]
impl crate::Readable for L2CacheSyncPreloadIntStSpec {}
#[doc = "`reset()` method sets L2_CACHE_SYNC_PRELOAD_INT_ST to value 0"]
impl crate::Resettable for L2CacheSyncPreloadIntStSpec {}
