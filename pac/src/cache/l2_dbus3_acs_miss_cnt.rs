#[doc = "Register `L2_DBUS3_ACS_MISS_CNT` reader"]
pub type R = crate::R<L2Dbus3AcsMissCntSpec>;
#[doc = "Field `L2_DBUS3_MISS_CNT` reader - The register records the number of missing when L1-DCache accesses L2-Cache due to bus3 accessing L1-DCache."]
pub type L2Dbus3MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-DCache accesses L2-Cache due to bus3 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus3_miss_cnt(&self) -> L2Dbus3MissCntR {
        L2Dbus3MissCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_dbus3_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Dbus3AcsMissCntSpec;
impl crate::RegisterSpec for L2Dbus3AcsMissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_dbus3_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L2Dbus3AcsMissCntSpec {}
#[doc = "`reset()` method sets L2_DBUS3_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L2Dbus3AcsMissCntSpec {}
