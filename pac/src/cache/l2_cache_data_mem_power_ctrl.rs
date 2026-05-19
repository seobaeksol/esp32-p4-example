#[doc = "Register `L2_CACHE_DATA_MEM_POWER_CTRL` reader"]
pub type R = crate::R<L2CacheDataMemPowerCtrlSpec>;
#[doc = "Register `L2_CACHE_DATA_MEM_POWER_CTRL` writer"]
pub type W = crate::W<L2CacheDataMemPowerCtrlSpec>;
#[doc = "Field `L2_CACHE_DATA_MEM_FORCE_ON` reader - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating."]
pub type L2CacheDataMemForceOnR = crate::BitReader;
#[doc = "Field `L2_CACHE_DATA_MEM_FORCE_ON` writer - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating."]
pub type L2CacheDataMemForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_DATA_MEM_FORCE_PD` reader - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L2CacheDataMemForcePdR = crate::BitReader;
#[doc = "Field `L2_CACHE_DATA_MEM_FORCE_PD` writer - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down"]
pub type L2CacheDataMemForcePdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_DATA_MEM_FORCE_PU` reader - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L2CacheDataMemForcePuR = crate::BitReader;
#[doc = "Field `L2_CACHE_DATA_MEM_FORCE_PU` writer - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up"]
pub type L2CacheDataMemForcePuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20 - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l2_cache_data_mem_force_on(&self) -> L2CacheDataMemForceOnR {
        L2CacheDataMemForceOnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l2_cache_data_mem_force_pd(&self) -> L2CacheDataMemForcePdR {
        L2CacheDataMemForcePdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l2_cache_data_mem_force_pu(&self) -> L2CacheDataMemForcePuR {
        L2CacheDataMemForcePuR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - The bit is used to close clock gating of L2-Cache data memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l2_cache_data_mem_force_on(
        &mut self,
    ) -> L2CacheDataMemForceOnW<'_, L2CacheDataMemPowerCtrlSpec> {
        L2CacheDataMemForceOnW::new(self, 20)
    }
    #[doc = "Bit 21 - The bit is used to power L2-Cache data memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l2_cache_data_mem_force_pd(
        &mut self,
    ) -> L2CacheDataMemForcePdW<'_, L2CacheDataMemPowerCtrlSpec> {
        L2CacheDataMemForcePdW::new(self, 21)
    }
    #[doc = "Bit 22 - The bit is used to power L2-Cache data memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l2_cache_data_mem_force_pu(
        &mut self,
    ) -> L2CacheDataMemForcePuW<'_, L2CacheDataMemPowerCtrlSpec> {
        L2CacheDataMemForcePuW::new(self, 22)
    }
}
#[doc = "Cache data memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_data_mem_power_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_data_mem_power_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheDataMemPowerCtrlSpec;
impl crate::RegisterSpec for L2CacheDataMemPowerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_data_mem_power_ctrl::R`](R) reader structure"]
impl crate::Readable for L2CacheDataMemPowerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_data_mem_power_ctrl::W`](W) writer structure"]
impl crate::Writable for L2CacheDataMemPowerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_DATA_MEM_POWER_CTRL to value 0x0050_0000"]
impl crate::Resettable for L2CacheDataMemPowerCtrlSpec {
    const RESET_VALUE: u32 = 0x0050_0000;
}
