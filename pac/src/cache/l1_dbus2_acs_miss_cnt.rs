#[doc = "Register `L1_DBUS2_ACS_MISS_CNT` reader"]
pub type R = crate::R<L1Dbus2AcsMissCntSpec>;
#[doc = "Field `L1_DBUS2_MISS_CNT` reader - The register records the number of missing when bus2 accesses L1-DCache."]
pub type L1Dbus2MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when bus2 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus2_miss_cnt(&self) -> L1Dbus2MissCntR {
        L1Dbus2MissCntR::new(self.bits)
    }
}
#[doc = "L1-DCache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dbus2_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Dbus2AcsMissCntSpec;
impl crate::RegisterSpec for L1Dbus2AcsMissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dbus2_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L1Dbus2AcsMissCntSpec {}
#[doc = "`reset()` method sets L1_DBUS2_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L1Dbus2AcsMissCntSpec {}
