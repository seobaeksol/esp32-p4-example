#[doc = "Register `L1_DCACHE_ACS_FAIL_ID_ATTR` reader"]
pub type R = crate::R<L1DcacheAcsFailIdAttrSpec>;
#[doc = "Field `L1_DCACHE_FAIL_ID` reader - The register records the ID of fail-access when cache accesses L1-DCache."]
pub type L1DcacheFailIdR = crate::FieldReader<u16>;
#[doc = "Field `L1_DCACHE_FAIL_ATTR` reader - The register records the attribution of fail-access when cache accesses L1-DCache."]
pub type L1DcacheFailAttrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The register records the ID of fail-access when cache accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_id(&self) -> L1DcacheFailIdR {
        L1DcacheFailIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The register records the attribution of fail-access when cache accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_fail_attr(&self) -> L1DcacheFailAttrR {
        L1DcacheFailAttrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "L1-DCache Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dcache_acs_fail_id_attr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1DcacheAcsFailIdAttrSpec;
impl crate::RegisterSpec for L1DcacheAcsFailIdAttrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_acs_fail_id_attr::R`](R) reader structure"]
impl crate::Readable for L1DcacheAcsFailIdAttrSpec {}
#[doc = "`reset()` method sets L1_DCACHE_ACS_FAIL_ID_ATTR to value 0"]
impl crate::Resettable for L1DcacheAcsFailIdAttrSpec {}
