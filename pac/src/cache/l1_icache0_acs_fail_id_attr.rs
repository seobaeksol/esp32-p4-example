#[doc = "Register `L1_ICACHE0_ACS_FAIL_ID_ATTR` reader"]
pub type R = crate::R<L1Icache0AcsFailIdAttrSpec>;
#[doc = "Field `L1_ICACHE0_FAIL_ID` reader - The register records the ID of fail-access when cache0 accesses L1-ICache."]
pub type L1Icache0FailIdR = crate::FieldReader<u16>;
#[doc = "Field `L1_ICACHE0_FAIL_ATTR` reader - The register records the attribution of fail-access when cache0 accesses L1-ICache."]
pub type L1Icache0FailAttrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The register records the ID of fail-access when cache0 accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache0_fail_id(&self) -> L1Icache0FailIdR {
        L1Icache0FailIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The register records the attribution of fail-access when cache0 accesses L1-ICache."]
    #[inline(always)]
    pub fn l1_icache0_fail_attr(&self) -> L1Icache0FailAttrR {
        L1Icache0FailAttrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "L1-ICache0 Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache0_acs_fail_id_attr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache0AcsFailIdAttrSpec;
impl crate::RegisterSpec for L1Icache0AcsFailIdAttrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_acs_fail_id_attr::R`](R) reader structure"]
impl crate::Readable for L1Icache0AcsFailIdAttrSpec {}
#[doc = "`reset()` method sets L1_ICACHE0_ACS_FAIL_ID_ATTR to value 0"]
impl crate::Resettable for L1Icache0AcsFailIdAttrSpec {}
