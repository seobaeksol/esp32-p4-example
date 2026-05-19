#[doc = "Register `L1_CACHE_WRAP_AROUND_CTRL` reader"]
pub type R = crate::R<L1CacheWrapAroundCtrlSpec>;
#[doc = "Register `L1_CACHE_WRAP_AROUND_CTRL` writer"]
pub type W = crate::W<L1CacheWrapAroundCtrlSpec>;
#[doc = "Field `L1_ICACHE0_WRAP` reader - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
pub type L1Icache0WrapR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_WRAP` writer - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
pub type L1Icache0WrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_WRAP` reader - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
pub type L1Icache1WrapR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_WRAP` writer - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
pub type L1Icache1WrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_WRAP` reader - Reserved"]
pub type L1Icache2WrapR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_WRAP` reader - Reserved"]
pub type L1Icache3WrapR = crate::BitReader;
#[doc = "Field `L1_DCACHE_WRAP` reader - Set this bit as 1 to enable L1-DCache wrap around mode."]
pub type L1DcacheWrapR = crate::BitReader;
#[doc = "Field `L1_DCACHE_WRAP` writer - Set this bit as 1 to enable L1-DCache wrap around mode."]
pub type L1DcacheWrapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
    #[inline(always)]
    pub fn l1_icache0_wrap(&self) -> L1Icache0WrapR {
        L1Icache0WrapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
    #[inline(always)]
    pub fn l1_icache1_wrap(&self) -> L1Icache1WrapR {
        L1Icache1WrapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_wrap(&self) -> L1Icache2WrapR {
        L1Icache2WrapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_wrap(&self) -> L1Icache3WrapR {
        L1Icache3WrapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit as 1 to enable L1-DCache wrap around mode."]
    #[inline(always)]
    pub fn l1_dcache_wrap(&self) -> L1DcacheWrapR {
        L1DcacheWrapR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
    #[inline(always)]
    pub fn l1_icache0_wrap(&mut self) -> L1Icache0WrapW<'_, L1CacheWrapAroundCtrlSpec> {
        L1Icache0WrapW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
    #[inline(always)]
    pub fn l1_icache1_wrap(&mut self) -> L1Icache1WrapW<'_, L1CacheWrapAroundCtrlSpec> {
        L1Icache1WrapW::new(self, 1)
    }
    #[doc = "Bit 4 - Set this bit as 1 to enable L1-DCache wrap around mode."]
    #[inline(always)]
    pub fn l1_dcache_wrap(&mut self) -> L1DcacheWrapW<'_, L1CacheWrapAroundCtrlSpec> {
        L1DcacheWrapW::new(self, 4)
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_wrap_around_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_wrap_around_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheWrapAroundCtrlSpec;
impl crate::RegisterSpec for L1CacheWrapAroundCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_wrap_around_ctrl::R`](R) reader structure"]
impl crate::Readable for L1CacheWrapAroundCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_wrap_around_ctrl::W`](W) writer structure"]
impl crate::Writable for L1CacheWrapAroundCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_WRAP_AROUND_CTRL to value 0"]
impl crate::Resettable for L1CacheWrapAroundCtrlSpec {}
