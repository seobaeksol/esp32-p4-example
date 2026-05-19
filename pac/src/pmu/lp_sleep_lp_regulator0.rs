#[doc = "Register `LP_SLEEP_LP_REGULATOR0` reader"]
pub type R = crate::R<LpSleepLpRegulator0Spec>;
#[doc = "Register `LP_SLEEP_LP_REGULATOR0` writer"]
pub type W = crate::W<LpSleepLpRegulator0Spec>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_XPD` reader - need_des"]
pub type LpSleepLpRegulatorSlpXpdR = crate::BitReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_XPD` writer - need_des"]
pub type LpSleepLpRegulatorSlpXpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_XPD` reader - need_des"]
pub type LpSleepLpRegulatorXpdR = crate::BitReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_XPD` writer - need_des"]
pub type LpSleepLpRegulatorXpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_DBIAS` reader - need_des"]
pub type LpSleepLpRegulatorSlpDbiasR = crate::FieldReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_SLP_DBIAS` writer - need_des"]
pub type LpSleepLpRegulatorSlpDbiasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DBIAS` reader - need_des"]
pub type LpSleepLpRegulatorDbiasR = crate::FieldReader;
#[doc = "Field `LP_SLEEP_LP_REGULATOR_DBIAS` writer - need_des"]
pub type LpSleepLpRegulatorDbiasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_slp_xpd(&self) -> LpSleepLpRegulatorSlpXpdR {
        LpSleepLpRegulatorSlpXpdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_xpd(&self) -> LpSleepLpRegulatorXpdR {
        LpSleepLpRegulatorXpdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_slp_dbias(&self) -> LpSleepLpRegulatorSlpDbiasR {
        LpSleepLpRegulatorSlpDbiasR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_dbias(&self) -> LpSleepLpRegulatorDbiasR {
        LpSleepLpRegulatorDbiasR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_slp_xpd(
        &mut self,
    ) -> LpSleepLpRegulatorSlpXpdW<'_, LpSleepLpRegulator0Spec> {
        LpSleepLpRegulatorSlpXpdW::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_xpd(
        &mut self,
    ) -> LpSleepLpRegulatorXpdW<'_, LpSleepLpRegulator0Spec> {
        LpSleepLpRegulatorXpdW::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_slp_dbias(
        &mut self,
    ) -> LpSleepLpRegulatorSlpDbiasW<'_, LpSleepLpRegulator0Spec> {
        LpSleepLpRegulatorSlpDbiasW::new(self, 23)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_regulator_dbias(
        &mut self,
    ) -> LpSleepLpRegulatorDbiasW<'_, LpSleepLpRegulator0Spec> {
        LpSleepLpRegulatorDbiasW::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_regulator0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpSleepLpRegulator0Spec;
impl crate::RegisterSpec for LpSleepLpRegulator0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_sleep_lp_regulator0::R`](R) reader structure"]
impl crate::Readable for LpSleepLpRegulator0Spec {}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_lp_regulator0::W`](W) writer structure"]
impl crate::Writable for LpSleepLpRegulator0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_REGULATOR0 to value 0xc660_0000"]
impl crate::Resettable for LpSleepLpRegulator0Spec {
    const RESET_VALUE: u32 = 0xc660_0000;
}
