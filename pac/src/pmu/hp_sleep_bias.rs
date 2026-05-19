#[doc = "Register `HP_SLEEP_BIAS` reader"]
pub type R = crate::R<HpSleepBiasSpec>;
#[doc = "Register `HP_SLEEP_BIAS` writer"]
pub type W = crate::W<HpSleepBiasSpec>;
#[doc = "Field `HP_SLEEP_DCM_VSET` reader - need_des"]
pub type HpSleepDcmVsetR = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DCM_VSET` writer - need_des"]
pub type HpSleepDcmVsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HP_SLEEP_DCM_MODE` reader - need_des"]
pub type HpSleepDcmModeR = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DCM_MODE` writer - need_des"]
pub type HpSleepDcmModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP_XPD_BIAS` reader - need_des"]
pub type HpSleepXpdBiasR = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_BIAS` writer - need_des"]
pub type HpSleepXpdBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_DBG_ATTEN` reader - need_des"]
pub type HpSleepDbgAttenR = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DBG_ATTEN` writer - need_des"]
pub type HpSleepDbgAttenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_SLEEP_PD_CUR` reader - need_des"]
pub type HpSleepPdCurR = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_CUR` writer - need_des"]
pub type HpSleepPdCurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - need_des"]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_vset(&self) -> HpSleepDcmVsetR {
        HpSleepDcmVsetR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_mode(&self) -> HpSleepDcmModeR {
        HpSleepDcmModeR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_bias(&self) -> HpSleepXpdBiasR {
        HpSleepXpdBiasR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dbg_atten(&self) -> HpSleepDbgAttenR {
        HpSleepDbgAttenR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_cur(&self) -> HpSleepPdCurR {
        HpSleepPdCurR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_vset(&mut self) -> HpSleepDcmVsetW<'_, HpSleepBiasSpec> {
        HpSleepDcmVsetW::new(self, 18)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_mode(&mut self) -> HpSleepDcmModeW<'_, HpSleepBiasSpec> {
        HpSleepDcmModeW::new(self, 23)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_bias(&mut self) -> HpSleepXpdBiasW<'_, HpSleepBiasSpec> {
        HpSleepXpdBiasW::new(self, 25)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dbg_atten(&mut self) -> HpSleepDbgAttenW<'_, HpSleepBiasSpec> {
        HpSleepDbgAttenW::new(self, 26)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_cur(&mut self) -> HpSleepPdCurW<'_, HpSleepBiasSpec> {
        HpSleepPdCurW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SleepW<'_, HpSleepBiasSpec> {
        SleepW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpSleepBiasSpec;
impl crate::RegisterSpec for HpSleepBiasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_bias::R`](R) reader structure"]
impl crate::Readable for HpSleepBiasSpec {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_bias::W`](W) writer structure"]
impl crate::Writable for HpSleepBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_BIAS to value 0x0050_0000"]
impl crate::Resettable for HpSleepBiasSpec {
    const RESET_VALUE: u32 = 0x0050_0000;
}
