#[doc = "Register `L1_DBUS3_ACS_NXTLVL_WR_CNT` reader"]
pub type R = crate::R<L1Dbus3AcsNxtlvlWrCntSpec>;
#[doc = "Field `L1_DBUS3_NXTLVL_WR_CNT` reader - The register records the number of write back when bus0 accesses L1-DCache."]
pub type L1Dbus3NxtlvlWrCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of write back when bus0 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus3_nxtlvl_wr_cnt(&self) -> L1Dbus3NxtlvlWrCntR {
        L1Dbus3NxtlvlWrCntR::new(self.bits)
    }
}
#[doc = "L1-DCache bus3 WB-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dbus3_acs_nxtlvl_wr_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Dbus3AcsNxtlvlWrCntSpec;
impl crate::RegisterSpec for L1Dbus3AcsNxtlvlWrCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dbus3_acs_nxtlvl_wr_cnt::R`](R) reader structure"]
impl crate::Readable for L1Dbus3AcsNxtlvlWrCntSpec {}
#[doc = "`reset()` method sets L1_DBUS3_ACS_NXTLVL_WR_CNT to value 0"]
impl crate::Resettable for L1Dbus3AcsNxtlvlWrCntSpec {}
