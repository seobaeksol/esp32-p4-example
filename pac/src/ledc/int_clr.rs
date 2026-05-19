#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `TIMER_OVF(0-3)` writer - Set this bit to clear the TIMER%s_OVF interrupt."]
pub type TimerOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH(0-7)` writer - Set this bit to clear the DUTY_CHNG_END_CH%s interrupt."]
pub type DutyChngEndChW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OVF_CNT_CH(0-7)` writer - Set this bit to clear the OVF_CNT_CH%s interrupt."]
pub type OvfCntChW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Set this bit to clear the TIMER(0-3)_OVF interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_OVF` field.</div>"]
    #[inline(always)]
    pub fn timer_ovf(&mut self, n: u8) -> TimerOvfW<'_, IntClrSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TimerOvfW::new(self, n)
    }
    #[doc = "Bit 0 - Set this bit to clear the TIMER0_OVF interrupt."]
    #[inline(always)]
    pub fn timer0_ovf(&mut self) -> TimerOvfW<'_, IntClrSpec> {
        TimerOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the TIMER1_OVF interrupt."]
    #[inline(always)]
    pub fn timer1_ovf(&mut self) -> TimerOvfW<'_, IntClrSpec> {
        TimerOvfW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the TIMER2_OVF interrupt."]
    #[inline(always)]
    pub fn timer2_ovf(&mut self) -> TimerOvfW<'_, IntClrSpec> {
        TimerOvfW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the TIMER3_OVF interrupt."]
    #[inline(always)]
    pub fn timer3_ovf(&mut self) -> TimerOvfW<'_, IntClrSpec> {
        TimerOvfW::new(self, 3)
    }
    #[doc = "Set this bit to clear the DUTY_CHNG_END_CH(0-7) interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_CH0` field.</div>"]
    #[inline(always)]
    pub fn duty_chng_end_ch(&mut self, n: u8) -> DutyChngEndChW<'_, IntClrSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DutyChngEndChW::new(self, n + 4)
    }
    #[doc = "Bit 4 - Set this bit to clear the DUTY_CHNG_END_CH0 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch0(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the DUTY_CHNG_END_CH1 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch1(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the DUTY_CHNG_END_CH2 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch2(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the DUTY_CHNG_END_CH3 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch3(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the DUTY_CHNG_END_CH4 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch4(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the DUTY_CHNG_END_CH5 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch5(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the DUTY_CHNG_END_CH6 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch6(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear the DUTY_CHNG_END_CH7 interrupt."]
    #[inline(always)]
    pub fn duty_chng_end_ch7(&mut self) -> DutyChngEndChW<'_, IntClrSpec> {
        DutyChngEndChW::new(self, 11)
    }
    #[doc = "Set this bit to clear the OVF_CNT_CH(0-7) interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OVF_CNT_CH0` field.</div>"]
    #[inline(always)]
    pub fn ovf_cnt_ch(&mut self, n: u8) -> OvfCntChW<'_, IntClrSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        OvfCntChW::new(self, n + 12)
    }
    #[doc = "Bit 12 - Set this bit to clear the OVF_CNT_CH0 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch0(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear the OVF_CNT_CH1 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch1(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear the OVF_CNT_CH2 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch2(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear the OVF_CNT_CH3 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch3(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 15)
    }
    #[doc = "Bit 16 - Set this bit to clear the OVF_CNT_CH4 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch4(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to clear the OVF_CNT_CH5 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch5(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to clear the OVF_CNT_CH6 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch6(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to clear the OVF_CNT_CH7 interrupt."]
    #[inline(always)]
    pub fn ovf_cnt_ch7(&mut self) -> OvfCntChW<'_, IntClrSpec> {
        OvfCntChW::new(self, 19)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
