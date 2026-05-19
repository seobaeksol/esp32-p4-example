#[doc = "Register `L1_IBUS0_ACS_MISS_CNT` reader"]
pub type R = crate::R<L1Ibus0AcsMissCntSpec>;
#[doc = "Field `L1_IBUS0_MISS_CNT` reader - The register records the number of missing when bus0 accesses L1-ICache0."]
pub type L1Ibus0MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when bus0 accesses L1-ICache0."]
    #[inline(always)]
    pub fn l1_ibus0_miss_cnt(&self) -> L1Ibus0MissCntR {
        L1Ibus0MissCntR::new(self.bits)
    }
}
#[doc = "L1-ICache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_ibus0_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Ibus0AcsMissCntSpec;
impl crate::RegisterSpec for L1Ibus0AcsMissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_ibus0_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L1Ibus0AcsMissCntSpec {}
#[doc = "`reset()` method sets L1_IBUS0_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L1Ibus0AcsMissCntSpec {}
