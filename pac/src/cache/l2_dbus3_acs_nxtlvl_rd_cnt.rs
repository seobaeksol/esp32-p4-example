#[doc = "Register `L2_DBUS3_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L2Dbus3AcsNxtlvlRdCntSpec>;
#[doc = "Field `L2_DBUS3_NXTLVL_RD_CNT` reader - The register records the number of times that L2-Cache accesses external memory due to L1-DCache accessing L2-Cache due to bus3 accessing L1-DCache."]
pub type L2Dbus3NxtlvlRdCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L2-Cache accesses external memory due to L1-DCache accessing L2-Cache due to bus3 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus3_nxtlvl_rd_cnt(&self) -> L2Dbus3NxtlvlRdCntR {
        L2Dbus3NxtlvlRdCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus3 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_dbus3_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Dbus3AcsNxtlvlRdCntSpec;
impl crate::RegisterSpec for L2Dbus3AcsNxtlvlRdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_dbus3_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L2Dbus3AcsNxtlvlRdCntSpec {}
#[doc = "`reset()` method sets L2_DBUS3_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L2Dbus3AcsNxtlvlRdCntSpec {}
