#[doc = "Register `LP_SLEEP_LP_REGULATOR1` reader"]
pub type R = crate::R<LpSleepLpRegulator1Spec>;
#[doc = "Register `LP_SLEEP_LP_REGULATOR1` writer"]
pub type W = crate::W<LpSleepLpRegulator1Spec>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DRV_B` reader - need_des"]
pub type LpSleepLpRegulatorDrvBR = crate::FieldReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DRV_B` writer - need_des"]
pub type LpSleepLpRegulatorDrvBW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_drv_b(&self) -> LpSleepLpRegulatorDrvBR {
        LpSleepLpRegulatorDrvBR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_drv_b(
        &mut self,
    ) -> LpSleepLpRegulatorDrvBW<'_, LpSleepLpRegulator1Spec> {
        LpSleepLpRegulatorDrvBW::new(self, 26)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_regulator1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpSleepLpRegulator1Spec;
impl crate::RegisterSpec for LpSleepLpRegulator1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_sleep_lp_regulator1::R`](R) reader structure"]
impl crate::Readable for LpSleepLpRegulator1Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_lp_regulator1::W`](W) writer structure"]
impl crate::Writable for LpSleepLpRegulator1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_REGULATOR1 to value 0"]
impl crate::Resettable for LpSleepLpRegulator1Spec {}
