#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `TIMER0_STOP` reader - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 0 stops."]
pub type Timer0StopR = crate::BitReader;
#[doc = "Field `TIMER1_STOP` reader - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 1 stops."]
pub type Timer1StopR = crate::BitReader;
#[doc = "Field `TIMER2_STOP` reader - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 2 stops."]
pub type Timer2StopR = crate::BitReader;
#[doc = "Field `TIMER0_TEZ` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
pub type Timer0TezR = crate::BitReader;
#[doc = "Field `TIMER1_TEZ` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
pub type Timer1TezR = crate::BitReader;
#[doc = "Field `TIMER2_TEZ` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
pub type Timer2TezR = crate::BitReader;
#[doc = "Field `TIMER0_TEP` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
pub type Timer0TepR = crate::BitReader;
#[doc = "Field `TIMER1_TEP` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
pub type Timer1TepR = crate::BitReader;
#[doc = "Field `TIMER2_TEP` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
pub type Timer2TepR = crate::BitReader;
#[doc = "Field `FAULT0` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 starts."]
pub type Fault0R = crate::BitReader;
#[doc = "Field `FAULT1` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 starts."]
pub type Fault1R = crate::BitReader;
#[doc = "Field `FAULT2` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 starts."]
pub type Fault2R = crate::BitReader;
#[doc = "Field `FAULT0_CLR` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 clears."]
pub type Fault0ClrR = crate::BitReader;
#[doc = "Field `FAULT1_CLR` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 clears."]
pub type Fault1ClrR = crate::BitReader;
#[doc = "Field `FAULT2_CLR` reader - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 clears."]
pub type Fault2ClrR = crate::BitReader;
#[doc = "Field `CMPR0_TEA` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
pub type Cmpr0TeaR = crate::BitReader;
#[doc = "Field `CMPR1_TEA` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
pub type Cmpr1TeaR = crate::BitReader;
#[doc = "Field `CMPR2_TEA` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
pub type Cmpr2TeaR = crate::BitReader;
#[doc = "Field `CMPR0_TEB` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
pub type Cmpr0TebR = crate::BitReader;
#[doc = "Field `CMPR1_TEB` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
pub type Cmpr1TebR = crate::BitReader;
#[doc = "Field `CMPR2_TEB` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
pub type Cmpr2TebR = crate::BitReader;
#[doc = "Field `TZ0_CBC` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
pub type Tz0CbcR = crate::BitReader;
#[doc = "Field `TZ1_CBC` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
pub type Tz1CbcR = crate::BitReader;
#[doc = "Field `TZ2_CBC` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
pub type Tz2CbcR = crate::BitReader;
#[doc = "Field `TZ0_OST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
pub type Tz0OstR = crate::BitReader;
#[doc = "Field `TZ1_OST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
pub type Tz1OstR = crate::BitReader;
#[doc = "Field `TZ2_OST` reader - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
pub type Tz2OstR = crate::BitReader;
#[doc = "Field `CAP0` reader - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP0."]
pub type Cap0R = crate::BitReader;
#[doc = "Field `CAP1` reader - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP1."]
pub type Cap1R = crate::BitReader;
#[doc = "Field `CAP2` reader - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP2."]
pub type Cap2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 0 stops."]
    #[inline(always)]
    pub fn timer0_stop(&self) -> Timer0StopR {
        Timer0StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 1 stops."]
    #[inline(always)]
    pub fn timer1_stop(&self) -> Timer1StopR {
        Timer1StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked status bit: The masked interrupt status of the interrupt triggered when the timer 2 stops."]
    #[inline(always)]
    pub fn timer2_stop(&self) -> Timer2StopR {
        Timer2StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEZ event."]
    #[inline(always)]
    pub fn timer0_tez(&self) -> Timer0TezR {
        Timer0TezR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEZ event."]
    #[inline(always)]
    pub fn timer1_tez(&self) -> Timer1TezR {
        Timer1TezR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEZ event."]
    #[inline(always)]
    pub fn timer2_tez(&self) -> Timer2TezR {
        Timer2TezR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 0 TEP event."]
    #[inline(always)]
    pub fn timer0_tep(&self) -> Timer0TepR {
        Timer0TepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 1 TEP event."]
    #[inline(always)]
    pub fn timer1_tep(&self) -> Timer1TepR {
        Timer1TepR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM timer 2 TEP event."]
    #[inline(always)]
    pub fn timer2_tep(&self) -> Timer2TepR {
        Timer2TepR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 starts."]
    #[inline(always)]
    pub fn fault0(&self) -> Fault0R {
        Fault0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 starts."]
    #[inline(always)]
    pub fn fault1(&self) -> Fault1R {
        Fault1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 starts."]
    #[inline(always)]
    pub fn fault2(&self) -> Fault2R {
        Fault2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f0 clears."]
    #[inline(always)]
    pub fn fault0_clr(&self) -> Fault0ClrR {
        Fault0ClrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f1 clears."]
    #[inline(always)]
    pub fn fault1_clr(&self) -> Fault1ClrR {
        Fault1ClrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Masked status bit: The masked interrupt status of the interrupt triggered when event_f2 clears."]
    #[inline(always)]
    pub fn fault2_clr(&self) -> Fault2ClrR {
        Fault2ClrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEA event"]
    #[inline(always)]
    pub fn cmpr0_tea(&self) -> Cmpr0TeaR {
        Cmpr0TeaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEA event"]
    #[inline(always)]
    pub fn cmpr1_tea(&self) -> Cmpr1TeaR {
        Cmpr1TeaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEA event"]
    #[inline(always)]
    pub fn cmpr2_tea(&self) -> Cmpr2TeaR {
        Cmpr2TeaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 0 TEB event"]
    #[inline(always)]
    pub fn cmpr0_teb(&self) -> Cmpr0TebR {
        Cmpr0TebR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 1 TEB event"]
    #[inline(always)]
    pub fn cmpr1_teb(&self) -> Cmpr1TebR {
        Cmpr1TebR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Masked status bit: The masked interrupt status of the interrupt triggered by a PWM operator 2 TEB event"]
    #[inline(always)]
    pub fn cmpr2_teb(&self) -> Cmpr2TebR {
        Cmpr2TebR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_cbc(&self) -> Tz0CbcR {
        Tz0CbcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_cbc(&self) -> Tz1CbcR {
        Tz1CbcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Masked status bit: The masked interrupt status of the interrupt triggered by a cycle-by-cycle mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_cbc(&self) -> Tz2CbcR {
        Tz2CbcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM0."]
    #[inline(always)]
    pub fn tz0_ost(&self) -> Tz0OstR {
        Tz0OstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM1."]
    #[inline(always)]
    pub fn tz1_ost(&self) -> Tz1OstR {
        Tz1OstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Masked status bit: The masked interrupt status of the interrupt triggered by a one-shot mode action on PWM2."]
    #[inline(always)]
    pub fn tz2_ost(&self) -> Tz2OstR {
        Tz2OstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP0."]
    #[inline(always)]
    pub fn cap0(&self) -> Cap0R {
        Cap0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP1."]
    #[inline(always)]
    pub fn cap1(&self) -> Cap1R {
        Cap1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Masked status bit: The masked interrupt status of the interrupt triggered by capture on CAP2."]
    #[inline(always)]
    pub fn cap2(&self) -> Cap2R {
        Cap2R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
