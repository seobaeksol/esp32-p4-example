#[doc = "Register `SHARP_CTRL1` reader"]
pub type R = crate::R<SharpCtrl1Spec>;
#[doc = "Field `SHARP_GRADIENT_MAX` reader - this field configures sharp max gradient, refresh at the end of each frame end"]
pub type SharpGradientMaxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures sharp max gradient, refresh at the end of each frame end"]
    #[inline(always)]
    pub fn sharp_gradient_max(&self) -> SharpGradientMaxR {
        SharpGradientMaxR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "sharp control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sharp_ctrl1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharpCtrl1Spec;
impl crate::RegisterSpec for SharpCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharp_ctrl1::R`](R) reader structure"]
impl crate::Readable for SharpCtrl1Spec {}
#[doc = "`reset()` method sets SHARP_CTRL1 to value 0"]
impl crate::Resettable for SharpCtrl1Spec {}
