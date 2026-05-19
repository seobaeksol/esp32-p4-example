#[doc = "Register `L2_CACHE_ACS_FAIL_ID_ATTR` reader"]
pub type R = crate::R<L2CacheAcsFailIdAttrSpec>;
#[doc = "Field `L2_CACHE_FAIL_ID` reader - The register records the ID of fail-access when L1-Cache accesses L2-Cache."]
pub type L2CacheFailIdR = crate::FieldReader<u16>;
#[doc = "Field `L2_CACHE_FAIL_ATTR` reader - The register records the attribution of fail-access when L1-Cache accesses L2-Cache due to cache accessing L1-Cache."]
pub type L2CacheFailAttrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The register records the ID of fail-access when L1-Cache accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_id(&self) -> L2CacheFailIdR {
        L2CacheFailIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The register records the attribution of fail-access when L1-Cache accesses L2-Cache due to cache accessing L1-Cache."]
    #[inline(always)]
    pub fn l2_cache_fail_attr(&self) -> L2CacheFailAttrR {
        L2CacheFailAttrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "L2-Cache Access Fail ID/attribution information register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_acs_fail_id_attr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CacheAcsFailIdAttrSpec;
impl crate::RegisterSpec for L2CacheAcsFailIdAttrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_fail_id_attr::R`](R) reader structure"]
impl crate::Readable for L2CacheAcsFailIdAttrSpec {}
#[doc = "`reset()` method sets L2_CACHE_ACS_FAIL_ID_ATTR to value 0"]
impl crate::Resettable for L2CacheAcsFailIdAttrSpec {}
