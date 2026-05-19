#[doc = "Register `RD_SYS_PART1_DATA%s` reader"]
pub type R = crate::R<RdSysPart1DataSpec>;
#[doc = "Field `SYS_DATA_PART1` reader - Represents the zeroth 32-bit of first part of system data."]
pub type SysDataPart1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the zeroth 32-bit of first part of system data."]
    #[inline(always)]
    pub fn sys_data_part1(&self) -> SysDataPart1R {
        SysDataPart1R::new(self.bits)
    }
}
#[doc = "Represents rd_sys_part1_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_sys_part1_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdSysPart1DataSpec;
impl crate::RegisterSpec for RdSysPart1DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_sys_part1_data::R`](R) reader structure"]
impl crate::Readable for RdSysPart1DataSpec {}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA%s to value 0"]
impl crate::Resettable for RdSysPart1DataSpec {}
