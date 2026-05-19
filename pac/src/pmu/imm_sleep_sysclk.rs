#[doc = "Register `IMM_SLEEP_SYSCLK` writer"]
pub type W = crate::W<ImmSleepSysclkSpec>;
#[doc = "Field `UPDATE_DIG_ICG_SWITCH` writer - need_des"]
pub type UpdateDigIcgSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_ICG_SLP_SEL` writer - need_des"]
pub type TieLowIcgSlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_ICG_SLP_SEL` writer - need_des"]
pub type TieHighIcgSlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE_DIG_SYS_CLK_SEL` writer - need_des"]
pub type UpdateDigSysClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn update_dig_icg_switch(&mut self) -> UpdateDigIcgSwitchW<'_, ImmSleepSysclkSpec> {
        UpdateDigIcgSwitchW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn tie_low_icg_slp_sel(&mut self) -> TieLowIcgSlpSelW<'_, ImmSleepSysclkSpec> {
        TieLowIcgSlpSelW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn tie_high_icg_slp_sel(&mut self) -> TieHighIcgSlpSelW<'_, ImmSleepSysclkSpec> {
        TieHighIcgSlpSelW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn update_dig_sys_clk_sel(&mut self) -> UpdateDigSysClkSelW<'_, ImmSleepSysclkSpec> {
        UpdateDigSysClkSelW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_sleep_sysclk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImmSleepSysclkSpec;
impl crate::RegisterSpec for ImmSleepSysclkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_sleep_sysclk::W`](W) writer structure"]
impl crate::Writable for ImmSleepSysclkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_SLEEP_SYSCLK to value 0"]
impl crate::Resettable for ImmSleepSysclkSpec {}
