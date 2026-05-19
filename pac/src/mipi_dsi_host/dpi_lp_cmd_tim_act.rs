#[doc = "Register `DPI_LP_CMD_TIM_ACT` reader"]
pub type R = crate::R<DpiLpCmdTimActSpec>;
#[doc = "Field `INVACT_LPCMD_TIME_ACT` reader - NA"]
pub type InvactLpcmdTimeActR = crate::FieldReader;
#[doc = "Field `OUTVACT_LPCMD_TIME_ACT` reader - NA"]
pub type OutvactLpcmdTimeActR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn invact_lpcmd_time_act(&self) -> InvactLpcmdTimeActR {
        InvactLpcmdTimeActR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn outvact_lpcmd_time_act(&self) -> OutvactLpcmdTimeActR {
        OutvactLpcmdTimeActR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_lp_cmd_tim_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiLpCmdTimActSpec;
impl crate::RegisterSpec for DpiLpCmdTimActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_lp_cmd_tim_act::R`](R) reader structure"]
impl crate::Readable for DpiLpCmdTimActSpec {}
#[doc = "`reset()` method sets DPI_LP_CMD_TIM_ACT to value 0"]
impl crate::Resettable for DpiLpCmdTimActSpec {}
