#[doc = "Register `L2_DBUS1_ACS_NXTLVL_WR_CNT` reader"]
pub type R = crate::R<L2Dbus1AcsNxtlvlWrCntSpec>;
#[doc = "Field `L2_DBUS1_NXTLVL_WR_CNT` reader - The register records the number of write back when L1-DCache accesses L2-Cache due to bus1 accessing L1-DCache."]
pub type L2Dbus1NxtlvlWrCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of write back when L1-DCache accesses L2-Cache due to bus1 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus1_nxtlvl_wr_cnt(&self) -> L2Dbus1NxtlvlWrCntR {
        L2Dbus1NxtlvlWrCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus1 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_dbus1_acs_nxtlvl_wr_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Dbus1AcsNxtlvlWrCntSpec;
impl crate::RegisterSpec for L2Dbus1AcsNxtlvlWrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_dbus1_acs_nxtlvl_wr_cnt::R`](R) reader structure"]
impl crate::Readable for L2Dbus1AcsNxtlvlWrCntSpec {}
#[doc = "`reset()` method sets L2_DBUS1_ACS_NXTLVL_WR_CNT to value 0"]
impl crate::Resettable for L2Dbus1AcsNxtlvlWrCntSpec {}
