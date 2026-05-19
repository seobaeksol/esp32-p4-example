#[doc = "Register `HP_SLEEP_LP_DIG_POWER` reader"]
pub type R = crate::R<HpSleepLpDigPowerSpec>;
#[doc = "Register `HP_SLEEP_LP_DIG_POWER` writer"]
pub type W = crate::W<HpSleepLpDigPowerSpec>;
#[doc = "Field `HP_SLEEP_LP_PAD_SLP_SEL` reader - need_des"]
pub type HpSleepLpPadSlpSelR = crate::BitReader;
#[doc = "Field `HP_SLEEP_LP_PAD_SLP_SEL` writer - need_des"]
pub type HpSleepLpPadSlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_BOD_SOURCE_SEL` reader - need_des"]
pub type HpSleepBodSourceSelR = crate::BitReader;
#[doc = "Field `HP_SLEEP_BOD_SOURCE_SEL` writer - need_des"]
pub type HpSleepBodSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_VDDBAT_MODE` reader - need_des"]
pub type HpSleepVddbatModeR = crate::FieldReader;
#[doc = "Field `HP_SLEEP_VDDBAT_MODE` writer - need_des"]
pub type HpSleepVddbatModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP_LP_MEM_DSLP` reader - need_des"]
pub type HpSleepLpMemDslpR = crate::BitReader;
#[doc = "Field `HP_SLEEP_LP_MEM_DSLP` writer - need_des"]
pub type HpSleepLpMemDslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_PD_LP_PERI_PD_EN` reader - need_des"]
pub type HpSleepPdLpPeriPdEnR = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_LP_PERI_PD_EN` writer - need_des"]
pub type HpSleepPdLpPeriPdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_pad_slp_sel(&self) -> HpSleepLpPadSlpSelR {
        HpSleepLpPadSlpSelR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_bod_source_sel(&self) -> HpSleepBodSourceSelR {
        HpSleepBodSourceSelR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_vddbat_mode(&self) -> HpSleepVddbatModeR {
        HpSleepVddbatModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_mem_dslp(&self) -> HpSleepLpMemDslpR {
        HpSleepLpMemDslpR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_lp_peri_pd_en(&self) -> HpSleepPdLpPeriPdEnR {
        HpSleepPdLpPeriPdEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_pad_slp_sel(&mut self) -> HpSleepLpPadSlpSelW<'_, HpSleepLpDigPowerSpec> {
        HpSleepLpPadSlpSelW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_bod_source_sel(&mut self) -> HpSleepBodSourceSelW<'_, HpSleepLpDigPowerSpec> {
        HpSleepBodSourceSelW::new(self, 27)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_vddbat_mode(&mut self) -> HpSleepVddbatModeW<'_, HpSleepLpDigPowerSpec> {
        HpSleepVddbatModeW::new(self, 28)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_mem_dslp(&mut self) -> HpSleepLpMemDslpW<'_, HpSleepLpDigPowerSpec> {
        HpSleepLpMemDslpW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_lp_peri_pd_en(&mut self) -> HpSleepPdLpPeriPdEnW<'_, HpSleepLpDigPowerSpec> {
        HpSleepPdLpPeriPdEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_dig_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_dig_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpSleepLpDigPowerSpec;
impl crate::RegisterSpec for HpSleepLpDigPowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_lp_dig_power::R`](R) reader structure"]
impl crate::Readable for HpSleepLpDigPowerSpec {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_lp_dig_power::W`](W) writer structure"]
impl crate::Writable for HpSleepLpDigPowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_DIG_POWER to value 0"]
impl crate::Resettable for HpSleepLpDigPowerSpec {}
