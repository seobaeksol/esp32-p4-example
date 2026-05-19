#[doc = "Register `L1_ICACHE3_ACS_FAIL_ID_ATTR` reader"]
pub type R = crate::R<L1Icache3AcsFailIdAttrSpec>;
#[doc = "Field `L1_ICACHE3_FAIL_ID` reader - The register records the ID of fail-access when cache3 accesses L1-ICache."]
pub type L1Icache3FailIdR = crate::FieldReader<u16>;
#[doc = "Field `L1_ICACHE3_FAIL_ATTR` reader - The register records the attribution of fail-access when cache3 accesses L1-ICache."]
pub type L1Icache3FailAttrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The register records the ID of fail-access when cache3 accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache3_fail_id(&self) -> L1Icache3FailIdR {
        L1Icache3FailIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The register records the attribution of fail-access when cache3 accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache3_fail_attr(&self) -> L1Icache3FailAttrR {
        L1Icache3FailAttrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "L1-ICache0 Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache3_acs_fail_id_attr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache3AcsFailIdAttrSpec;
impl crate::RegisterSpec for L1Icache3AcsFailIdAttrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache3_acs_fail_id_attr::R`](R) reader structure"]
impl crate::Readable for L1Icache3AcsFailIdAttrSpec {}
#[doc = "`reset()` method sets L1_ICACHE3_ACS_FAIL_ID_ATTR to value 0"]
impl crate::Resettable for L1Icache3AcsFailIdAttrSpec {}
