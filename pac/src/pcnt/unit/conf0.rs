#[doc = "Register `CONF0` reader"]
pub type R = crate::R<Conf0Spec>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<Conf0Spec>;
#[doc = "Field `FILTER_THRES` reader - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
pub type FilterThresR = crate::FieldReader<u16>;
#[doc = "Field `FILTER_THRES` writer - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
pub type FilterThresW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FILTER_EN` reader - This is the enable bit for unit %s's input filter."]
pub type FilterEnR = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - This is the enable bit for unit %s's input filter."]
pub type FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_ZERO_EN` reader - This is the enable bit for unit %s's zero comparator."]
pub type ThrZeroEnR = crate::BitReader;
#[doc = "Field `THR_ZERO_EN` writer - This is the enable bit for unit %s's zero comparator."]
pub type ThrZeroEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_H_LIM_EN` reader - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
pub type ThrHLimEnR = crate::BitReader;
#[doc = "Field `THR_H_LIM_EN` writer - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
pub type ThrHLimEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_L_LIM_EN` reader - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
pub type ThrLLimEnR = crate::BitReader;
#[doc = "Field `THR_L_LIM_EN` writer - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
pub type ThrLLimEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_THRES0_EN` reader - This is the enable bit for unit %s's thres0 comparator."]
pub type ThrThres0EnR = crate::BitReader;
#[doc = "Field `THR_THRES0_EN` writer - This is the enable bit for unit %s's thres0 comparator."]
pub type ThrThres0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_THRES1_EN` reader - This is the enable bit for unit %s's thres1 comparator."]
pub type ThrThres1EnR = crate::BitReader;
#[doc = "Field `THR_THRES1_EN` writer - This is the enable bit for unit %s's thres1 comparator."]
pub type ThrThres1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configures the behavior when the signal input of channel %s detects a negative edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EdgeMode {
    #[doc = "1: Increase the counter"]
    Increment = 1,
    #[doc = "2: Decrease the counter"]
    Decrement = 2,
    #[doc = "0: No effect on counter"]
    Hold = 0,
}
impl From<EdgeMode> for u8 {
    #[inline(always)]
    fn from(variant: EdgeMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EdgeMode {
    type Ux = u8;
}
impl crate::IsEnum for EdgeMode {}
#[doc = "Field `CH_NEG_MODE(0-1)` reader - Configures the behavior when the signal input of channel %s detects a negative edge."]
pub type ChNegModeR = crate::FieldReader<EdgeMode>;
impl ChNegModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdgeMode {
        match self.bits {
            1 => EdgeMode::Increment,
            2 => EdgeMode::Decrement,
            _ => EdgeMode::Hold,
        }
    }
    #[doc = "Increase the counter"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == EdgeMode::Increment
    }
    #[doc = "Decrease the counter"]
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        *self == EdgeMode::Decrement
    }
    #[doc = "No effect on counter"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        matches!(self.variant(), EdgeMode::Hold)
    }
}
#[doc = "Field `CH_NEG_MODE(0-1)` writer - Configures the behavior when the signal input of channel %s detects a negative edge."]
pub type ChNegModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, EdgeMode, crate::Safe>;
impl<'a, REG> ChNegModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increase the counter"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(EdgeMode::Increment)
    }
    #[doc = "Decrease the counter"]
    #[inline(always)]
    pub fn decrement(self) -> &'a mut crate::W<REG> {
        self.variant(EdgeMode::Decrement)
    }
    #[doc = "No effect on counter"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(EdgeMode::Hold)
    }
}
#[doc = "Field `CH_POS_MODE(0-1)` reader - Configures the behavior when the signal input of channel %s detects a positive edge."]
pub use ChNegModeR as ChPosModeR;
#[doc = "Field `CH_POS_MODE(0-1)` writer - Configures the behavior when the signal input of channel %s detects a positive edge."]
pub use ChNegModeW as ChPosModeW;
#[doc = "Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrlMode {
    #[doc = "0: No modification"]
    Keep = 0,
    #[doc = "1: Invert behavior (increase -> decrease"]
    Reverse = 1,
    #[doc = "2: Inhibit counter modification"]
    Disable = 2,
}
impl From<CtrlMode> for u8 {
    #[inline(always)]
    fn from(variant: CtrlMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrlMode {
    type Ux = u8;
}
impl crate::IsEnum for CtrlMode {}
#[doc = "Field `CH_HCTRL_MODE(0-1)` reader - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
pub type ChHctrlModeR = crate::FieldReader<CtrlMode>;
impl ChHctrlModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrlMode {
        match self.bits {
            0 => CtrlMode::Keep,
            1 => CtrlMode::Reverse,
            _ => CtrlMode::Disable,
        }
    }
    #[doc = "No modification"]
    #[inline(always)]
    pub fn is_keep(&self) -> bool {
        *self == CtrlMode::Keep
    }
    #[doc = "Invert behavior (increase -> decrease"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == CtrlMode::Reverse
    }
    #[doc = "Inhibit counter modification"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        matches!(self.variant(), CtrlMode::Disable)
    }
}
#[doc = "Field `CH_HCTRL_MODE(0-1)` writer - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
pub type ChHctrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CtrlMode, crate::Safe>;
impl<'a, REG> ChHctrlModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No modification"]
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> {
        self.variant(CtrlMode::Keep)
    }
    #[doc = "Invert behavior (increase -> decrease"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(CtrlMode::Reverse)
    }
    #[doc = "Inhibit counter modification"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtrlMode::Disable)
    }
}
#[doc = "Field `CH_LCTRL_MODE(0-1)` reader - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
pub use ChHctrlModeR as ChLctrlModeR;
#[doc = "Field `CH_LCTRL_MODE(0-1)` writer - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
pub use ChHctrlModeW as ChLctrlModeW;
impl R {
    #[doc = "Bits 0:9 - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
    #[inline(always)]
    pub fn filter_thres(&self) -> FilterThresR {
        FilterThresR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en(&self) -> FilterEnR {
        FilterEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en(&self) -> ThrZeroEnR {
        ThrZeroEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
    #[inline(always)]
    pub fn thr_h_lim_en(&self) -> ThrHLimEnR {
        ThrHLimEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
    #[inline(always)]
    pub fn thr_l_lim_en(&self) -> ThrLLimEnR {
        ThrLLimEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en(&self) -> ThrThres0EnR {
        ThrThres0EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en(&self) -> ThrThres1EnR {
        ThrThres1EnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Configures the behavior when the signal input of channel (0-1) detects a negative edge."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_NEG_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_neg_mode(&self, n: u8) -> ChNegModeR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChNegModeR::new(((self.bits >> (n * 8 + 16)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures the behavior when the signal input of channel (0-1) detects a negative edge."]
    #[inline(always)]
    pub fn ch_neg_mode_iter(&self) -> impl Iterator<Item = ChNegModeR> + '_ {
        (0..2).map(move |n| ChNegModeR::new(((self.bits >> (n * 8 + 16)) & 3) as u8))
    }
    #[doc = "Bits 16:17 - Configures the behavior when the signal input of channel 0 detects a negative edge."]
    #[inline(always)]
    pub fn ch0_neg_mode(&self) -> ChNegModeR {
        ChNegModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Configures the behavior when the signal input of channel 1 detects a negative edge."]
    #[inline(always)]
    pub fn ch1_neg_mode(&self) -> ChNegModeR {
        ChNegModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Configures the behavior when the signal input of channel (0-1) detects a positive edge."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_POS_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_pos_mode(&self, n: u8) -> ChPosModeR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChPosModeR::new(((self.bits >> (n * 8 + 18)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures the behavior when the signal input of channel (0-1) detects a positive edge."]
    #[inline(always)]
    pub fn ch_pos_mode_iter(&self) -> impl Iterator<Item = ChPosModeR> + '_ {
        (0..2).map(move |n| ChPosModeR::new(((self.bits >> (n * 8 + 18)) & 3) as u8))
    }
    #[doc = "Bits 18:19 - Configures the behavior when the signal input of channel 0 detects a positive edge."]
    #[inline(always)]
    pub fn ch0_pos_mode(&self) -> ChPosModeR {
        ChPosModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Configures the behavior when the signal input of channel 1 detects a positive edge."]
    #[inline(always)]
    pub fn ch1_pos_mode(&self) -> ChPosModeR {
        ChPosModeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_HCTRL_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_hctrl_mode(&self, n: u8) -> ChHctrlModeR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChHctrlModeR::new(((self.bits >> (n * 8 + 20)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
    #[inline(always)]
    pub fn ch_hctrl_mode_iter(&self) -> impl Iterator<Item = ChHctrlModeR> + '_ {
        (0..2).map(move |n| ChHctrlModeR::new(((self.bits >> (n * 8 + 20)) & 3) as u8))
    }
    #[doc = "Bits 20:21 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
    #[inline(always)]
    pub fn ch0_hctrl_mode(&self) -> ChHctrlModeR {
        ChHctrlModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
    #[inline(always)]
    pub fn ch1_hctrl_mode(&self) -> ChHctrlModeR {
        ChHctrlModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_LCTRL_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_lctrl_mode(&self, n: u8) -> ChLctrlModeR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChLctrlModeR::new(((self.bits >> (n * 8 + 22)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
    #[inline(always)]
    pub fn ch_lctrl_mode_iter(&self) -> impl Iterator<Item = ChLctrlModeR> + '_ {
        (0..2).map(move |n| ChLctrlModeR::new(((self.bits >> (n * 8 + 22)) & 3) as u8))
    }
    #[doc = "Bits 22:23 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
    #[inline(always)]
    pub fn ch0_lctrl_mode(&self) -> ChLctrlModeR {
        ChLctrlModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
    #[inline(always)]
    pub fn ch1_lctrl_mode(&self) -> ChLctrlModeR {
        ChLctrlModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - This sets the maximum threshold, in APB_CLK cycles, for the filter. Any pulses with width less than this will be ignored when the filter is enabled."]
    #[inline(always)]
    pub fn filter_thres(&mut self) -> FilterThresW<'_, Conf0Spec> {
        FilterThresW::new(self, 0)
    }
    #[doc = "Bit 10 - This is the enable bit for unit %s's input filter."]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FilterEnW<'_, Conf0Spec> {
        FilterEnW::new(self, 10)
    }
    #[doc = "Bit 11 - This is the enable bit for unit %s's zero comparator."]
    #[inline(always)]
    pub fn thr_zero_en(&mut self) -> ThrZeroEnW<'_, Conf0Spec> {
        ThrZeroEnW::new(self, 11)
    }
    #[doc = "Bit 12 - This is the enable bit for unit %s's thr_h_lim comparator. Configures it to enable the high limit interrupt."]
    #[inline(always)]
    pub fn thr_h_lim_en(&mut self) -> ThrHLimEnW<'_, Conf0Spec> {
        ThrHLimEnW::new(self, 12)
    }
    #[doc = "Bit 13 - This is the enable bit for unit %s's thr_l_lim comparator. Configures it to enable the low limit interrupt."]
    #[inline(always)]
    pub fn thr_l_lim_en(&mut self) -> ThrLLimEnW<'_, Conf0Spec> {
        ThrLLimEnW::new(self, 13)
    }
    #[doc = "Bit 14 - This is the enable bit for unit %s's thres0 comparator."]
    #[inline(always)]
    pub fn thr_thres0_en(&mut self) -> ThrThres0EnW<'_, Conf0Spec> {
        ThrThres0EnW::new(self, 14)
    }
    #[doc = "Bit 15 - This is the enable bit for unit %s's thres1 comparator."]
    #[inline(always)]
    pub fn thr_thres1_en(&mut self) -> ThrThres1EnW<'_, Conf0Spec> {
        ThrThres1EnW::new(self, 15)
    }
    #[doc = "Configures the behavior when the signal input of channel (0-1) detects a negative edge."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_NEG_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_neg_mode(&mut self, n: u8) -> ChNegModeW<'_, Conf0Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChNegModeW::new(self, n * 8 + 16)
    }
    #[doc = "Bits 16:17 - Configures the behavior when the signal input of channel 0 detects a negative edge."]
    #[inline(always)]
    pub fn ch0_neg_mode(&mut self) -> ChNegModeW<'_, Conf0Spec> {
        ChNegModeW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Configures the behavior when the signal input of channel 1 detects a negative edge."]
    #[inline(always)]
    pub fn ch1_neg_mode(&mut self) -> ChNegModeW<'_, Conf0Spec> {
        ChNegModeW::new(self, 24)
    }
    #[doc = "Configures the behavior when the signal input of channel (0-1) detects a positive edge."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_POS_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_pos_mode(&mut self, n: u8) -> ChPosModeW<'_, Conf0Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChPosModeW::new(self, n * 8 + 18)
    }
    #[doc = "Bits 18:19 - Configures the behavior when the signal input of channel 0 detects a positive edge."]
    #[inline(always)]
    pub fn ch0_pos_mode(&mut self) -> ChPosModeW<'_, Conf0Spec> {
        ChPosModeW::new(self, 18)
    }
    #[doc = "Bits 26:27 - Configures the behavior when the signal input of channel 1 detects a positive edge."]
    #[inline(always)]
    pub fn ch1_pos_mode(&mut self) -> ChPosModeW<'_, Conf0Spec> {
        ChPosModeW::new(self, 26)
    }
    #[doc = "Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_HCTRL_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_hctrl_mode(&mut self, n: u8) -> ChHctrlModeW<'_, Conf0Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChHctrlModeW::new(self, n * 8 + 20)
    }
    #[doc = "Bits 20:21 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
    #[inline(always)]
    pub fn ch0_hctrl_mode(&mut self) -> ChHctrlModeW<'_, Conf0Spec> {
        ChHctrlModeW::new(self, 20)
    }
    #[doc = "Bits 28:29 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is high."]
    #[inline(always)]
    pub fn ch1_hctrl_mode(&mut self) -> ChHctrlModeW<'_, Conf0Spec> {
        ChHctrlModeW::new(self, 28)
    }
    #[doc = "Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_LCTRL_MODE` field.</div>"]
    #[inline(always)]
    pub fn ch_lctrl_mode(&mut self, n: u8) -> ChLctrlModeW<'_, Conf0Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ChLctrlModeW::new(self, n * 8 + 22)
    }
    #[doc = "Bits 22:23 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
    #[inline(always)]
    pub fn ch0_lctrl_mode(&mut self) -> ChLctrlModeW<'_, Conf0Spec> {
        ChLctrlModeW::new(self, 22)
    }
    #[doc = "Bits 30:31 - Configures how the CHn_POS_MODE/CHn_NEG_MODE settings will be modified when the control signal is low."]
    #[inline(always)]
    pub fn ch1_lctrl_mode(&mut self) -> ChLctrlModeW<'_, Conf0Spec> {
        ChLctrlModeW::new(self, 30)
    }
}
#[doc = "Configuration register 0 for unit\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conf0Spec;
impl crate::RegisterSpec for Conf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for Conf0Spec {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for Conf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF0 to value 0x3c10"]
impl crate::Resettable for Conf0Spec {
    const RESET_VALUE: u32 = 0x3c10;
}
