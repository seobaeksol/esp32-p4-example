#[doc = "Register `TOUCH_FILTER1` reader"]
pub type R = crate::R<TouchFilter1Spec>;
#[doc = "Register `TOUCH_FILTER1` writer"]
pub type W = crate::W<TouchFilter1Spec>;
#[doc = "Field `TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN` reader - Reserved"]
pub type TouchNegNoiseDisupdateBaselineEnR = crate::BitReader;
#[doc = "Field `TOUCH_NEG_NOISE_DISUPDATE_BASELINE_EN` writer - Reserved"]
pub type TouchNegNoiseDisupdateBaselineEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_HYSTERESIS` reader - need_des"]
pub type TouchHysteresisR = crate::FieldReader;
#[doc = "Field `TOUCH_HYSTERESIS` writer - need_des"]
pub type TouchHysteresisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` reader - need_des"]
pub type TouchNegNoiseThresR = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_THRES` writer - need_des"]
pub type TouchNegNoiseThresW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_NOISE_THRES` reader - need_des"]
pub type TouchNoiseThresR = crate::FieldReader;
#[doc = "Field `TOUCH_NOISE_THRES` writer - need_des"]
pub type TouchNoiseThresW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_SMOOTH_LVL` reader - need_des"]
pub type TouchSmoothLvlR = crate::FieldReader;
#[doc = "Field `TOUCH_SMOOTH_LVL` writer - need_des"]
pub type TouchSmoothLvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_JITTER_STEP` reader - need_des"]
pub type TouchJitterStepR = crate::FieldReader;
#[doc = "Field `TOUCH_JITTER_STEP` writer - need_des"]
pub type TouchJitterStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_FILTER_MODE` reader - need_des"]
pub type TouchFilterModeR = crate::FieldReader;
#[doc = "Field `TOUCH_FILTER_MODE` writer - need_des"]
pub type TouchFilterModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_FILTER_EN` reader - need_des"]
pub type TouchFilterEnR = crate::BitReader;
#[doc = "Field `TOUCH_FILTER_EN` writer - need_des"]
pub type TouchFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` reader - need_des"]
pub type TouchNegNoiseLimitR = crate::FieldReader;
#[doc = "Field `TOUCH_NEG_NOISE_LIMIT` writer - need_des"]
pub type TouchNegNoiseLimitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_APPROACH_LIMIT` reader - need_des"]
pub type TouchApproachLimitR = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_LIMIT` writer - need_des"]
pub type TouchApproachLimitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOUCH_DEBOUNCE_LIMIT` reader - need_des"]
pub type TouchDebounceLimitR = crate::FieldReader;
#[doc = "Field `TOUCH_DEBOUNCE_LIMIT` writer - need_des"]
pub type TouchDebounceLimitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn touch_neg_noise_disupdate_baseline_en(&self) -> TouchNegNoiseDisupdateBaselineEnR {
        TouchNegNoiseDisupdateBaselineEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn touch_hysteresis(&self) -> TouchHysteresisR {
        TouchHysteresisR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn touch_neg_noise_thres(&self) -> TouchNegNoiseThresR {
        TouchNegNoiseThresR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn touch_noise_thres(&self) -> TouchNoiseThresR {
        TouchNoiseThresR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_smooth_lvl(&self) -> TouchSmoothLvlR {
        TouchSmoothLvlR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_jitter_step(&self) -> TouchJitterStepR {
        TouchJitterStepR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn touch_filter_mode(&self) -> TouchFilterModeR {
        TouchFilterModeR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn touch_filter_en(&self) -> TouchFilterEnR {
        TouchFilterEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    pub fn touch_neg_noise_limit(&self) -> TouchNegNoiseLimitR {
        TouchNegNoiseLimitR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn touch_approach_limit(&self) -> TouchApproachLimitR {
        TouchApproachLimitR::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn touch_debounce_limit(&self) -> TouchDebounceLimitR {
        TouchDebounceLimitR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn touch_neg_noise_disupdate_baseline_en(
        &mut self,
    ) -> TouchNegNoiseDisupdateBaselineEnW<'_, TouchFilter1Spec> {
        TouchNegNoiseDisupdateBaselineEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - need_des"]
    #[inline(always)]
    pub fn touch_hysteresis(&mut self) -> TouchHysteresisW<'_, TouchFilter1Spec> {
        TouchHysteresisW::new(self, 1)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn touch_neg_noise_thres(&mut self) -> TouchNegNoiseThresW<'_, TouchFilter1Spec> {
        TouchNegNoiseThresW::new(self, 3)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn touch_noise_thres(&mut self) -> TouchNoiseThresW<'_, TouchFilter1Spec> {
        TouchNoiseThresW::new(self, 5)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_smooth_lvl(&mut self) -> TouchSmoothLvlW<'_, TouchFilter1Spec> {
        TouchSmoothLvlW::new(self, 7)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_jitter_step(&mut self) -> TouchJitterStepW<'_, TouchFilter1Spec> {
        TouchJitterStepW::new(self, 9)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn touch_filter_mode(&mut self) -> TouchFilterModeW<'_, TouchFilter1Spec> {
        TouchFilterModeW::new(self, 13)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn touch_filter_en(&mut self) -> TouchFilterEnW<'_, TouchFilter1Spec> {
        TouchFilterEnW::new(self, 16)
    }
    #[doc = "Bits 17:20 - need_des"]
    #[inline(always)]
    pub fn touch_neg_noise_limit(&mut self) -> TouchNegNoiseLimitW<'_, TouchFilter1Spec> {
        TouchNegNoiseLimitW::new(self, 17)
    }
    #[doc = "Bits 21:28 - need_des"]
    #[inline(always)]
    pub fn touch_approach_limit(&mut self) -> TouchApproachLimitW<'_, TouchFilter1Spec> {
        TouchApproachLimitW::new(self, 21)
    }
    #[doc = "Bits 29:31 - need_des"]
    #[inline(always)]
    pub fn touch_debounce_limit(&mut self) -> TouchDebounceLimitW<'_, TouchFilter1Spec> {
        TouchDebounceLimitW::new(self, 29)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_filter1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_filter1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchFilter1Spec;
impl crate::RegisterSpec for TouchFilter1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_filter1::R`](R) reader structure"]
impl crate::Readable for TouchFilter1Spec {}
#[doc = "`write(|w| ..)` method takes [`touch_filter1::W`](W) writer structure"]
impl crate::Writable for TouchFilter1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_FILTER1 to value 0x6a0a_0200"]
impl crate::Resettable for TouchFilter1Spec {
    const RESET_VALUE: u32 = 0x6a0a_0200;
}
