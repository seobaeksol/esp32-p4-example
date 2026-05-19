#[doc = "Register `L1_ICACHE2_PRELOCK_CONF` reader"]
pub type R = crate::R<L1Icache2PrelockConfSpec>;
#[doc = "Field `L1_ICACHE2_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L1-ICache2."]
pub type L1Icache2PrelockSct0EnR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L1-ICache2."]
pub type L1Icache2PrelockSct1EnR = crate::BitReader;
#[doc = "Field `L1_ICACHE2_PRELOCK_RGID` reader - The bit is used to set the gid of l1 icache2 prelock."]
pub type L1Icache2PrelockRgidR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-ICache2."]
    #[inline(always)]
    pub fn l1_icache2_prelock_sct0_en(&self) -> L1Icache2PrelockSct0EnR {
        L1Icache2PrelockSct0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-ICache2."]
    #[inline(always)]
    pub fn l1_icache2_prelock_sct1_en(&self) -> L1Icache2PrelockSct1EnR {
        L1Icache2PrelockSct1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 icache2 prelock."]
    #[inline(always)]
    pub fn l1_icache2_prelock_rgid(&self) -> L1Icache2PrelockRgidR {
        L1Icache2PrelockRgidR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[doc = "L1 instruction Cache 2 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache2_prelock_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache2PrelockConfSpec;
impl crate::RegisterSpec for L1Icache2PrelockConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache2_prelock_conf::R`](R) reader structure"]
impl crate::Readable for L1Icache2PrelockConfSpec {}
#[doc = "`reset()` method sets L1_ICACHE2_PRELOCK_CONF to value 0"]
impl crate::Resettable for L1Icache2PrelockConfSpec {}
