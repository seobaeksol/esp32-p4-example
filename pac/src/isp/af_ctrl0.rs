#[doc = "Register `AF_CTRL0` reader"]
pub type R = crate::R<AfCtrl0Spec>;
#[doc = "Register `AF_CTRL0` writer"]
pub type W = crate::W<AfCtrl0Spec>;
#[doc = "Field `AF_AUTO_UPDATE` reader - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
pub type AfAutoUpdateR = crate::BitReader;
#[doc = "Field `AF_AUTO_UPDATE` writer - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
pub type AfAutoUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_MANUAL_UPDATE` writer - write 1 to this bit will update the sum and lum once"]
pub type AfManualUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_ENV_THRESHOLD` reader - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
pub type AfEnvThresholdR = crate::FieldReader;
#[doc = "Field `AF_ENV_THRESHOLD` writer - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
pub type AfEnvThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF_ENV_PERIOD` reader - this field configures environment changes detection period (frame). When set to 0, disable this function"]
pub type AfEnvPeriodR = crate::FieldReader;
#[doc = "Field `AF_ENV_PERIOD` writer - this field configures environment changes detection period (frame). When set to 0, disable this function"]
pub type AfEnvPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
    #[inline(always)]
    pub fn af_auto_update(&self) -> AfAutoUpdateR {
        AfAutoUpdateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
    #[inline(always)]
    pub fn af_env_threshold(&self) -> AfEnvThresholdR {
        AfEnvThresholdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - this field configures environment changes detection period (frame). When set to 0, disable this function"]
    #[inline(always)]
    pub fn af_env_period(&self) -> AfEnvPeriodR {
        AfEnvPeriodR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
    #[inline(always)]
    pub fn af_auto_update(&mut self) -> AfAutoUpdateW<'_, AfCtrl0Spec> {
        AfAutoUpdateW::new(self, 0)
    }
    #[doc = "Bit 4 - write 1 to this bit will update the sum and lum once"]
    #[inline(always)]
    pub fn af_manual_update(&mut self) -> AfManualUpdateW<'_, AfCtrl0Spec> {
        AfManualUpdateW::new(self, 4)
    }
    #[doc = "Bits 8:11 - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
    #[inline(always)]
    pub fn af_env_threshold(&mut self) -> AfEnvThresholdW<'_, AfCtrl0Spec> {
        AfEnvThresholdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures environment changes detection period (frame). When set to 0, disable this function"]
    #[inline(always)]
    pub fn af_env_period(&mut self) -> AfEnvPeriodW<'_, AfCtrl0Spec> {
        AfEnvPeriodW::new(self, 16)
    }
}
#[doc = "af control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`af_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfCtrl0Spec;
impl crate::RegisterSpec for AfCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_ctrl0::R`](R) reader structure"]
impl crate::Readable for AfCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`af_ctrl0::W`](W) writer structure"]
impl crate::Writable for AfCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF_CTRL0 to value 0"]
impl crate::Resettable for AfCtrl0Spec {}
