#[doc = "Register `L2_IBUS1_ACS_MISS_CNT` reader"]
pub type R = crate::R<L2Ibus1AcsMissCntSpec>;
#[doc = "Field `L2_IBUS1_MISS_CNT` reader - The register records the number of missing when L1-ICache1 accesses L2-Cache due to bus1 accessing L1-ICache1."]
pub type L2Ibus1MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-ICache1 accesses L2-Cache due to bus1 accessing L1-ICache1."]
    #[inline(always)]
    pub fn l2_ibus1_miss_cnt(&self) -> L2Ibus1MissCntR {
        L2Ibus1MissCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus1 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_ibus1_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Ibus1AcsMissCntSpec;
impl crate::RegisterSpec for L2Ibus1AcsMissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_ibus1_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L2Ibus1AcsMissCntSpec {}
#[doc = "`reset()` method sets L2_IBUS1_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L2Ibus1AcsMissCntSpec {}
