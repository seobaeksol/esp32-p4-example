#[doc = "Register `PAD_RTC_HOLD_CTRL0` reader"]
pub type R = crate::R<PadRtcHoldCtrl0Spec>;
#[doc = "Register `PAD_RTC_HOLD_CTRL0` writer"]
pub type W = crate::W<PadRtcHoldCtrl0Spec>;
#[doc = "Field `PAD_RTC_HOLD_CTRL0` reader - Set 1 to hold pad 0-31 status"]
pub type PadRtcHoldCtrl0R = crate::FieldReader<u32>;
#[doc = "Field `PAD_RTC_HOLD_CTRL0` writer - Set 1 to hold pad 0-31 status"]
pub type PadRtcHoldCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set 1 to hold pad 0-31 status"]
    #[inline(always)]
    pub fn pad_rtc_hold_ctrl0(&self) -> PadRtcHoldCtrl0R {
        PadRtcHoldCtrl0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set 1 to hold pad 0-31 status"]
    #[inline(always)]
    pub fn pad_rtc_hold_ctrl0(&mut self) -> PadRtcHoldCtrl0W<'_, PadRtcHoldCtrl0Spec> {
        PadRtcHoldCtrl0W::new(self, 0)
    }
}
#[doc = "enable pad hold ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_rtc_hold_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_rtc_hold_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadRtcHoldCtrl0Spec;
impl crate::RegisterSpec for PadRtcHoldCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_rtc_hold_ctrl0::R`](R) reader structure"]
impl crate::Readable for PadRtcHoldCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pad_rtc_hold_ctrl0::W`](W) writer structure"]
impl crate::Writable for PadRtcHoldCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_RTC_HOLD_CTRL0 to value 0"]
impl crate::Resettable for PadRtcHoldCtrl0Spec {}
