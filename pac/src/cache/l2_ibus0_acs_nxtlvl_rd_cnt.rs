#[doc = "Register `L2_IBUS0_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L2Ibus0AcsNxtlvlRdCntSpec>;
#[doc = "Field `L2_IBUS0_NXTLVL_RD_CNT` reader - The register records the number of times that L2-Cache accesses external memory due to L1-ICache0 accessing L2-Cache due to bus0 accessing L1-ICache0."]
pub type L2Ibus0NxtlvlRdCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L2-Cache accesses external memory due to L1-ICache0 accessing L2-Cache due to bus0 accessing L1-ICache0."]
    #[inline(always)]
    pub fn l2_ibus0_nxtlvl_rd_cnt(&self) -> L2Ibus0NxtlvlRdCntR {
        L2Ibus0NxtlvlRdCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_ibus0_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Ibus0AcsNxtlvlRdCntSpec;
impl crate::RegisterSpec for L2Ibus0AcsNxtlvlRdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_ibus0_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L2Ibus0AcsNxtlvlRdCntSpec {}
#[doc = "`reset()` method sets L2_IBUS0_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L2Ibus0AcsNxtlvlRdCntSpec {}
