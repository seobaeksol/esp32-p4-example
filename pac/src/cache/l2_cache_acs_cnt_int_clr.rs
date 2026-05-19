#[doc = "Register `L2_CACHE_ACS_CNT_INT_CLR` reader"]
pub type R = crate::R<L2CacheAcsCntIntClrSpec>;
#[doc = "Register `L2_CACHE_ACS_CNT_INT_CLR` writer"]
pub type W = crate::W<L2CacheAcsCntIntClrSpec>;
#[doc = "Field `L2_IBUS0_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
pub type L2Ibus0OvfIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS1_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
pub type L2Ibus1OvfIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS2_OVF_INT_CLR` reader - Reserved"]
pub type L2Ibus2OvfIntClrR = crate::BitReader;
#[doc = "Field `L2_IBUS3_OVF_INT_CLR` reader - Reserved"]
pub type L2Ibus3OvfIntClrR = crate::BitReader;
#[doc = "Field `L2_DBUS0_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
pub type L2Dbus0OvfIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS1_OVF_INT_CLR` writer - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
pub type L2Dbus1OvfIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS2_OVF_INT_CLR` reader - Reserved"]
pub type L2Dbus2OvfIntClrR = crate::BitReader;
#[doc = "Field `L2_DBUS3_OVF_INT_CLR` reader - Reserved"]
pub type L2Dbus3OvfIntClrR = crate::BitReader;
impl R {
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_clr(&self) -> L2Ibus2OvfIntClrR {
        L2Ibus2OvfIntClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_clr(&self) -> L2Ibus3OvfIntClrR {
        L2Ibus3OvfIntClrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_clr(&self) -> L2Dbus2OvfIntClrR {
        L2Dbus2OvfIntClrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_clr(&self) -> L2Dbus3OvfIntClrR {
        L2Dbus3OvfIntClrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_clr(&mut self) -> L2Ibus0OvfIntClrW<'_, L2CacheAcsCntIntClrSpec> {
        L2Ibus0OvfIntClrW::new(self, 8)
    }
    #[doc = "Bit 9 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_clr(&mut self) -> L2Ibus1OvfIntClrW<'_, L2CacheAcsCntIntClrSpec> {
        L2Ibus1OvfIntClrW::new(self, 9)
    }
    #[doc = "Bit 12 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_clr(&mut self) -> L2Dbus0OvfIntClrW<'_, L2CacheAcsCntIntClrSpec> {
        L2Dbus0OvfIntClrW::new(self, 12)
    }
    #[doc = "Bit 13 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_clr(&mut self) -> L2Dbus1OvfIntClrW<'_, L2CacheAcsCntIntClrSpec> {
        L2Dbus1OvfIntClrW::new(self, 13)
    }
}
#[doc = "Cache Access Counter Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_acs_cnt_int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheAcsCntIntClrSpec;
impl crate::RegisterSpec for L2CacheAcsCntIntClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_cnt_int_clr::R`](R) reader structure"]
impl crate::Readable for L2CacheAcsCntIntClrSpec {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_acs_cnt_int_clr::W`](W) writer structure"]
impl crate::Writable for L2CacheAcsCntIntClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_CNT_INT_CLR to value 0"]
impl crate::Resettable for L2CacheAcsCntIntClrSpec {}
