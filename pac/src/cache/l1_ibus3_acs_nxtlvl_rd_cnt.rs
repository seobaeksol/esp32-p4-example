#[doc = "Register `L1_IBUS3_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L1Ibus3AcsNxtlvlRdCntSpec>;
#[doc = "Field `L1_IBUS3_NXTLVL_RD_CNT` reader - The register records the number of times that L1-ICache accesses L2-Cache due to bus3 accessing L1-ICache3."]
pub type L1Ibus3NxtlvlRdCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-ICache accesses L2-Cache due to bus3 accessing L1-ICache3."]
    #[inline(always)]
    pub fn l1_ibus3_nxtlvl_rd_cnt(&self) -> L1Ibus3NxtlvlRdCntR {
        L1Ibus3NxtlvlRdCntR::new(self.bits)
    }
}
#[doc = "L1-ICache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_ibus3_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Ibus3AcsNxtlvlRdCntSpec;
impl crate::RegisterSpec for L1Ibus3AcsNxtlvlRdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_ibus3_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L1Ibus3AcsNxtlvlRdCntSpec {}
#[doc = "`reset()` method sets L1_IBUS3_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L1Ibus3AcsNxtlvlRdCntSpec {}
