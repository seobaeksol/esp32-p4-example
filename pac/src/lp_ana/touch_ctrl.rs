#[doc = "Register `TOUCH_CTRL` reader"]
pub type R = crate::R<TouchCtrlSpec>;
#[doc = "Register `TOUCH_CTRL` writer"]
pub type W = crate::W<TouchCtrlSpec>;
#[doc = "Field `TOUCH_UPDATE_BASELINE_FREQ_SEL` reader - Configure the frequency point for software to update the baseline"]
pub type TouchUpdateBaselineFreqSelR = crate::FieldReader;
#[doc = "Field `TOUCH_UPDATE_BASELINE_FREQ_SEL` writer - Configure the frequency point for software to update the baseline"]
pub type TouchUpdateBaselineFreqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_UPDATE_BASELINE_PAD_SEL` reader - Configure the channel for software to update the baseline."]
pub type TouchUpdateBaselinePadSelR = crate::FieldReader;
#[doc = "Field `TOUCH_UPDATE_BASELINE_PAD_SEL` writer - Configure the channel for software to update the baseline."]
pub type TouchUpdateBaselinePadSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FREQ_SCAN_CNT_RISE` reader - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
pub type FreqScanCntRiseR = crate::FieldReader;
#[doc = "Field `FREQ_SCAN_CNT_RISE` writer - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
pub type FreqScanCntRiseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Configure the frequency point for software to update the baseline"]
    #[inline(always)]
    pub fn touch_update_baseline_freq_sel(&self) -> TouchUpdateBaselineFreqSelR {
        TouchUpdateBaselineFreqSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Configure the channel for software to update the baseline."]
    #[inline(always)]
    pub fn touch_update_baseline_pad_sel(&self) -> TouchUpdateBaselinePadSelR {
        TouchUpdateBaselinePadSelR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
    #[inline(always)]
    pub fn freq_scan_cnt_rise(&self) -> FreqScanCntRiseR {
        FreqScanCntRiseR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure the frequency point for software to update the baseline"]
    #[inline(always)]
    pub fn touch_update_baseline_freq_sel(
        &mut self,
    ) -> TouchUpdateBaselineFreqSelW<'_, TouchCtrlSpec> {
        TouchUpdateBaselineFreqSelW::new(self, 0)
    }
    #[doc = "Bits 2:5 - Configure the channel for software to update the baseline."]
    #[inline(always)]
    pub fn touch_update_baseline_pad_sel(
        &mut self,
    ) -> TouchUpdateBaselinePadSelW<'_, TouchCtrlSpec> {
        TouchUpdateBaselinePadSelW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
    #[inline(always)]
    pub fn freq_scan_cnt_rise(&mut self) -> FreqScanCntRiseW<'_, TouchCtrlSpec> {
        FreqScanCntRiseW::new(self, 6)
    }
}
#[doc = "Touch Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchCtrlSpec;
impl crate::RegisterSpec for TouchCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_ctrl::R`](R) reader structure"]
impl crate::Readable for TouchCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_ctrl::W`](W) writer structure"]
impl crate::Writable for TouchCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_CTRL to value 0x40"]
impl crate::Resettable for TouchCtrlSpec {
    const RESET_VALUE: u32 = 0x40;
}
