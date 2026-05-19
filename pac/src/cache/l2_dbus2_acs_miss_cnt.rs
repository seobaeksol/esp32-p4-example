#[doc = "Register `L2_DBUS2_ACS_MISS_CNT` reader"]
pub type R = crate::R<L2Dbus2AcsMissCntSpec>;
#[doc = "Field `L2_DBUS2_MISS_CNT` reader - The register records the number of missing when L1-DCache accesses L2-Cache due to bus2 accessing L1-DCache."]
pub type L2Dbus2MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-DCache accesses L2-Cache due to bus2 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus2_miss_cnt(&self) -> L2Dbus2MissCntR {
        L2Dbus2MissCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_dbus2_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Dbus2AcsMissCntSpec;
impl crate::RegisterSpec for L2Dbus2AcsMissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_dbus2_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L2Dbus2AcsMissCntSpec {}
#[doc = "`reset()` method sets L2_DBUS2_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L2Dbus2AcsMissCntSpec {}
