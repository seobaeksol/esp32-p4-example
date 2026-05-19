#[doc = "Register `L2_IBUS3_ACS_HIT_CNT` reader"]
pub type R = crate::R<L2Ibus3AcsHitCntSpec>;
#[doc = "Field `L2_IBUS3_HIT_CNT` reader - The register records the number of hits when L1-ICache3 accesses L2-Cache due to bus3 accessing L1-ICache3."]
pub type L2Ibus3HitCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of hits when L1-ICache3 accesses L2-Cache due to bus3 accessing L1-ICache3."]
    #[inline(always)]
    pub fn l2_ibus3_hit_cnt(&self) -> L2Ibus3HitCntR {
        L2Ibus3HitCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_ibus3_acs_hit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Ibus3AcsHitCntSpec;
impl crate::RegisterSpec for L2Ibus3AcsHitCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_ibus3_acs_hit_cnt::R`](R) reader structure"]
impl crate::Readable for L2Ibus3AcsHitCntSpec {}
#[doc = "`reset()` method sets L2_IBUS3_ACS_HIT_CNT to value 0"]
impl crate::Resettable for L2Ibus3AcsHitCntSpec {}
