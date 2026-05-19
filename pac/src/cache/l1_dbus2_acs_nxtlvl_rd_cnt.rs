#[doc = "Register `L1_DBUS2_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L1Dbus2AcsNxtlvlRdCntSpec>;
#[doc = "Field `L1_DBUS2_NXTLVL_RD_CNT` reader - The register records the number of times that L1-DCache accesses L2-Cache due to bus2 accessing L1-DCache."]
pub type L1Dbus2NxtlvlRdCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-DCache accesses L2-Cache due to bus2 accessing L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus2_nxtlvl_rd_cnt(&self) -> L1Dbus2NxtlvlRdCntR {
        L1Dbus2NxtlvlRdCntR::new(self.bits)
    }
}
#[doc = "L1-DCache bus2 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dbus2_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Dbus2AcsNxtlvlRdCntSpec;
impl crate::RegisterSpec for L1Dbus2AcsNxtlvlRdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dbus2_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L1Dbus2AcsNxtlvlRdCntSpec {}
#[doc = "`reset()` method sets L1_DBUS2_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L1Dbus2AcsNxtlvlRdCntSpec {}
