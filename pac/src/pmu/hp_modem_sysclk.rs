#[doc = "Register `HP_MODEM_SYSCLK` writer"]
pub type W = crate::W<HpModemSysclkSpec>;
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_NO_DIV` writer - need_des"]
pub type HpModemDigSysClkNoDivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_ICG_SYS_CLOCK_EN` writer - need_des"]
pub type HpModemIcgSysClockEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_SYS_CLK_SLP_SEL` writer - need_des"]
pub type HpModemSysClkSlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_ICG_SLP_SEL` writer - need_des"]
pub type HpModemIcgSlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_SEL` writer - need_des"]
pub type HpModemDigSysClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_sys_clk_no_div(&mut self) -> HpModemDigSysClkNoDivW<'_, HpModemSysclkSpec> {
        HpModemDigSysClkNoDivW::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_modem_icg_sys_clock_en(&mut self) -> HpModemIcgSysClockEnW<'_, HpModemSysclkSpec> {
        HpModemIcgSysClockEnW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_modem_sys_clk_slp_sel(&mut self) -> HpModemSysClkSlpSelW<'_, HpModemSysclkSpec> {
        HpModemSysClkSlpSelW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_modem_icg_slp_sel(&mut self) -> HpModemIcgSlpSelW<'_, HpModemSysclkSpec> {
        HpModemIcgSlpSelW::new(self, 29)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_sys_clk_sel(&mut self) -> HpModemDigSysClkSelW<'_, HpModemSysclkSpec> {
        HpModemDigSysClkSelW::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_sysclk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpModemSysclkSpec;
impl crate::RegisterSpec for HpModemSysclkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_sysclk::W`](W) writer structure"]
impl crate::Writable for HpModemSysclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_SYSCLK to value 0"]
impl crate::Resettable for HpModemSysclkSpec {}
