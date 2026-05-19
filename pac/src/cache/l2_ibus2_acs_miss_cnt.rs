#[doc = "Register `L2_IBUS2_ACS_MISS_CNT` reader"]
pub type R = crate::R<L2Ibus2AcsMissCntSpec>;
#[doc = "Field `L2_IBUS2_MISS_CNT` reader - The register records the number of missing when L1-ICache2 accesses L2-Cache due to bus2 accessing L1-ICache2."]
pub type L2Ibus2MissCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-ICache2 accesses L2-Cache due to bus2 accessing L1-ICache2."]
    #[inline(always)]
    pub fn l2_ibus2_miss_cnt(&self) -> L2Ibus2MissCntR {
        L2Ibus2MissCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus2 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_ibus2_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Ibus2AcsMissCntSpec;
impl crate::RegisterSpec for L2Ibus2AcsMissCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_ibus2_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for L2Ibus2AcsMissCntSpec {}
#[doc = "`reset()` method sets L2_IBUS2_ACS_MISS_CNT to value 0"]
impl crate::Resettable for L2Ibus2AcsMissCntSpec {}
