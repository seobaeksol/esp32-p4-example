#[doc = "Register `L1_IBUS2_ACS_CONFLICT_CNT` reader"]
pub type R = crate::R<L1Ibus2AcsConflictCntSpec>;
#[doc = "Field `L1_IBUS2_CONFLICT_CNT` reader - The register records the number of access-conflicts when bus2 accesses L1-ICache2."]
pub type L1Ibus2ConflictCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of access-conflicts when bus2 accesses L1-ICache2."]
    #[inline(always)]
    pub fn l1_ibus2_conflict_cnt(&self) -> L1Ibus2ConflictCntR {
        L1Ibus2ConflictCntR::new(self.bits)
    }
}
#[doc = "L1-ICache bus2 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_ibus2_acs_conflict_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Ibus2AcsConflictCntSpec;
impl crate::RegisterSpec for L1Ibus2AcsConflictCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_ibus2_acs_conflict_cnt::R`](R) reader structure"]
impl crate::Readable for L1Ibus2AcsConflictCntSpec {}
#[doc = "`reset()` method sets L1_IBUS2_ACS_CONFLICT_CNT to value 0"]
impl crate::Resettable for L1Ibus2AcsConflictCntSpec {}
