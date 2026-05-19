#[doc = "Register `L1_IBUS3_ACS_MISS_CNT` reader"]
pub type R = crate::R<L1Ibus3AcsMissCntSpec>;
#[doc = "Field `L1_IBUS3_MISS_CNT` reader - The register records the number of missing when bus3 accesses L1-ICache3."]
pub type L1Ibus3MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when bus3 accesses L1-ICache3."]
    #[inline(always)]
    pub fn l1_ibus3_miss_cnt(&self) -> L1Ibus3MissCntR {
        L1Ibus3MissCntR::new(self.bits)
    }
}
#[doc = "L1-ICache bus3 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_ibus3_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Ibus3AcsMissCntSpec;
impl crate::RegisterSpec for L1Ibus3AcsMissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_ibus3_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L1Ibus3AcsMissCntSpec {}
#[doc = "`reset()` method sets L1_IBUS3_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L1Ibus3AcsMissCntSpec {}
