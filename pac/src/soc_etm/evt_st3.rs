#[doc = "Register `EVT_ST3` reader"]
pub type R = crate::R<EvtSt3Spec>;
#[doc = "Register `EVT_ST3` writer"]
pub type W = crate::W<EvtSt3Spec>;
#[doc = "Field `MCPWM1_EVT_TIMER1_STOP_ST` reader - Represents MCPWM1_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer1StopStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER1_STOP_ST` writer - Represents MCPWM1_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer1StopStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TIMER2_STOP_ST` reader - Represents MCPWM1_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer2StopStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER2_STOP_ST` writer - Represents MCPWM1_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer2StopStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TIMER0_TEZ_ST` reader - Represents MCPWM1_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer0TezStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER0_TEZ_ST` writer - Represents MCPWM1_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer0TezStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TIMER1_TEZ_ST` reader - Represents MCPWM1_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer1TezStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER1_TEZ_ST` writer - Represents MCPWM1_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer1TezStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TIMER2_TEZ_ST` reader - Represents MCPWM1_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer2TezStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER2_TEZ_ST` writer - Represents MCPWM1_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer2TezStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TIMER0_TEP_ST` reader - Represents MCPWM1_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer0TepStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER0_TEP_ST` writer - Represents MCPWM1_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer0TepStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TIMER1_TEP_ST` reader - Represents MCPWM1_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer1TepStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER1_TEP_ST` writer - Represents MCPWM1_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer1TepStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TIMER2_TEP_ST` reader - Represents MCPWM1_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer2TepStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TIMER2_TEP_ST` writer - Represents MCPWM1_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTimer2TepStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP0_TEA_ST` reader - Represents MCPWM1_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0TeaStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP0_TEA_ST` writer - Represents MCPWM1_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0TeaStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP1_TEA_ST` reader - Represents MCPWM1_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1TeaStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP1_TEA_ST` writer - Represents MCPWM1_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1TeaStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP2_TEA_ST` reader - Represents MCPWM1_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2TeaStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP2_TEA_ST` writer - Represents MCPWM1_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2TeaStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP0_TEB_ST` reader - Represents MCPWM1_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0TebStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP0_TEB_ST` writer - Represents MCPWM1_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0TebStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP1_TEB_ST` reader - Represents MCPWM1_evt_op1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1TebStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP1_TEB_ST` writer - Represents MCPWM1_evt_op1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1TebStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP2_TEB_ST` reader - Represents MCPWM1_evt_op2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2TebStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP2_TEB_ST` writer - Represents MCPWM1_evt_op2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2TebStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_F0_ST` reader - Represents MCPWM1_evt_f0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF0StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_F0_ST` writer - Represents MCPWM1_evt_f0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_F1_ST` reader - Represents MCPWM1_evt_f1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF1StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_F1_ST` writer - Represents MCPWM1_evt_f1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_F2_ST` reader - Represents MCPWM1_evt_f2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF2StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_F2_ST` writer - Represents MCPWM1_evt_f2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_F0_CLR_ST` reader - Represents MCPWM1_evt_f0_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF0ClrStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_F0_CLR_ST` writer - Represents MCPWM1_evt_f0_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF0ClrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_F1_CLR_ST` reader - Represents MCPWM1_evt_f1_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF1ClrStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_F1_CLR_ST` writer - Represents MCPWM1_evt_f1_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF1ClrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_F2_CLR_ST` reader - Represents MCPWM1_evt_f2_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF2ClrStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_F2_CLR_ST` writer - Represents MCPWM1_evt_f2_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtF2ClrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TZ0_CBC_ST` reader - Represents MCPWM1_evt_tz0_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz0CbcStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TZ0_CBC_ST` writer - Represents MCPWM1_evt_tz0_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz0CbcStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TZ1_CBC_ST` reader - Represents MCPWM1_evt_tz1_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz1CbcStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TZ1_CBC_ST` writer - Represents MCPWM1_evt_tz1_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz1CbcStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TZ2_CBC_ST` reader - Represents MCPWM1_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz2CbcStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TZ2_CBC_ST` writer - Represents MCPWM1_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz2CbcStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TZ0_OST_ST` reader - Represents MCPWM1_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz0OstStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TZ0_OST_ST` writer - Represents MCPWM1_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz0OstStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TZ1_OST_ST` reader - Represents MCPWM1_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz1OstStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TZ1_OST_ST` writer - Represents MCPWM1_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz1OstStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_TZ2_OST_ST` reader - Represents MCPWM1_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz2OstStR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_TZ2_OST_ST` writer - Represents MCPWM1_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtTz2OstStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_CAP0_ST` reader - Represents MCPWM1_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtCap0StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_CAP0_ST` writer - Represents MCPWM1_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtCap0StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_CAP1_ST` reader - Represents MCPWM1_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtCap1StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_CAP1_ST` writer - Represents MCPWM1_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtCap1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_CAP2_ST` reader - Represents MCPWM1_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtCap2StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_CAP2_ST` writer - Represents MCPWM1_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtCap2StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP0_TEE1_ST` reader - Represents MCPWM1_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0Tee1StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP0_TEE1_ST` writer - Represents MCPWM1_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp0Tee1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP1_TEE1_ST` reader - Represents MCPWM1_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1Tee1StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP1_TEE1_ST` writer - Represents MCPWM1_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp1Tee1StW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP2_TEE1_ST` reader - Represents MCPWM1_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2Tee1StR = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP2_TEE1_ST` writer - Represents MCPWM1_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type Mcpwm1EvtOp2Tee1StW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents MCPWM1_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer1_stop_st(&self) -> Mcpwm1EvtTimer1StopStR {
        Mcpwm1EvtTimer1StopStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents MCPWM1_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer2_stop_st(&self) -> Mcpwm1EvtTimer2StopStR {
        Mcpwm1EvtTimer2StopStR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents MCPWM1_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer0_tez_st(&self) -> Mcpwm1EvtTimer0TezStR {
        Mcpwm1EvtTimer0TezStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents MCPWM1_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer1_tez_st(&self) -> Mcpwm1EvtTimer1TezStR {
        Mcpwm1EvtTimer1TezStR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents MCPWM1_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer2_tez_st(&self) -> Mcpwm1EvtTimer2TezStR {
        Mcpwm1EvtTimer2TezStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents MCPWM1_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer0_tep_st(&self) -> Mcpwm1EvtTimer0TepStR {
        Mcpwm1EvtTimer0TepStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents MCPWM1_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer1_tep_st(&self) -> Mcpwm1EvtTimer1TepStR {
        Mcpwm1EvtTimer1TepStR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents MCPWM1_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer2_tep_st(&self) -> Mcpwm1EvtTimer2TepStR {
        Mcpwm1EvtTimer2TepStR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents MCPWM1_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_tea_st(&self) -> Mcpwm1EvtOp0TeaStR {
        Mcpwm1EvtOp0TeaStR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents MCPWM1_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_tea_st(&self) -> Mcpwm1EvtOp1TeaStR {
        Mcpwm1EvtOp1TeaStR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents MCPWM1_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_tea_st(&self) -> Mcpwm1EvtOp2TeaStR {
        Mcpwm1EvtOp2TeaStR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents MCPWM1_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_teb_st(&self) -> Mcpwm1EvtOp0TebStR {
        Mcpwm1EvtOp0TebStR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents MCPWM1_evt_op1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_teb_st(&self) -> Mcpwm1EvtOp1TebStR {
        Mcpwm1EvtOp1TebStR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents MCPWM1_evt_op2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_teb_st(&self) -> Mcpwm1EvtOp2TebStR {
        Mcpwm1EvtOp2TebStR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents MCPWM1_evt_f0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f0_st(&self) -> Mcpwm1EvtF0StR {
        Mcpwm1EvtF0StR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents MCPWM1_evt_f1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f1_st(&self) -> Mcpwm1EvtF1StR {
        Mcpwm1EvtF1StR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents MCPWM1_evt_f2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f2_st(&self) -> Mcpwm1EvtF2StR {
        Mcpwm1EvtF2StR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents MCPWM1_evt_f0_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f0_clr_st(&self) -> Mcpwm1EvtF0ClrStR {
        Mcpwm1EvtF0ClrStR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents MCPWM1_evt_f1_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f1_clr_st(&self) -> Mcpwm1EvtF1ClrStR {
        Mcpwm1EvtF1ClrStR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents MCPWM1_evt_f2_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f2_clr_st(&self) -> Mcpwm1EvtF2ClrStR {
        Mcpwm1EvtF2ClrStR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents MCPWM1_evt_tz0_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz0_cbc_st(&self) -> Mcpwm1EvtTz0CbcStR {
        Mcpwm1EvtTz0CbcStR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents MCPWM1_evt_tz1_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz1_cbc_st(&self) -> Mcpwm1EvtTz1CbcStR {
        Mcpwm1EvtTz1CbcStR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents MCPWM1_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz2_cbc_st(&self) -> Mcpwm1EvtTz2CbcStR {
        Mcpwm1EvtTz2CbcStR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents MCPWM1_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz0_ost_st(&self) -> Mcpwm1EvtTz0OstStR {
        Mcpwm1EvtTz0OstStR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents MCPWM1_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz1_ost_st(&self) -> Mcpwm1EvtTz1OstStR {
        Mcpwm1EvtTz1OstStR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents MCPWM1_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz2_ost_st(&self) -> Mcpwm1EvtTz2OstStR {
        Mcpwm1EvtTz2OstStR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents MCPWM1_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_cap0_st(&self) -> Mcpwm1EvtCap0StR {
        Mcpwm1EvtCap0StR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents MCPWM1_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_cap1_st(&self) -> Mcpwm1EvtCap1StR {
        Mcpwm1EvtCap1StR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents MCPWM1_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_cap2_st(&self) -> Mcpwm1EvtCap2StR {
        Mcpwm1EvtCap2StR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents MCPWM1_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_tee1_st(&self) -> Mcpwm1EvtOp0Tee1StR {
        Mcpwm1EvtOp0Tee1StR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents MCPWM1_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_tee1_st(&self) -> Mcpwm1EvtOp1Tee1StR {
        Mcpwm1EvtOp1Tee1StR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents MCPWM1_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_tee1_st(&self) -> Mcpwm1EvtOp2Tee1StR {
        Mcpwm1EvtOp2Tee1StR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Represents MCPWM1_evt_timer1_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer1_stop_st(&mut self) -> Mcpwm1EvtTimer1StopStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer1StopStW::new(self, 0)
    }
    #[doc = "Bit 1 - Represents MCPWM1_evt_timer2_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer2_stop_st(&mut self) -> Mcpwm1EvtTimer2StopStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer2StopStW::new(self, 1)
    }
    #[doc = "Bit 2 - Represents MCPWM1_evt_timer0_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer0_tez_st(&mut self) -> Mcpwm1EvtTimer0TezStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer0TezStW::new(self, 2)
    }
    #[doc = "Bit 3 - Represents MCPWM1_evt_timer1_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer1_tez_st(&mut self) -> Mcpwm1EvtTimer1TezStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer1TezStW::new(self, 3)
    }
    #[doc = "Bit 4 - Represents MCPWM1_evt_timer2_tez trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer2_tez_st(&mut self) -> Mcpwm1EvtTimer2TezStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer2TezStW::new(self, 4)
    }
    #[doc = "Bit 5 - Represents MCPWM1_evt_timer0_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer0_tep_st(&mut self) -> Mcpwm1EvtTimer0TepStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer0TepStW::new(self, 5)
    }
    #[doc = "Bit 6 - Represents MCPWM1_evt_timer1_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer1_tep_st(&mut self) -> Mcpwm1EvtTimer1TepStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer1TepStW::new(self, 6)
    }
    #[doc = "Bit 7 - Represents MCPWM1_evt_timer2_tep trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_timer2_tep_st(&mut self) -> Mcpwm1EvtTimer2TepStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTimer2TepStW::new(self, 7)
    }
    #[doc = "Bit 8 - Represents MCPWM1_evt_op0_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_tea_st(&mut self) -> Mcpwm1EvtOp0TeaStW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp0TeaStW::new(self, 8)
    }
    #[doc = "Bit 9 - Represents MCPWM1_evt_op1_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_tea_st(&mut self) -> Mcpwm1EvtOp1TeaStW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp1TeaStW::new(self, 9)
    }
    #[doc = "Bit 10 - Represents MCPWM1_evt_op2_tea trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_tea_st(&mut self) -> Mcpwm1EvtOp2TeaStW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp2TeaStW::new(self, 10)
    }
    #[doc = "Bit 11 - Represents MCPWM1_evt_op0_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_teb_st(&mut self) -> Mcpwm1EvtOp0TebStW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp0TebStW::new(self, 11)
    }
    #[doc = "Bit 12 - Represents MCPWM1_evt_op1_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_teb_st(&mut self) -> Mcpwm1EvtOp1TebStW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp1TebStW::new(self, 12)
    }
    #[doc = "Bit 13 - Represents MCPWM1_evt_op2_teb trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_teb_st(&mut self) -> Mcpwm1EvtOp2TebStW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp2TebStW::new(self, 13)
    }
    #[doc = "Bit 14 - Represents MCPWM1_evt_f0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f0_st(&mut self) -> Mcpwm1EvtF0StW<'_, EvtSt3Spec> {
        Mcpwm1EvtF0StW::new(self, 14)
    }
    #[doc = "Bit 15 - Represents MCPWM1_evt_f1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f1_st(&mut self) -> Mcpwm1EvtF1StW<'_, EvtSt3Spec> {
        Mcpwm1EvtF1StW::new(self, 15)
    }
    #[doc = "Bit 16 - Represents MCPWM1_evt_f2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f2_st(&mut self) -> Mcpwm1EvtF2StW<'_, EvtSt3Spec> {
        Mcpwm1EvtF2StW::new(self, 16)
    }
    #[doc = "Bit 17 - Represents MCPWM1_evt_f0_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f0_clr_st(&mut self) -> Mcpwm1EvtF0ClrStW<'_, EvtSt3Spec> {
        Mcpwm1EvtF0ClrStW::new(self, 17)
    }
    #[doc = "Bit 18 - Represents MCPWM1_evt_f1_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f1_clr_st(&mut self) -> Mcpwm1EvtF1ClrStW<'_, EvtSt3Spec> {
        Mcpwm1EvtF1ClrStW::new(self, 18)
    }
    #[doc = "Bit 19 - Represents MCPWM1_evt_f2_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_f2_clr_st(&mut self) -> Mcpwm1EvtF2ClrStW<'_, EvtSt3Spec> {
        Mcpwm1EvtF2ClrStW::new(self, 19)
    }
    #[doc = "Bit 20 - Represents MCPWM1_evt_tz0_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz0_cbc_st(&mut self) -> Mcpwm1EvtTz0CbcStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTz0CbcStW::new(self, 20)
    }
    #[doc = "Bit 21 - Represents MCPWM1_evt_tz1_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz1_cbc_st(&mut self) -> Mcpwm1EvtTz1CbcStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTz1CbcStW::new(self, 21)
    }
    #[doc = "Bit 22 - Represents MCPWM1_evt_tz2_cbc trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz2_cbc_st(&mut self) -> Mcpwm1EvtTz2CbcStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTz2CbcStW::new(self, 22)
    }
    #[doc = "Bit 23 - Represents MCPWM1_evt_tz0_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz0_ost_st(&mut self) -> Mcpwm1EvtTz0OstStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTz0OstStW::new(self, 23)
    }
    #[doc = "Bit 24 - Represents MCPWM1_evt_tz1_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz1_ost_st(&mut self) -> Mcpwm1EvtTz1OstStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTz1OstStW::new(self, 24)
    }
    #[doc = "Bit 25 - Represents MCPWM1_evt_tz2_ost trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_tz2_ost_st(&mut self) -> Mcpwm1EvtTz2OstStW<'_, EvtSt3Spec> {
        Mcpwm1EvtTz2OstStW::new(self, 25)
    }
    #[doc = "Bit 26 - Represents MCPWM1_evt_cap0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_cap0_st(&mut self) -> Mcpwm1EvtCap0StW<'_, EvtSt3Spec> {
        Mcpwm1EvtCap0StW::new(self, 26)
    }
    #[doc = "Bit 27 - Represents MCPWM1_evt_cap1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_cap1_st(&mut self) -> Mcpwm1EvtCap1StW<'_, EvtSt3Spec> {
        Mcpwm1EvtCap1StW::new(self, 27)
    }
    #[doc = "Bit 28 - Represents MCPWM1_evt_cap2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_cap2_st(&mut self) -> Mcpwm1EvtCap2StW<'_, EvtSt3Spec> {
        Mcpwm1EvtCap2StW::new(self, 28)
    }
    #[doc = "Bit 29 - Represents MCPWM1_evt_op0_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_tee1_st(&mut self) -> Mcpwm1EvtOp0Tee1StW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp0Tee1StW::new(self, 29)
    }
    #[doc = "Bit 30 - Represents MCPWM1_evt_op1_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_tee1_st(&mut self) -> Mcpwm1EvtOp1Tee1StW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp1Tee1StW::new(self, 30)
    }
    #[doc = "Bit 31 - Represents MCPWM1_evt_op2_tee1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_tee1_st(&mut self) -> Mcpwm1EvtOp2Tee1StW<'_, EvtSt3Spec> {
        Mcpwm1EvtOp2Tee1StW::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtSt3Spec;
impl crate::RegisterSpec for EvtSt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st3::R`](R) reader structure"]
impl crate::Readable for EvtSt3Spec {}
#[doc = "`write(|w| ..)` method takes [`evt_st3::W`](W) writer structure"]
impl crate::Writable for EvtSt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST3 to value 0"]
impl crate::Resettable for EvtSt3Spec {}
