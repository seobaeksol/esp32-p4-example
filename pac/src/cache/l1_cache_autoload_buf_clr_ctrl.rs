#[doc = "Register `L1_CACHE_AUTOLOAD_BUF_CLR_CTRL` reader"]
pub type R = crate::R<L1CacheAutoloadBufClrCtrlSpec>;
#[doc = "Register `L1_CACHE_AUTOLOAD_BUF_CLR_CTRL` writer"]
pub type W = crate::W<L1CacheAutoloadBufClrCtrlSpec>;
#[doc = "Field `L1_ICACHE0_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L1-ICache0. If this bit is active, autoload will not work in L1-ICache0. This bit should not be active when autoload works in L1-ICache0."]
pub type L1Icache0AldBufClrR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_ALD_BUF_CLR` writer - set this bit to clear autoload-buffer inside L1-ICache0. If this bit is active, autoload will not work in L1-ICache0. This bit should not be active when autoload works in L1-ICache0."]
pub type L1Icache0AldBufClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L1-ICache1. If this bit is active, autoload will not work in L1-ICache1. This bit should not be active when autoload works in L1-ICache1."]
pub type L1Icache1AldBufClrR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_ALD_BUF_CLR` writer - set this bit to clear autoload-buffer inside L1-ICache1. If this bit is active, autoload will not work in L1-ICache1. This bit should not be active when autoload works in L1-ICache1."]
pub type L1Icache1AldBufClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_ALD_BUF_CLR` reader - Reserved"]
pub type L1Icache2AldBufClrR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_ALD_BUF_CLR` reader - Reserved"]
pub type L1Icache3AldBufClrR = crate::BitReader;
#[doc = "Field `L1_DCACHE_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L1-DCache. If this bit is active, autoload will not work in L1-DCache. This bit should not be active when autoload works in L1-DCache."]
pub type L1DcacheAldBufClrR = crate::BitReader;
#[doc = "Field `L1_DCACHE_ALD_BUF_CLR` writer - set this bit to clear autoload-buffer inside L1-DCache. If this bit is active, autoload will not work in L1-DCache. This bit should not be active when autoload works in L1-DCache."]
pub type L1DcacheAldBufClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set this bit to clear autoload-buffer inside L1-ICache0. If this bit is active, autoload will not work in L1-ICache0. This bit should not be active when autoload works in L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_ald_buf_clr(&self) -> L1Icache0AldBufClrR {
        L1Icache0AldBufClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit to clear autoload-buffer inside L1-ICache1. If this bit is active, autoload will not work in L1-ICache1. This bit should not be active when autoload works in L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_ald_buf_clr(&self) -> L1Icache1AldBufClrR {
        L1Icache1AldBufClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_ald_buf_clr(&self) -> L1Icache2AldBufClrR {
        L1Icache2AldBufClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_ald_buf_clr(&self) -> L1Icache3AldBufClrR {
        L1Icache3AldBufClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to clear autoload-buffer inside L1-DCache. If this bit is active, autoload will not work in L1-DCache. This bit should not be active when autoload works in L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_ald_buf_clr(&self) -> L1DcacheAldBufClrR {
        L1DcacheAldBufClrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set this bit to clear autoload-buffer inside L1-ICache0. If this bit is active, autoload will not work in L1-ICache0. This bit should not be active when autoload works in L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_ald_buf_clr(
        &mut self,
    ) -> L1Icache0AldBufClrW<'_, L1CacheAutoloadBufClrCtrlSpec> {
        L1Icache0AldBufClrW::new(self, 0)
    }
    #[doc = "Bit 1 - set this bit to clear autoload-buffer inside L1-ICache1. If this bit is active, autoload will not work in L1-ICache1. This bit should not be active when autoload works in L1-ICache1."]
    #[inline(always)]
    pub fn l1_icache1_ald_buf_clr(
        &mut self,
    ) -> L1Icache1AldBufClrW<'_, L1CacheAutoloadBufClrCtrlSpec> {
        L1Icache1AldBufClrW::new(self, 1)
    }
    #[doc = "Bit 4 - set this bit to clear autoload-buffer inside L1-DCache. If this bit is active, autoload will not work in L1-DCache. This bit should not be active when autoload works in L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_ald_buf_clr(
        &mut self,
    ) -> L1DcacheAldBufClrW<'_, L1CacheAutoloadBufClrCtrlSpec> {
        L1DcacheAldBufClrW::new(self, 4)
    }
}
#[doc = "Cache Autoload buffer clear control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_autoload_buf_clr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_autoload_buf_clr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAutoloadBufClrCtrlSpec;
impl crate::RegisterSpec for L1CacheAutoloadBufClrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_autoload_buf_clr_ctrl::R`](R) reader structure"]
impl crate::Readable for L1CacheAutoloadBufClrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_autoload_buf_clr_ctrl::W`](W) writer structure"]
impl crate::Writable for L1CacheAutoloadBufClrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_AUTOLOAD_BUF_CLR_CTRL to value 0"]
impl crate::Resettable for L1CacheAutoloadBufClrCtrlSpec {}
