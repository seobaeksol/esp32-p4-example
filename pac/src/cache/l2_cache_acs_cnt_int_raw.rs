#[doc = "Register `L2_CACHE_ACS_CNT_INT_RAW` reader"]
pub type R = crate::R<L2CacheAcsCntIntRawSpec>;
#[doc = "Register `L2_CACHE_ACS_CNT_INT_RAW` writer"]
pub type W = crate::W<L2CacheAcsCntIntRawSpec>;
#[doc = "Field `L2_IBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
pub type L2Ibus0OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_IBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
pub type L2Ibus0OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
pub type L2Ibus1OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_IBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
pub type L2Ibus1OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
pub type L2Ibus2OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_IBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
pub type L2Ibus2OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
pub type L2Ibus3OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_IBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
pub type L2Ibus3OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS0_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
pub type L2Dbus0OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_DBUS0_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
pub type L2Dbus0OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS1_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
pub type L2Dbus1OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_DBUS1_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
pub type L2Dbus1OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS2_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
pub type L2Dbus2OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_DBUS2_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
pub type L2Dbus2OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS3_OVF_INT_RAW` reader - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
pub type L2Dbus3OvfIntRawR = crate::BitReader;
#[doc = "Field `L2_DBUS3_OVF_INT_RAW` writer - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
pub type L2Dbus3OvfIntRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_raw(&self) -> L2Ibus0OvfIntRawR {
        L2Ibus0OvfIntRawR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_raw(&self) -> L2Ibus1OvfIntRawR {
        L2Ibus1OvfIntRawR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_raw(&self) -> L2Ibus2OvfIntRawR {
        L2Ibus2OvfIntRawR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_raw(&self) -> L2Ibus3OvfIntRawR {
        L2Ibus3OvfIntRawR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_raw(&self) -> L2Dbus0OvfIntRawR {
        L2Dbus0OvfIntRawR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_raw(&self) -> L2Dbus1OvfIntRawR {
        L2Dbus1OvfIntRawR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_raw(&self) -> L2Dbus2OvfIntRawR {
        L2Dbus2OvfIntRawR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_raw(&self) -> L2Dbus3OvfIntRawR {
        L2Dbus3OvfIntRawR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-ICache0."]
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_raw(&mut self) -> L2Ibus0OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Ibus0OvfIntRawW::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-ICache1."]
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_raw(&mut self) -> L2Ibus1OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Ibus1OvfIntRawW::new(self, 9)
    }
    #[doc = "Bit 10 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-ICache2."]
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_raw(&mut self) -> L2Ibus2OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Ibus2OvfIntRawW::new(self, 10)
    }
    #[doc = "Bit 11 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-ICache3."]
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_raw(&mut self) -> L2Ibus3OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Ibus3OvfIntRawW::new(self, 11)
    }
    #[doc = "Bit 12 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus0 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_raw(&mut self) -> L2Dbus0OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Dbus0OvfIntRawW::new(self, 12)
    }
    #[doc = "Bit 13 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus1 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_raw(&mut self) -> L2Dbus1OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Dbus1OvfIntRawW::new(self, 13)
    }
    #[doc = "Bit 14 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus2 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_raw(&mut self) -> L2Dbus2OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Dbus2OvfIntRawW::new(self, 14)
    }
    #[doc = "Bit 15 - The raw bit of the interrupt of one of counters overflow that occurs in L2-Cache due to bus3 accesses L2-DCache."]
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_raw(&mut self) -> L2Dbus3OvfIntRawW<'_, L2CacheAcsCntIntRawSpec> {
        L2Dbus3OvfIntRawW::new(self, 15)
    }
}
#[doc = "Cache Access Counter Interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_acs_cnt_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheAcsCntIntRawSpec;
impl crate::RegisterSpec for L2CacheAcsCntIntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_cnt_int_raw::R`](R) reader structure"]
impl crate::Readable for L2CacheAcsCntIntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_acs_cnt_int_raw::W`](W) writer structure"]
impl crate::Writable for L2CacheAcsCntIntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_CNT_INT_RAW to value 0"]
impl crate::Resettable for L2CacheAcsCntIntRawSpec {}
