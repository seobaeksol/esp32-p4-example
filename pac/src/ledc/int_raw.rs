#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `TIMER_OVF(0-3)` reader - Triggered when the timer%s has reached its maximum counter value."]
pub type TimerOvfR = crate::BitReader;
#[doc = "Field `TIMER_OVF(0-3)` writer - Triggered when the timer%s has reached its maximum counter value."]
pub type TimerOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY_CHNG_END_CH(0-7)` reader - Interrupt raw bit for channel %s. Triggered when the gradual change of duty has finished."]
pub type DutyChngEndChR = crate::BitReader;
#[doc = "Field `DUTY_CHNG_END_CH(0-7)` writer - Interrupt raw bit for channel %s. Triggered when the gradual change of duty has finished."]
pub type DutyChngEndChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_CNT_CH(0-7)` reader - Interrupt raw bit for channel %s. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
pub type OvfCntChR = crate::BitReader;
#[doc = "Field `OVF_CNT_CH(0-7)` writer - Interrupt raw bit for channel %s. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
pub type OvfCntChW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Triggered when the timer(0-3) has reached its maximum counter value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_OVF` field.</div>"]
    #[inline(always)]
    pub fn timer_ovf(&self, n: u8) -> TimerOvfR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TimerOvfR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Triggered when the timer(0-3) has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer_ovf_iter(&self) -> impl Iterator<Item = TimerOvfR> + '_ {
        (0..4).map(move |n| TimerOvfR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Triggered when the timer0 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer0_ovf(&self) -> TimerOvfR {
        TimerOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Triggered when the timer1 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer1_ovf(&self) -> TimerOvfR {
        TimerOvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Triggered when the timer2 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer2_ovf(&self) -> TimerOvfR {
        TimerOvfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Triggered when the timer3 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer3_ovf(&self) -> TimerOvfR {
        TimerOvfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Interrupt raw bit for channel (0-7). Triggered when the gradual change of duty has finished."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_CH0` field.</div>"]
    #[inline(always)]
    pub fn duty_chng_end_ch(&self, n: u8) -> DutyChngEndChR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DutyChngEndChR::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Interrupt raw bit for channel (0-7). Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch_iter(&self) -> impl Iterator<Item = DutyChngEndChR> + '_ {
        (0..8).map(move |n| DutyChngEndChR::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch0(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch1(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch2(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch3(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch4(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch5(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch6(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch7(&self) -> DutyChngEndChR {
        DutyChngEndChR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Interrupt raw bit for channel (0-7). Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OVF_CNT_CH0` field.</div>"]
    #[inline(always)]
    pub fn ovf_cnt_ch(&self, n: u8) -> OvfCntChR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        OvfCntChR::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Interrupt raw bit for channel (0-7). Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch_iter(&self) -> impl Iterator<Item = OvfCntChR> + '_ {
        (0..8).map(move |n| OvfCntChR::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - Interrupt raw bit for channel 0. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch0(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt raw bit for channel 1. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch1(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt raw bit for channel 2. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch2(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt raw bit for channel 3. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch3(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt raw bit for channel 4. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch4(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt raw bit for channel 5. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch5(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt raw bit for channel 6. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch6(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt raw bit for channel 7. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch7(&self) -> OvfCntChR {
        OvfCntChR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Triggered when the timer(0-3) has reached its maximum counter value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER0_OVF` field.</div>"]
    #[inline(always)]
    pub fn timer_ovf(&mut self, n: u8) -> TimerOvfW<'_, IntRawSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        TimerOvfW::new(self, n)
    }
    #[doc = "Bit 0 - Triggered when the timer0 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer0_ovf(&mut self) -> TimerOvfW<'_, IntRawSpec> {
        TimerOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Triggered when the timer1 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer1_ovf(&mut self) -> TimerOvfW<'_, IntRawSpec> {
        TimerOvfW::new(self, 1)
    }
    #[doc = "Bit 2 - Triggered when the timer2 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer2_ovf(&mut self) -> TimerOvfW<'_, IntRawSpec> {
        TimerOvfW::new(self, 2)
    }
    #[doc = "Bit 3 - Triggered when the timer3 has reached its maximum counter value."]
    #[inline(always)]
    pub fn timer3_ovf(&mut self) -> TimerOvfW<'_, IntRawSpec> {
        TimerOvfW::new(self, 3)
    }
    #[doc = "Interrupt raw bit for channel (0-7). Triggered when the gradual change of duty has finished."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DUTY_CHNG_END_CH0` field.</div>"]
    #[inline(always)]
    pub fn duty_chng_end_ch(&mut self, n: u8) -> DutyChngEndChW<'_, IntRawSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DutyChngEndChW::new(self, n + 4)
    }
    #[doc = "Bit 4 - Interrupt raw bit for channel 0. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch0(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt raw bit for channel 1. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch1(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt raw bit for channel 2. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch2(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt raw bit for channel 3. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch3(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt raw bit for channel 4. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch4(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt raw bit for channel 5. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch5(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt raw bit for channel 6. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch6(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt raw bit for channel 7. Triggered when the gradual change of duty has finished."]
    #[inline(always)]
    pub fn duty_chng_end_ch7(&mut self) -> DutyChngEndChW<'_, IntRawSpec> {
        DutyChngEndChW::new(self, 11)
    }
    #[doc = "Interrupt raw bit for channel (0-7). Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OVF_CNT_CH0` field.</div>"]
    #[inline(always)]
    pub fn ovf_cnt_ch(&mut self, n: u8) -> OvfCntChW<'_, IntRawSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        OvfCntChW::new(self, n + 12)
    }
    #[doc = "Bit 12 - Interrupt raw bit for channel 0. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch0(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt raw bit for channel 1. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch1(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt raw bit for channel 2. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch2(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt raw bit for channel 3. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch3(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt raw bit for channel 4. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch4(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt raw bit for channel 5. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch5(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt raw bit for channel 6. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch6(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt raw bit for channel 7. Triggered when the OVF_CNT has reached the value specified by LEDC.CHx.CONF0.OVF_NUM."]
    #[inline(always)]
    pub fn ovf_cnt_ch7(&mut self) -> OvfCntChW<'_, IntRawSpec> {
        OvfCntChW::new(self, 19)
    }
}
#[doc = "Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
