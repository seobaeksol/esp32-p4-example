#[doc = "Register `L1_CACHE_PRELOAD_RST_CTRL` reader"]
pub type R = crate::R<L1CachePreloadRstCtrlSpec>;
#[doc = "Register `L1_CACHE_PRELOAD_RST_CTRL` writer"]
pub type W = crate::W<L1CachePreloadRstCtrlSpec>;
#[doc = "Field `L1_ICACHE0_PLD_RST` reader - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1Icache0PldRstR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_PLD_RST` writer - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1Icache0PldRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_PLD_RST` reader - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1Icache1PldRstR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_PLD_RST` writer - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1Icache1PldRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_PLD_RST` reader - Reserved"]
pub type L1Icache2PldRstR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_PLD_RST` reader - Reserved"]
pub type L1Icache3PldRstR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PLD_RST` reader - set this bit to reset preload-logic inside L1-DCache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1DcachePldRstR = crate::BitReader;
#[doc = "Field `L1_DCACHE_PLD_RST` writer - set this bit to reset preload-logic inside L1-DCache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type L1DcachePldRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_icache0_pld_rst(&self) -> L1Icache0PldRstR {
        L1Icache0PldRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_icache1_pld_rst(&self) -> L1Icache1PldRstR {
        L1Icache1PldRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_pld_rst(&self) -> L1Icache2PldRstR {
        L1Icache2PldRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_pld_rst(&self) -> L1Icache3PldRstR {
        L1Icache3PldRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to reset preload-logic inside L1-DCache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_dcache_pld_rst(&self) -> L1DcachePldRstR {
        L1DcachePldRstR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_icache0_pld_rst(&mut self) -> L1Icache0PldRstW<'_, L1CachePreloadRstCtrlSpec> {
        L1Icache0PldRstW::new(self, 0)
    }
    #[doc = "Bit 1 - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_icache1_pld_rst(&mut self) -> L1Icache1PldRstW<'_, L1CachePreloadRstCtrlSpec> {
        L1Icache1PldRstW::new(self, 1)
    }
    #[doc = "Bit 4 - set this bit to reset preload-logic inside L1-DCache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn l1_dcache_pld_rst(&mut self) -> L1DcachePldRstW<'_, L1CachePreloadRstCtrlSpec> {
        L1DcachePldRstW::new(self, 4)
    }
}
#[doc = "Cache Preload Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_preload_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_preload_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CachePreloadRstCtrlSpec;
impl crate::RegisterSpec for L1CachePreloadRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_preload_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for L1CachePreloadRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_preload_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for L1CachePreloadRstCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_PRELOAD_RST_CTRL to value 0"]
impl crate::Resettable for L1CachePreloadRstCtrlSpec {}
