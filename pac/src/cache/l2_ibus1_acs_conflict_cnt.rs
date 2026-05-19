#[doc = "Register `L2_IBUS1_ACS_CONFLICT_CNT` reader"]
pub type R = crate::R<L2Ibus1AcsConflictCntSpec>;
#[doc = "Field `L2_IBUS1_CONFLICT_CNT` reader - The register records the number of access-conflicts when L1-ICache1 accesses L2-Cache due to bus1 accessing L1-ICache1."]
pub type L2Ibus1ConflictCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of access-conflicts when L1-ICache1 accesses L2-Cache due to bus1 accessing L1-ICache1."]
    #[inline(always)]
    pub fn l2_ibus1_conflict_cnt(&self) -> L2Ibus1ConflictCntR {
        L2Ibus1ConflictCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus1 Conflict-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_ibus1_acs_conflict_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Ibus1AcsConflictCntSpec;
impl crate::RegisterSpec for L2Ibus1AcsConflictCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_ibus1_acs_conflict_cnt::R`](R) reader structure"]
impl crate::Readable for L2Ibus1AcsConflictCntSpec {}
#[doc = "`reset()` method sets L2_IBUS1_ACS_CONFLICT_CNT to value 0"]
impl crate::Resettable for L2Ibus1AcsConflictCntSpec {}
