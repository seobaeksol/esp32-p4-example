#[doc = "Register `HP_MODEM_BIAS` writer"]
pub type W = crate::W<HpModemBiasSpec>;
#[doc = "Field `HP_MODEM_DCM_VSET` writer - need_des"]
pub type HpModemDcmVsetW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HP_MODEM_DCM_MODE` writer - need_des"]
pub type HpModemDcmModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MODEM_XPD_BIAS` writer - need_des"]
pub type HpModemXpdBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DBG_ATTEN` writer - need_des"]
pub type HpModemDbgAttenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_MODEM_PD_CUR` writer - need_des"]
pub type HpModemPdCurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dcm_vset(&mut self) -> HpModemDcmVsetW<'_, HpModemBiasSpec> {
        HpModemDcmVsetW::new(self, 18)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dcm_mode(&mut self) -> HpModemDcmModeW<'_, HpModemBiasSpec> {
        HpModemDcmModeW::new(self, 23)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_bias(&mut self) -> HpModemXpdBiasW<'_, HpModemBiasSpec> {
        HpModemXpdBiasW::new(self, 25)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dbg_atten(&mut self) -> HpModemDbgAttenW<'_, HpModemBiasSpec> {
        HpModemDbgAttenW::new(self, 26)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_cur(&mut self) -> HpModemPdCurW<'_, HpModemBiasSpec> {
        HpModemPdCurW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SleepW<'_, HpModemBiasSpec> {
        SleepW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_bias::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpModemBiasSpec;
impl crate::RegisterSpec for HpModemBiasSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_bias::W`](W) writer structure"]
impl crate::Writable for HpModemBiasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_BIAS to value 0"]
impl crate::Resettable for HpModemBiasSpec {}
