#[doc = "Register `L1_CACHE_TAG_MEM_POWER_CTRL` reader"]
pub type R = crate::R<L1CacheTagMemPowerCtrlSpec>;
#[doc = "Register `L1_CACHE_TAG_MEM_POWER_CTRL` writer"]
pub type W = crate::W<L1CacheTagMemPowerCtrlSpec>;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-ICache0 tag memory. 1: close gating, 0: open clock gating."]
pub type L1Icache0TagMemForceOnR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_ON` writer - The bit is used to close clock gating of L1-ICache0 tag memory. 1: close gating, 0: open clock gating."]
pub type L1Icache0TagMemForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_PD` reader - The bit is used to power L1-ICache0 tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1Icache0TagMemForcePdR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_PD` writer - The bit is used to power L1-ICache0 tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1Icache0TagMemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_PU` reader - The bit is used to power L1-ICache0 tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1Icache0TagMemForcePuR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_TAG_MEM_FORCE_PU` writer - The bit is used to power L1-ICache0 tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1Icache0TagMemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-ICache1 tag memory. 1: close gating, 0: open clock gating."]
pub type L1Icache1TagMemForceOnR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_ON` writer - The bit is used to close clock gating of L1-ICache1 tag memory. 1: close gating, 0: open clock gating."]
pub type L1Icache1TagMemForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_PD` reader - The bit is used to power L1-ICache1 tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1Icache1TagMemForcePdR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_PD` writer - The bit is used to power L1-ICache1 tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1Icache1TagMemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_PU` reader - The bit is used to power L1-ICache1 tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1Icache1TagMemForcePuR = crate::BitReader;
#[doc = "Field `L1_ICACHE1_TAG_MEM_FORCE_PU` writer - The bit is used to power L1-ICache1 tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1Icache1TagMemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_TAG_MEM_FORCE_ON` reader - Reserved"]
pub type L1Icache2TagMemForceOnR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_TAG_MEM_FORCE_PD` reader - Reserved"]
pub type L1Icache2TagMemForcePdR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_TAG_MEM_FORCE_PU` reader - Reserved"]
pub type L1Icache2TagMemForcePuR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_TAG_MEM_FORCE_ON` reader - Reserved"]
pub type L1Icache3TagMemForceOnR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_TAG_MEM_FORCE_PD` reader - Reserved"]
pub type L1Icache3TagMemForcePdR = crate::BitReader;
#[doc = "Field `L1_ICACHE3_TAG_MEM_FORCE_PU` reader - Reserved"]
pub type L1Icache3TagMemForcePuR = crate::BitReader;
#[doc = "Field `L1_DCACHE_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of L1-DCache tag memory. 1: close gating, 0: open clock gating."]
pub type L1DcacheTagMemForceOnR = crate::BitReader;
#[doc = "Field `L1_DCACHE_TAG_MEM_FORCE_ON` writer - The bit is used to close clock gating of L1-DCache tag memory. 1: close gating, 0: open clock gating."]
pub type L1DcacheTagMemForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_TAG_MEM_FORCE_PD` reader - The bit is used to power L1-DCache tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1DcacheTagMemForcePdR = crate::BitReader;
#[doc = "Field `L1_DCACHE_TAG_MEM_FORCE_PD` writer - The bit is used to power L1-DCache tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L1DcacheTagMemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_TAG_MEM_FORCE_PU` reader - The bit is used to power L1-DCache tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1DcacheTagMemForcePuR = crate::BitReader;
#[doc = "Field `L1_DCACHE_TAG_MEM_FORCE_PU` writer - The bit is used to power L1-DCache tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L1DcacheTagMemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to close clock gating of L1-ICache0 tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_on(&self) -> L1Icache0TagMemForceOnR {
        L1Icache0TagMemForceOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power L1-ICache0 tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_pd(&self) -> L1Icache0TagMemForcePdR {
        L1Icache0TagMemForcePdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power L1-ICache0 tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_pu(&self) -> L1Icache0TagMemForcePuR {
        L1Icache0TagMemForcePuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to close clock gating of L1-ICache1 tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_on(&self) -> L1Icache1TagMemForceOnR {
        L1Icache1TagMemForceOnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to power L1-ICache1 tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_pd(&self) -> L1Icache1TagMemForcePdR {
        L1Icache1TagMemForcePdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to power L1-ICache1 tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_pu(&self) -> L1Icache1TagMemForcePuR {
        L1Icache1TagMemForcePuR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_mem_force_on(&self) -> L1Icache2TagMemForceOnR {
        L1Icache2TagMemForceOnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_mem_force_pd(&self) -> L1Icache2TagMemForcePdR {
        L1Icache2TagMemForcePdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_tag_mem_force_pu(&self) -> L1Icache2TagMemForcePuR {
        L1Icache2TagMemForcePuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_mem_force_on(&self) -> L1Icache3TagMemForceOnR {
        L1Icache3TagMemForceOnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_mem_force_pd(&self) -> L1Icache3TagMemForcePdR {
        L1Icache3TagMemForcePdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_tag_mem_force_pu(&self) -> L1Icache3TagMemForcePuR {
        L1Icache3TagMemForcePuR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to close clock gating of L1-DCache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_dcache_tag_mem_force_on(&self) -> L1DcacheTagMemForceOnR {
        L1DcacheTagMemForceOnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to power L1-DCache tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_dcache_tag_mem_force_pd(&self) -> L1DcacheTagMemForcePdR {
        L1DcacheTagMemForcePdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to power L1-DCache tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_dcache_tag_mem_force_pu(&self) -> L1DcacheTagMemForcePuR {
        L1DcacheTagMemForcePuR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to close clock gating of L1-ICache0 tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_on(
        &mut self,
    ) -> L1Icache0TagMemForceOnW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1Icache0TagMemForceOnW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to power L1-ICache0 tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_pd(
        &mut self,
    ) -> L1Icache0TagMemForcePdW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1Icache0TagMemForcePdW::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to power L1-ICache0 tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache0_tag_mem_force_pu(
        &mut self,
    ) -> L1Icache0TagMemForcePuW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1Icache0TagMemForcePuW::new(self, 2)
    }
    #[doc = "Bit 4 - The bit is used to close clock gating of L1-ICache1 tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_on(
        &mut self,
    ) -> L1Icache1TagMemForceOnW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1Icache1TagMemForceOnW::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to power L1-ICache1 tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_pd(
        &mut self,
    ) -> L1Icache1TagMemForcePdW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1Icache1TagMemForcePdW::new(self, 5)
    }
    #[doc = "Bit 6 - The bit is used to power L1-ICache1 tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_icache1_tag_mem_force_pu(
        &mut self,
    ) -> L1Icache1TagMemForcePuW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1Icache1TagMemForcePuW::new(self, 6)
    }
    #[doc = "Bit 16 - The bit is used to close clock gating of L1-DCache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l1_dcache_tag_mem_force_on(
        &mut self,
    ) -> L1DcacheTagMemForceOnW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1DcacheTagMemForceOnW::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to power L1-DCache tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l1_dcache_tag_mem_force_pd(
        &mut self,
    ) -> L1DcacheTagMemForcePdW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1DcacheTagMemForcePdW::new(self, 17)
    }
    #[doc = "Bit 18 - The bit is used to power L1-DCache tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l1_dcache_tag_mem_force_pu(
        &mut self,
    ) -> L1DcacheTagMemForcePuW<'_, L1CacheTagMemPowerCtrlSpec> {
        L1DcacheTagMemForcePuW::new(self, 18)
    }
}
#[doc = "Cache tag memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_tag_mem_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_tag_mem_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheTagMemPowerCtrlSpec;
impl crate::RegisterSpec for L1CacheTagMemPowerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_tag_mem_power_ctrl::R`](R) reader structure"]
impl crate::Readable for L1CacheTagMemPowerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_tag_mem_power_ctrl::W`](W) writer structure"]
impl crate::Writable for L1CacheTagMemPowerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_TAG_MEM_POWER_CTRL to value 0x0005_5555"]
impl crate::Resettable for L1CacheTagMemPowerCtrlSpec {
    const RESET_VALUE: u32 = 0x0005_5555;
}
