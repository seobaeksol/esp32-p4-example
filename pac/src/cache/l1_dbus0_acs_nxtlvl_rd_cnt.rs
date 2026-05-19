#[doc = "Register `L1_DBUS0_ACS_NXTLVL_RD_CNT` reader"]
pub type R = crate::R<L1Dbus0AcsNxtlvlRdCntSpec>;
#[doc = "Field `L1_DBUS0_NXTLVL_RD_CNT` reader - The register records the number of times that L1-DCache accesses L2-Cache due to bus0 accessing L1-DCache."]
pub type L1Dbus0NxtlvlRdCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of times that L1-DCache accesses L2-Cache due to bus0 accessing L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus0_nxtlvl_rd_cnt(&self) -> L1Dbus0NxtlvlRdCntR {
        L1Dbus0NxtlvlRdCntR::new(self.bits)
    }
}
#[doc = "L1-DCache bus0 Next-Level-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dbus0_acs_nxtlvl_rd_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Dbus0AcsNxtlvlRdCntSpec;
impl crate::RegisterSpec for L1Dbus0AcsNxtlvlRdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dbus0_acs_nxtlvl_rd_cnt::R`](R) reader structure"]
impl crate::Readable for L1Dbus0AcsNxtlvlRdCntSpec {}
#[doc = "`reset()` method sets L1_DBUS0_ACS_NXTLVL_RD_CNT to value 0"]
impl crate::Resettable for L1Dbus0AcsNxtlvlRdCntSpec {}
