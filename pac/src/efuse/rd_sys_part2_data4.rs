#[doc = "Register `RD_SYS_PART2_DATA4` reader"]
pub type R = crate::R<RdSysPart2Data4Spec>;
#[doc = "Field `SYS_DATA_PART2_4` reader - Represents the fourth 32-bit of second part of system data."]
pub type SysDataPart2_4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the fourth 32-bit of second part of system data."]
    #[inline(always)]
    pub fn sys_data_part2_4(&self) -> SysDataPart2_4R {
        SysDataPart2_4R::new(self.bits)
    }
}
#[doc = "Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdSysPart2Data4Spec;
impl crate::RegisterSpec for RdSysPart2Data4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data4::R`](R) reader structure"]
impl crate::Readable for RdSysPart2Data4Spec {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA4 to value 0"]
impl crate::Resettable for RdSysPart2Data4Spec {}
