#[doc = "Register `L1_CACHE_ACS_CNT_INT_ST` reader"]
pub type R = crate::R<L1CacheAcsCntIntStSpec>;
#[doc = "Field `L1_IBUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
pub type L1Ibus0OvfIntStR = crate::BitReader;
#[doc = "Field `L1_IBUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
pub type L1Ibus1OvfIntStR = crate::BitReader;
#[doc = "Field `L1_IBUS2_OVF_INT_ST` reader - Reserved"]
pub type L1Ibus2OvfIntStR = crate::BitReader;
#[doc = "Field `L1_IBUS3_OVF_INT_ST` reader - Reserved"]
pub type L1Ibus3OvfIntStR = crate::BitReader;
#[doc = "Field `L1_DBUS0_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
pub type L1Dbus0OvfIntStR = crate::BitReader;
#[doc = "Field `L1_DBUS1_OVF_INT_ST` reader - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
pub type L1Dbus1OvfIntStR = crate::BitReader;
#[doc = "Field `L1_DBUS2_OVF_INT_ST` reader - Reserved"]
pub type L1Dbus2OvfIntStR = crate::BitReader;
#[doc = "Field `L1_DBUS3_OVF_INT_ST` reader - Reserved"]
pub type L1Dbus3OvfIntStR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache0 due to bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_ovf_int_st(&self) -> L1Ibus0OvfIntStR {
        L1Ibus0OvfIntStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-ICache1 due to bus1 accesses L1-ICache1."]
    #[inline(always)]
    pub fn l1_ibus1_ovf_int_st(&self) -> L1Ibus1OvfIntStR {
        L1Ibus1OvfIntStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus2_ovf_int_st(&self) -> L1Ibus2OvfIntStR {
        L1Ibus2OvfIntStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_ibus3_ovf_int_st(&self) -> L1Ibus3OvfIntStR {
        L1Ibus3OvfIntStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_ovf_int_st(&self) -> L1Dbus0OvfIntStR {
        L1Dbus0OvfIntStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit indicates the interrupt status of one of counters overflow that occurs in L1-DCache due to bus1 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus1_ovf_int_st(&self) -> L1Dbus1OvfIntStR {
        L1Dbus1OvfIntStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus2_ovf_int_st(&self) -> L1Dbus2OvfIntStR {
        L1Dbus2OvfIntStR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn l1_dbus3_ovf_int_st(&self) -> L1Dbus3OvfIntStR {
        L1Dbus3OvfIntStR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Cache Access Counter Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_acs_cnt_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CacheAcsCntIntStSpec;
impl crate::RegisterSpec for L1CacheAcsCntIntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_acs_cnt_int_st::R`](R) reader structure"]
impl crate::Readable for L1CacheAcsCntIntStSpec {}
#[doc = "`reset()` method sets L1_CACHE_ACS_CNT_INT_ST to value 0"]
impl crate::Resettable for L1CacheAcsCntIntStSpec {}
