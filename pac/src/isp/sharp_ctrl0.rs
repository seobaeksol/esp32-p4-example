#[doc = "Register `SHARP_CTRL0` reader"]
pub type R = crate::R<SharpCtrl0Spec>;
#[doc = "Register `SHARP_CTRL0` writer"]
pub type W = crate::W<SharpCtrl0Spec>;
#[doc = "Field `SHARP_THRESHOLD_LOW` reader - this field configures sharpen threshold for detail"]
pub type SharpThresholdLowR = crate::FieldReader;
#[doc = "Field `SHARP_THRESHOLD_LOW` writer - this field configures sharpen threshold for detail"]
pub type SharpThresholdLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_THRESHOLD_HIGH` reader - this field configures sharpen threshold for edge"]
pub type SharpThresholdHighR = crate::FieldReader;
#[doc = "Field `SHARP_THRESHOLD_HIGH` writer - this field configures sharpen threshold for edge"]
pub type SharpThresholdHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_AMOUNT_LOW` reader - this field configures sharpen amount for detail"]
pub type SharpAmountLowR = crate::FieldReader;
#[doc = "Field `SHARP_AMOUNT_LOW` writer - this field configures sharpen amount for detail"]
pub type SharpAmountLowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_AMOUNT_HIGH` reader - this field configures sharpen amount for edge"]
pub type SharpAmountHighR = crate::FieldReader;
#[doc = "Field `SHARP_AMOUNT_HIGH` writer - this field configures sharpen amount for edge"]
pub type SharpAmountHighW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures sharpen threshold for detail"]
    #[inline(always)]
    pub fn sharp_threshold_low(&self) -> SharpThresholdLowR {
        SharpThresholdLowR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures sharpen threshold for edge"]
    #[inline(always)]
    pub fn sharp_threshold_high(&self) -> SharpThresholdHighR {
        SharpThresholdHighR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures sharpen amount for detail"]
    #[inline(always)]
    pub fn sharp_amount_low(&self) -> SharpAmountLowR {
        SharpAmountLowR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures sharpen amount for edge"]
    #[inline(always)]
    pub fn sharp_amount_high(&self) -> SharpAmountHighR {
        SharpAmountHighR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures sharpen threshold for detail"]
    #[inline(always)]
    pub fn sharp_threshold_low(&mut self) -> SharpThresholdLowW<'_, SharpCtrl0Spec> {
        SharpThresholdLowW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures sharpen threshold for edge"]
    #[inline(always)]
    pub fn sharp_threshold_high(&mut self) -> SharpThresholdHighW<'_, SharpCtrl0Spec> {
        SharpThresholdHighW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures sharpen amount for detail"]
    #[inline(always)]
    pub fn sharp_amount_low(&mut self) -> SharpAmountLowW<'_, SharpCtrl0Spec> {
        SharpAmountLowW::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures sharpen amount for edge"]
    #[inline(always)]
    pub fn sharp_amount_high(&mut self) -> SharpAmountHighW<'_, SharpCtrl0Spec> {
        SharpAmountHighW::new(self, 24)
    }
}
#[doc = "sharp control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sharp_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharpCtrl0Spec;
impl crate::RegisterSpec for SharpCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharp_ctrl0::R`](R) reader structure"]
impl crate::Readable for SharpCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`sharp_ctrl0::W`](W) writer structure"]
impl crate::Writable for SharpCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHARP_CTRL0 to value 0"]
impl crate::Resettable for SharpCtrl0Spec {}
