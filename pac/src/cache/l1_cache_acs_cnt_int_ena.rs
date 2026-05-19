#[doc = "Register `L1_CACHE_ACS_CNT_INT_ENA` reader"]
pub type R = crate::R<L1CacheAcsCntIntEnaSpec>;
#[doc = "Register `L1_CACHE_ACS_CNT_INT_ENA` writer"]
pub type W = crate::W<L1CacheAcsCntIntEnaSpec>;
#[doc = "Field `L1_IBUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1Ibus0OvfIntEnaR = crate::BitReader;
#[doc = "Field `L1_IBUS0_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1Ibus0OvfIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1Ibus1OvfIntEnaR = crate::BitReader;
#[doc = "Field `L1_IBUS1_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1Ibus1OvfIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_IBUS2_OVF_INT_ENA` reader - Reserved"]
pub type L1Ibus2OvfIntEnaR = crate::BitReader;
#[doc = "Field `L1_IBUS3_OVF_INT_ENA` reader - Reserved"]
pub type L1Ibus3OvfIntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS0_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1Dbus0OvfIntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS0_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1Dbus0OvfIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS1_OVF_INT_ENA` reader - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1Dbus1OvfIntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS1_OVF_INT_ENA` writer - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1Dbus1OvfIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DBUS2_OVF_INT_ENA` reader - Reserved"]
pub type L1Dbus2OvfIntEnaR = crate::BitReader;
#[doc = "Field `L1_DBUS3_OVF_INT_ENA` reader - Reserved"]
pub type L1Dbus3OvfIntEnaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_ena(&self) -> L1Ibus0OvfIntEnaR {
        L1Ibus0OvfIntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_ena(&self) -> L1Ibus1OvfIntEnaR {
        L1Ibus1OvfIntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_ena(&self) -> L1Ibus2OvfIntEnaR {
        L1Ibus2OvfIntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_ena(&self) -> L1Ibus3OvfIntEnaR {
        L1Ibus3OvfIntEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_ovf_int_ena(&self) -> L1Dbus0OvfIntEnaR {
        L1Dbus0OvfIntEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_ovf_int_ena(&self) -> L1Dbus1OvfIntEnaR {
        L1Dbus1OvfIntEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_ena(&self) -> L1Dbus2OvfIntEnaR {
        L1Dbus2OvfIntEnaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_ena(&self) -> L1Dbus3OvfIntEnaR {
        L1Dbus3OvfIntEnaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_ena(&mut self) -> L1Ibus0OvfIntEnaW<'_, L1CacheAcsCntIntEnaSpec> {
        L1Ibus0OvfIntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_ena(&mut self) -> L1Ibus1OvfIntEnaW<'_, L1CacheAcsCntIntEnaSpec> {
        L1Ibus1OvfIntEnaW::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_ovf_int_ena(&mut self) -> L1Dbus0OvfIntEnaW<'_, L1CacheAcsCntIntEnaSpec> {
        L1Dbus0OvfIntEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to enable interrupt of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_ovf_int_ena(&mut self) -> L1Dbus1OvfIntEnaW<'_, L1CacheAcsCntIntEnaSpec> {
        L1Dbus1OvfIntEnaW::new(self, 5)
    }
}
#[doc = "Cache Access Counter Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_cnt_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAcsCntIntEnaSpec;
impl crate::RegisterSpec for L1CacheAcsCntIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_cnt_int_ena::R`](R) reader structure"]
impl crate::Readable for L1CacheAcsCntIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_acs_cnt_int_ena::W`](W) writer structure"]
impl crate::Writable for L1CacheAcsCntIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_ENA to value 0"]
impl crate::Resettable for L1CacheAcsCntIntEnaSpec {}
