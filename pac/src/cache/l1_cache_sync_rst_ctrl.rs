#[doc = "Register `L1_CACHE_SYNC_RST_CTRL` reader"]
pub type R = crate::R<L1CacheSyncRstCtrlSpec>;
#[doc = "Register `L1_CACHE_SYNC_RST_CTRL` writer"]
pub type W = crate::W<L1CacheSyncRstCtrlSpec>;
#[doc = "Field `L1_ICACHE0_SYNC_RST` reader - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L1Icache0SyncRstR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_SYNC_RST` writer - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L1Icache0SyncRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_SYNC_RST` reader - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L1Icache1SyncRstR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_SYNC_RST` writer - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L1Icache1SyncRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_SYNC_RST` reader - Reserved"]
pub type L1Icache2SyncRstR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_SYNC_RST` reader - Reserved"]
pub type L1Icache3SyncRstR = crate::BitReader;
#[doc = "Field `L1_DCACHE_SYNC_RST` reader - set this bit to reset sync-logic inside L1-DCache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L1DcacheSyncRstR = crate::BitReader;
#[doc = "Field `L1_DCACHE_SYNC_RST` writer - set this bit to reset sync-logic inside L1-DCache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L1DcacheSyncRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l1_icache0_sync_rst(&self) -> L1Icache0SyncRstR {
        L1Icache0SyncRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l1_icache1_sync_rst(&self) -> L1Icache1SyncRstR {
        L1Icache1SyncRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_sync_rst(&self) -> L1Icache2SyncRstR {
        L1Icache2SyncRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_sync_rst(&self) -> L1Icache3SyncRstR {
        L1Icache3SyncRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to reset sync-logic inside L1-DCache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l1_dcache_sync_rst(&self) -> L1DcacheSyncRstR {
        L1DcacheSyncRstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l1_icache0_sync_rst(&mut self) -> L1Icache0SyncRstW<'_, L1CacheSyncRstCtrlSpec> {
        L1Icache0SyncRstW::new(self, 0)
    }
    #[doc = "Bit 1 - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l1_icache1_sync_rst(&mut self) -> L1Icache1SyncRstW<'_, L1CacheSyncRstCtrlSpec> {
        L1Icache1SyncRstW::new(self, 1)
    }
    #[doc = "Bit 4 - set this bit to reset sync-logic inside L1-DCache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l1_dcache_sync_rst(&mut self) -> L1DcacheSyncRstW<'_, L1CacheSyncRstCtrlSpec> {
        L1DcacheSyncRstW::new(self, 4)
    }
}
#[doc = "Cache Sync Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_sync_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_sync_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheSyncRstCtrlSpec;
impl crate::RegisterSpec for L1CacheSyncRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_sync_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for L1CacheSyncRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_sync_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for L1CacheSyncRstCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_SYNC_RST_CTRL to value 0"]
impl crate::Resettable for L1CacheSyncRstCtrlSpec {}
