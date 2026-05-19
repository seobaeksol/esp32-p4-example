#[doc = "Register `L1_IBUS2_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L1Ibus2AcsNxtlvlRdCntSpec>;
#[doc = "Field `L1_IBUS2_NXTLVL_RD_CNT` reader - The register records the number of times that L1-ICache accesses L2-Cache due to bus2 accessing L1-ICache2."]
pub type L1Ibus2NxtlvlRdCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-ICache accesses L2-Cache due to bus2 accessing L1-ICache2."]
    #[inline(always)]
    pub fn l1_ibus2_nxtlvl_rd_cnt(&self) -> L1Ibus2NxtlvlRdCntR {
        L1Ibus2NxtlvlRdCntR::new(self.bits)
    }
}
#[doc = "L1-ICache bus2 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_ibus2_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Ibus2AcsNxtlvlRdCntSpec;
impl crate::RegisterSpec for L1Ibus2AcsNxtlvlRdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_ibus2_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L1Ibus2AcsNxtlvlRdCntSpec {}
#[doc = "`reset()` method sets L1_IBUS2_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L1Ibus2AcsNxtlvlRdCntSpec {}
