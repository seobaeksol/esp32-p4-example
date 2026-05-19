#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `TIMER0_STOP` reader - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 0 stops."]
pub type Timer0StopR = crate::BitReader;
#[doc = "Field `TIMER0_STOP` writer - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 0 stops."]
pub type Timer0StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_STOP` reader - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 1 stops."]
pub type Timer1StopR = crate::BitReader;
#[doc = "Field `TIMER1_STOP` writer - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 1 stops."]
pub type Timer1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_STOP` reader - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 2 stops."]
pub type Timer2StopR = crate::BitReader;
#[doc = "Field `TIMER2_STOP` writer - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 2 stops."]
pub type Timer2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEZ` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
pub type Timer0TezR = crate::BitReader;
#[doc = "Field `TIMER0_TEZ` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
pub type Timer0TezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEZ` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
pub type Timer1TezR = crate::BitReader;
#[doc = "Field `TIMER1_TEZ` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
pub type Timer1TezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEZ` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
pub type Timer2TezR = crate::BitReader;
#[doc = "Field `TIMER2_TEZ` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
pub type Timer2TezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEP` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
pub type Timer0TepR = crate::BitReader;
#[doc = "Field `TIMER0_TEP` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
pub type Timer0TepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEP` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
pub type Timer1TepR = crate::BitReader;
#[doc = "Field `TIMER1_TEP` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
pub type Timer1TepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEP` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
pub type Timer2TepR = crate::BitReader;
#[doc = "Field `TIMER2_TEP` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
pub type Timer2TepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0` reader - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 starts."]
pub type Fault0R = crate::BitReader;
#[doc = "Field `FAULT0` writer - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 starts."]
pub type Fault0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1` reader - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 starts."]
pub type Fault1R = crate::BitReader;
#[doc = "Field `FAULT1` writer - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 starts."]
pub type Fault1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT2` reader - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 starts."]
pub type Fault2R = crate::BitReader;
#[doc = "Field `FAULT2` writer - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 starts."]
pub type Fault2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT0_CLR` reader - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 clears."]
pub type Fault0ClrR = crate::BitReader;
#[doc = "Field `FAULT0_CLR` writer - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 clears."]
pub type Fault0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT1_CLR` reader - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 clears."]
pub type Fault1ClrR = crate::BitReader;
#[doc = "Field `FAULT1_CLR` writer - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 clears."]
pub type Fault1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT2_CLR` reader - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 clears."]
pub type Fault2ClrR = crate::BitReader;
#[doc = "Field `FAULT2_CLR` writer - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 clears."]
pub type Fault2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR0_TEA` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
pub type Cmpr0TeaR = crate::BitReader;
#[doc = "Field `CMPR0_TEA` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
pub type Cmpr0TeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR1_TEA` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
pub type Cmpr1TeaR = crate::BitReader;
#[doc = "Field `CMPR1_TEA` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
pub type Cmpr1TeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR2_TEA` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
pub type Cmpr2TeaR = crate::BitReader;
#[doc = "Field `CMPR2_TEA` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
pub type Cmpr2TeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR0_TEB` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
pub type Cmpr0TebR = crate::BitReader;
#[doc = "Field `CMPR0_TEB` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
pub type Cmpr0TebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR1_TEB` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
pub type Cmpr1TebR = crate::BitReader;
#[doc = "Field `CMPR1_TEB` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
pub type Cmpr1TebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR2_TEB` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
pub type Cmpr2TebR = crate::BitReader;
#[doc = "Field `CMPR2_TEB` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
pub type Cmpr2TebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ0_CBC` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type Tz0CbcR = crate::BitReader;
#[doc = "Field `TZ0_CBC` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type Tz0CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_CBC` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type Tz1CbcR = crate::BitReader;
#[doc = "Field `TZ1_CBC` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type Tz1CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_CBC` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type Tz2CbcR = crate::BitReader;
#[doc = "Field `TZ2_CBC` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type Tz2CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ0_OST` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
pub type Tz0OstR = crate::BitReader;
#[doc = "Field `TZ0_OST` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
pub type Tz0OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_OST` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
pub type Tz1OstR = crate::BitReader;
#[doc = "Field `TZ1_OST` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
pub type Tz1OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_OST` reader - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
pub type Tz2OstR = crate::BitReader;
#[doc = "Field `TZ2_OST` writer - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
pub type Tz2OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0` reader - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP0."]
pub type Cap0R = crate::BitReader;
#[doc = "Field `CAP0` writer - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP0."]
pub type Cap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1` reader - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP1."]
pub type Cap1R = crate::BitReader;
#[doc = "Field `CAP1` writer - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP1."]
pub type Cap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2` reader - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP2."]
pub type Cap2R = crate::BitReader;
#[doc = "Field `CAP2` writer - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP2."]
pub type Cap2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    pub fn timer0_stop(&self) -> Timer0StopR {
        Timer0StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    pub fn timer1_stop(&self) -> Timer1StopR {
        Timer1StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    pub fn timer2_stop(&self) -> Timer2StopR {
        Timer2StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    pub fn timer0_tez(&self) -> Timer0TezR {
        Timer0TezR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    pub fn timer1_tez(&self) -> Timer1TezR {
        Timer1TezR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    pub fn timer2_tez(&self) -> Timer2TezR {
        Timer2TezR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    pub fn timer0_tep(&self) -> Timer0TepR {
        Timer0TepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    pub fn timer1_tep(&self) -> Timer1TepR {
        Timer1TepR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    pub fn timer2_tep(&self) -> Timer2TepR {
        Timer2TepR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    pub fn fault0(&self) -> Fault0R {
        Fault0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    pub fn fault1(&self) -> Fault1R {
        Fault1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    pub fn fault2(&self) -> Fault2R {
        Fault2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 clears."]
    #[inline(always)]
    pub fn fault0_clr(&self) -> Fault0ClrR {
        Fault0ClrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 clears."]
    #[inline(always)]
    pub fn fault1_clr(&self) -> Fault1ClrR {
        Fault1ClrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 clears."]
    #[inline(always)]
    pub fn fault2_clr(&self) -> Fault2ClrR {
        Fault2ClrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn cmpr0_tea(&self) -> Cmpr0TeaR {
        Cmpr0TeaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn cmpr1_tea(&self) -> Cmpr1TeaR {
        Cmpr1TeaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn cmpr2_tea(&self) -> Cmpr2TeaR {
        Cmpr2TeaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn cmpr0_teb(&self) -> Cmpr0TebR {
        Cmpr0TebR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn cmpr1_teb(&self) -> Cmpr1TebR {
        Cmpr1TebR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn cmpr2_teb(&self) -> Cmpr2TebR {
        Cmpr2TebR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_cbc(&self) -> Tz0CbcR {
        Tz0CbcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_cbc(&self) -> Tz1CbcR {
        Tz1CbcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_cbc(&self) -> Tz2CbcR {
        Tz2CbcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_ost(&self) -> Tz0OstR {
        Tz0OstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_ost(&self) -> Tz1OstR {
        Tz1OstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_ost(&self) -> Tz2OstR {
        Tz2OstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP0."]
    #[inline(always)]
    pub fn cap0(&self) -> Cap0R {
        Cap0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP1."]
    #[inline(always)]
    pub fn cap1(&self) -> Cap1R {
        Cap1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP2."]
    #[inline(always)]
    pub fn cap2(&self) -> Cap2R {
        Cap2R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    pub fn timer0_stop(&mut self) -> Timer0StopW<'_, IntRawSpec> {
        Timer0StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    pub fn timer1_stop(&mut self) -> Timer1StopW<'_, IntRawSpec> {
        Timer1StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Raw status bit: The raw interrupt status of the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    pub fn timer2_stop(&mut self) -> Timer2StopW<'_, IntRawSpec> {
        Timer2StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    pub fn timer0_tez(&mut self) -> Timer0TezW<'_, IntRawSpec> {
        Timer0TezW::new(self, 3)
    }
    #[doc = "Bit 4 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    pub fn timer1_tez(&mut self) -> Timer1TezW<'_, IntRawSpec> {
        Timer1TezW::new(self, 4)
    }
    #[doc = "Bit 5 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    pub fn timer2_tez(&mut self) -> Timer2TezW<'_, IntRawSpec> {
        Timer2TezW::new(self, 5)
    }
    #[doc = "Bit 6 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    pub fn timer0_tep(&mut self) -> Timer0TepW<'_, IntRawSpec> {
        Timer0TepW::new(self, 6)
    }
    #[doc = "Bit 7 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    pub fn timer1_tep(&mut self) -> Timer1TepW<'_, IntRawSpec> {
        Timer1TepW::new(self, 7)
    }
    #[doc = "Bit 8 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    pub fn timer2_tep(&mut self) -> Timer2TepW<'_, IntRawSpec> {
        Timer2TepW::new(self, 8)
    }
    #[doc = "Bit 9 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    pub fn fault0(&mut self) -> Fault0W<'_, IntRawSpec> {
        Fault0W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    pub fn fault1(&mut self) -> Fault1W<'_, IntRawSpec> {
        Fault1W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    pub fn fault2(&mut self) -> Fault2W<'_, IntRawSpec> {
        Fault2W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f0 clears."]
    #[inline(always)]
    pub fn fault0_clr(&mut self) -> Fault0ClrW<'_, IntRawSpec> {
        Fault0ClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f1 clears."]
    #[inline(always)]
    pub fn fault1_clr(&mut self) -> Fault1ClrW<'_, IntRawSpec> {
        Fault1ClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Raw status bit: The raw interrupt status of the interrupt triggered when event_f2 clears."]
    #[inline(always)]
    pub fn fault2_clr(&mut self) -> Fault2ClrW<'_, IntRawSpec> {
        Fault2ClrW::new(self, 14)
    }
    #[doc = "Bit 15 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn cmpr0_tea(&mut self) -> Cmpr0TeaW<'_, IntRawSpec> {
        Cmpr0TeaW::new(self, 15)
    }
    #[doc = "Bit 16 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn cmpr1_tea(&mut self) -> Cmpr1TeaW<'_, IntRawSpec> {
        Cmpr1TeaW::new(self, 16)
    }
    #[doc = "Bit 17 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn cmpr2_tea(&mut self) -> Cmpr2TeaW<'_, IntRawSpec> {
        Cmpr2TeaW::new(self, 17)
    }
    #[doc = "Bit 18 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn cmpr0_teb(&mut self) -> Cmpr0TebW<'_, IntRawSpec> {
        Cmpr0TebW::new(self, 18)
    }
    #[doc = "Bit 19 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn cmpr1_teb(&mut self) -> Cmpr1TebW<'_, IntRawSpec> {
        Cmpr1TebW::new(self, 19)
    }
    #[doc = "Bit 20 - Raw status bit: The raw interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn cmpr2_teb(&mut self) -> Cmpr2TebW<'_, IntRawSpec> {
        Cmpr2TebW::new(self, 20)
    }
    #[doc = "Bit 21 - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_cbc(&mut self) -> Tz0CbcW<'_, IntRawSpec> {
        Tz0CbcW::new(self, 21)
    }
    #[doc = "Bit 22 - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_cbc(&mut self) -> Tz1CbcW<'_, IntRawSpec> {
        Tz1CbcW::new(self, 22)
    }
    #[doc = "Bit 23 - Raw status bit: The raw interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_cbc(&mut self) -> Tz2CbcW<'_, IntRawSpec> {
        Tz2CbcW::new(self, 23)
    }
    #[doc = "Bit 24 - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_ost(&mut self) -> Tz0OstW<'_, IntRawSpec> {
        Tz0OstW::new(self, 24)
    }
    #[doc = "Bit 25 - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_ost(&mut self) -> Tz1OstW<'_, IntRawSpec> {
        Tz1OstW::new(self, 25)
    }
    #[doc = "Bit 26 - Raw status bit: The raw interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_ost(&mut self) -> Tz2OstW<'_, IntRawSpec> {
        Tz2OstW::new(self, 26)
    }
    #[doc = "Bit 27 - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP0."]
    #[inline(always)]
    pub fn cap0(&mut self) -> Cap0W<'_, IntRawSpec> {
        Cap0W::new(self, 27)
    }
    #[doc = "Bit 28 - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP1."]
    #[inline(always)]
    pub fn cap1(&mut self) -> Cap1W<'_, IntRawSpec> {
        Cap1W::new(self, 28)
    }
    #[doc = "Bit 29 - Raw status bit: The raw interrupt status of the interrupt triggered by capture on CAP2."]
    #[inline(always)]
    pub fn cap2(&mut self) -> Cap2W<'_, IntRawSpec> {
        Cap2W::new(self, 29)
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
