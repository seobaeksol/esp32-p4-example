#[doc = "Register `L2_CACHE_SYNC_PRELOAD_INT_CLR` writer"]
pub type W = crate::W<L2CacheSyncPreloadIntClrSpec>;
#[doc = "Field `L2_CACHE_PLD_DONE_INT_CLR` writer - The bit is used to clear interrupt that occurs only when L2-Cache preload-operation is done."]
pub type L2CachePldDoneIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_PLD_ERR_INT_CLR` writer - The bit is used to clear interrupt of L2-Cache preload-operation error."]
pub type L2CachePldErrIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - The bit is used to clear interrupt that occurs only when L2-Cache preload-operation is done."]
    #[inline(always)]
    pub fn l2_cache_pld_done_int_clr(
        &mut self,
    ) -> L2CachePldDoneIntClrW<'_, L2CacheSyncPreloadIntClrSpec> {
        L2CachePldDoneIntClrW::new(self, 5)
    }
    #[doc = "Bit 12 - The bit is used to clear interrupt of L2-Cache preload-operation error."]
    #[inline(always)]
    pub fn l2_cache_pld_err_int_clr(
        &mut self,
    ) -> L2CachePldErrIntClrW<'_, L2CacheSyncPreloadIntClrSpec> {
        L2CachePldErrIntClrW::new(self, 12)
    }
}
#[doc = "Sync Preload operation Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_sync_preload_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheSyncPreloadIntClrSpec;
impl crate::RegisterSpec for L2CacheSyncPreloadIntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l2_cache_sync_preload_int_clr::W`](W) writer structure"]
impl crate::Writable for L2CacheSyncPreloadIntClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_SYNC_PRELOAD_INT_CLR to value 0"]
impl crate::Resettable for L2CacheSyncPreloadIntClrSpec {}
