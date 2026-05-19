#[doc = "Register `EVT_EN` reader"]
pub type R = crate::R<EvtEnSpec>;
#[doc = "Register `EVT_EN` writer"]
pub type W = crate::W<EvtEnSpec>;
#[doc = "Field `TIMER0_STOP` reader - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer0StopR = crate::BitReader;
#[doc = "Field `TIMER0_STOP` writer - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer0StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_STOP` reader - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer1StopR = crate::BitReader;
#[doc = "Field `TIMER1_STOP` writer - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_STOP` reader - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer2StopR = crate::BitReader;
#[doc = "Field `TIMER2_STOP` writer - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer2StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEZ` reader - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer0TezR = crate::BitReader;
#[doc = "Field `TIMER0_TEZ` writer - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer0TezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEZ` reader - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer1TezR = crate::BitReader;
#[doc = "Field `TIMER1_TEZ` writer - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer1TezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEZ` reader - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer2TezR = crate::BitReader;
#[doc = "Field `TIMER2_TEZ` writer - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer2TezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0_TEP` reader - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer0TepR = crate::BitReader;
#[doc = "Field `TIMER0_TEP` writer - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer0TepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1_TEP` reader - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer1TepR = crate::BitReader;
#[doc = "Field `TIMER1_TEP` writer - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer1TepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2_TEP` reader - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer2TepR = crate::BitReader;
#[doc = "Field `TIMER2_TEP` writer - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
pub type Timer2TepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEA` reader - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0TeaR = crate::BitReader;
#[doc = "Field `OP0_TEA` writer - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0TeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEA` reader - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1TeaR = crate::BitReader;
#[doc = "Field `OP1_TEA` writer - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1TeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEA` reader - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2TeaR = crate::BitReader;
#[doc = "Field `OP2_TEA` writer - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2TeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0_TEB` reader - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0TebR = crate::BitReader;
#[doc = "Field `OP0_TEB` writer - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op0TebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1_TEB` reader - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1TebR = crate::BitReader;
#[doc = "Field `OP1_TEB` writer - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op1TebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2_TEB` reader - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2TebR = crate::BitReader;
#[doc = "Field `OP2_TEB` writer - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
pub type Op2TebW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0` reader - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0R = crate::BitReader;
#[doc = "Field `F0` writer - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1` reader - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1R = crate::BitReader;
#[doc = "Field `F1` writer - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F2` reader - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2R = crate::BitReader;
#[doc = "Field `F2` writer - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F0_CLR` reader - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0ClrR = crate::BitReader;
#[doc = "Field `F0_CLR` writer - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1_CLR` reader - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1ClrR = crate::BitReader;
#[doc = "Field `F1_CLR` writer - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F2_CLR` reader - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2ClrR = crate::BitReader;
#[doc = "Field `F2_CLR` writer - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
pub type F2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ0_CBC` reader - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz0CbcR = crate::BitReader;
#[doc = "Field `TZ0_CBC` writer - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz0CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_CBC` reader - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz1CbcR = crate::BitReader;
#[doc = "Field `TZ1_CBC` writer - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz1CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_CBC` reader - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz2CbcR = crate::BitReader;
#[doc = "Field `TZ2_CBC` writer - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz2CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ0_OST` reader - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz0OstR = crate::BitReader;
#[doc = "Field `TZ0_OST` writer - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz0OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_OST` reader - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz1OstR = crate::BitReader;
#[doc = "Field `TZ1_OST` writer - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz1OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_OST` reader - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz2OstR = crate::BitReader;
#[doc = "Field `TZ2_OST` writer - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Tz2OstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0` reader - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Cap0R = crate::BitReader;
#[doc = "Field `CAP0` writer - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Cap0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1` reader - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Cap1R = crate::BitReader;
#[doc = "Field `CAP1` writer - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Cap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2` reader - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Cap2R = crate::BitReader;
#[doc = "Field `CAP2` writer - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
pub type Cap2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_stop(&self) -> Timer0StopR {
        Timer0StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_stop(&self) -> Timer1StopR {
        Timer1StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_stop(&self) -> Timer2StopR {
        Timer2StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tez(&self) -> Timer0TezR {
        Timer0TezR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tez(&self) -> Timer1TezR {
        Timer1TezR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tez(&self) -> Timer2TezR {
        Timer2TezR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tep(&self) -> Timer0TepR {
        Timer0TepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tep(&self) -> Timer1TepR {
        Timer1TepR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tep(&self) -> Timer2TepR {
        Timer2TepR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tea(&self) -> Op0TeaR {
        Op0TeaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tea(&self) -> Op1TeaR {
        Op1TeaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tea(&self) -> Op2TeaR {
        Op2TeaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_teb(&self) -> Op0TebR {
        Op0TebR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_teb(&self) -> Op1TebR {
        Op1TebR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_teb(&self) -> Op2TebR {
        Op2TebR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0(&self) -> F0R {
        F0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1(&self) -> F1R {
        F1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2(&self) -> F2R {
        F2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0_clr(&self) -> F0ClrR {
        F0ClrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1_clr(&self) -> F1ClrR {
        F1ClrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2_clr(&self) -> F2ClrR {
        F2ClrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_cbc(&self) -> Tz0CbcR {
        Tz0CbcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_cbc(&self) -> Tz1CbcR {
        Tz1CbcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_cbc(&self) -> Tz2CbcR {
        Tz2CbcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_ost(&self) -> Tz0OstR {
        Tz0OstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_ost(&self) -> Tz1OstR {
        Tz1OstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_ost(&self) -> Tz2OstR {
        Tz2OstR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap0(&self) -> Cap0R {
        Cap0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap1(&self) -> Cap1R {
        Cap1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap2(&self) -> Cap2R {
        Cap2R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable timer0 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_stop(&mut self) -> Timer0StopW<'_, EvtEnSpec> {
        Timer0StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable timer1 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_stop(&mut self) -> Timer1StopW<'_, EvtEnSpec> {
        Timer1StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable timer2 stop event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_stop(&mut self) -> Timer2StopW<'_, EvtEnSpec> {
        Timer2StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable timer0 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tez(&mut self) -> Timer0TezW<'_, EvtEnSpec> {
        Timer0TezW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable timer1 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tez(&mut self) -> Timer1TezW<'_, EvtEnSpec> {
        Timer1TezW::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable timer2 equal zero event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tez(&mut self) -> Timer2TezW<'_, EvtEnSpec> {
        Timer2TezW::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable timer0 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer0_tep(&mut self) -> Timer0TepW<'_, EvtEnSpec> {
        Timer0TepW::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable timer1 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer1_tep(&mut self) -> Timer1TepW<'_, EvtEnSpec> {
        Timer1TepW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable timer2 equal period event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn timer2_tep(&mut self) -> Timer2TepW<'_, EvtEnSpec> {
        Timer2TepW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable PWM generator0 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_tea(&mut self) -> Op0TeaW<'_, EvtEnSpec> {
        Op0TeaW::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable PWM generator1 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_tea(&mut self) -> Op1TeaW<'_, EvtEnSpec> {
        Op1TeaW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable PWM generator2 timer equal a event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_tea(&mut self) -> Op2TeaW<'_, EvtEnSpec> {
        Op2TeaW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable PWM generator0 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op0_teb(&mut self) -> Op0TebW<'_, EvtEnSpec> {
        Op0TebW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable PWM generator1 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op1_teb(&mut self) -> Op1TebW<'_, EvtEnSpec> {
        Op1TebW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable PWM generator2 timer equal b event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn op2_teb(&mut self) -> Op2TebW<'_, EvtEnSpec> {
        Op2TebW::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable fault0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0(&mut self) -> F0W<'_, EvtEnSpec> {
        F0W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable fault1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1(&mut self) -> F1W<'_, EvtEnSpec> {
        F1W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable fault2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2(&mut self) -> F2W<'_, EvtEnSpec> {
        F2W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to enable fault0 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f0_clr(&mut self) -> F0ClrW<'_, EvtEnSpec> {
        F0ClrW::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to enable fault1 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f1_clr(&mut self) -> F1ClrW<'_, EvtEnSpec> {
        F1ClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to enable fault2 clear event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn f2_clr(&mut self) -> F2ClrW<'_, EvtEnSpec> {
        F2ClrW::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to enable cycle-by-cycle trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_cbc(&mut self) -> Tz0CbcW<'_, EvtEnSpec> {
        Tz0CbcW::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to enable cycle-by-cycle trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_cbc(&mut self) -> Tz1CbcW<'_, EvtEnSpec> {
        Tz1CbcW::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to enable cycle-by-cycle trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_cbc(&mut self) -> Tz2CbcW<'_, EvtEnSpec> {
        Tz2CbcW::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to enable one-shot trip0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz0_ost(&mut self) -> Tz0OstW<'_, EvtEnSpec> {
        Tz0OstW::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to enable one-shot trip1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz1_ost(&mut self) -> Tz1OstW<'_, EvtEnSpec> {
        Tz1OstW::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to enable one-shot trip2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn tz2_ost(&mut self) -> Tz2OstW<'_, EvtEnSpec> {
        Tz2OstW::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to enable capture0 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap0(&mut self) -> Cap0W<'_, EvtEnSpec> {
        Cap0W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable capture1 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap1(&mut self) -> Cap1W<'_, EvtEnSpec> {
        Cap1W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to enable capture2 event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap2(&mut self) -> Cap2W<'_, EvtEnSpec> {
        Cap2W::new(self, 29)
    }
}
#[doc = "Event enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtEnSpec;
impl crate::RegisterSpec for EvtEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_en::R`](R) reader structure"]
impl crate::Readable for EvtEnSpec {}
#[doc = "`write(|w| ..)` method takes [`evt_en::W`](W) writer structure"]
impl crate::Writable for EvtEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_EN to value 0"]
impl crate::Resettable for EvtEnSpec {}
