#[doc = "Register `RD_SYS_PART2_DATA1` reader"]
pub type R = crate::R<RdSysPart2Data1Spec>;
#[doc = "Field `SYS_DATA_PART2_1` reader - Represents the first 32-bit of second part of system data."]
pub type SysDataPart2_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the first 32-bit of second part of system data."]
    #[inline(always)]
    pub fn sys_data_part2_1(&self) -> SysDataPart2_1R {
        SysDataPart2_1R::new(self.bits)
    }
}
#[doc = "Represents rd_sys_part2_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part2_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdSysPart2Data1Spec;
impl crate::RegisterSpec for RdSysPart2Data1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part2_data1::R`](R) reader structure"]
impl crate::Readable for RdSysPart2Data1Spec {}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA1 to value 0"]
impl crate::Resettable for RdSysPart2Data1Spec {}
