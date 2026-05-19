#[doc = "Register `HP_SLEEP_LP_REGULATOR0` reader"]
pub type R = crate::R<HpSleepLpRegulator0Spec>;
#[doc = "Register `HP_SLEEP_LP_REGULATOR0` writer"]
pub type W = crate::W<HpSleepLpRegulator0Spec>;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_SLP_XPD` reader - need_des"]
pub type HpSleepLpRegulatorSlpXpdR = crate::BitReader;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_SLP_XPD` writer - need_des"]
pub type HpSleepLpRegulatorSlpXpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_XPD` reader - need_des"]
pub type HpSleepLpRegulatorXpdR = crate::BitReader;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_XPD` writer - need_des"]
pub type HpSleepLpRegulatorXpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_SLP_DBIAS` reader - need_des"]
pub type HpSleepLpRegulatorSlpDbiasR = crate::FieldReader;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_SLP_DBIAS` writer - need_des"]
pub type HpSleepLpRegulatorSlpDbiasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_DBIAS` reader - need_des"]
pub type HpSleepLpRegulatorDbiasR = crate::FieldReader;
#[doc = "Field `HP_SLEEP_LP_REGULATOR_DBIAS` writer - need_des"]
pub type HpSleepLpRegulatorDbiasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_slp_xpd(&self) -> HpSleepLpRegulatorSlpXpdR {
        HpSleepLpRegulatorSlpXpdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_xpd(&self) -> HpSleepLpRegulatorXpdR {
        HpSleepLpRegulatorXpdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_slp_dbias(&self) -> HpSleepLpRegulatorSlpDbiasR {
        HpSleepLpRegulatorSlpDbiasR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_dbias(&self) -> HpSleepLpRegulatorDbiasR {
        HpSleepLpRegulatorDbiasR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_slp_xpd(
        &mut self,
    ) -> HpSleepLpRegulatorSlpXpdW<'_, HpSleepLpRegulator0Spec> {
        HpSleepLpRegulatorSlpXpdW::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_xpd(
        &mut self,
    ) -> HpSleepLpRegulatorXpdW<'_, HpSleepLpRegulator0Spec> {
        HpSleepLpRegulatorXpdW::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_slp_dbias(
        &mut self,
    ) -> HpSleepLpRegulatorSlpDbiasW<'_, HpSleepLpRegulator0Spec> {
        HpSleepLpRegulatorSlpDbiasW::new(self, 23)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_regulator_dbias(
        &mut self,
    ) -> HpSleepLpRegulatorDbiasW<'_, HpSleepLpRegulator0Spec> {
        HpSleepLpRegulatorDbiasW::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_regulator0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpSleepLpRegulator0Spec;
impl crate::RegisterSpec for HpSleepLpRegulator0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_lp_regulator0::R`](R) reader structure"]
impl crate::Readable for HpSleepLpRegulator0Spec {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_lp_regulator0::W`](W) writer structure"]
impl crate::Writable for HpSleepLpRegulator0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_REGULATOR0 to value 0xc660_0000"]
impl crate::Resettable for HpSleepLpRegulator0Spec {
    const RESET_VALUE: u32 = 0xc660_0000;
}
