#[doc = "Register `RD_MAC_SYS5` reader"]
pub type R = crate::R<RdMacSys5Spec>;
#[doc = "Field `SYS_DATA_PART0_2` reader - Represents the third 32-bit of zeroth part of system data."]
pub type SysDataPart0_2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the third 32-bit of zeroth part of system data."]
    #[inline(always)]
    pub fn sys_data_part0_2(&self) -> SysDataPart0_2R {
        SysDataPart0_2R::new(self.bits)
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdMacSys5Spec;
impl crate::RegisterSpec for RdMacSys5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys5::R`](R) reader structure"]
impl crate::Readable for RdMacSys5Spec {}
#[doc = "`reset()` method sets RD_MAC_SYS5 to value 0"]
impl crate::Resettable for RdMacSys5Spec {}
