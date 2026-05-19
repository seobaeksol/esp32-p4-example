#[doc = "Register `CLK_STATE1` reader"]
pub type R = crate::R<ClkState1Spec>;
#[doc = "Field `PMU_ICG_FUNC_EN_STATE` reader - need_des"]
pub type PmuIcgFuncEnStateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn pmu_icg_func_en_state(&self) -> PmuIcgFuncEnStateR {
        PmuIcgFuncEnStateR::new(self.bits)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkState1Spec;
impl crate::RegisterSpec for ClkState1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_state1::R`](R) reader structure"]
impl crate::Readable for ClkState1Spec {}
#[doc = "`reset()` method sets CLK_STATE1 to value 0xffff_ffff"]
impl crate::Resettable for ClkState1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
