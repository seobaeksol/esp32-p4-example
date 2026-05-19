#[doc = "Register `COLOR_CTRL` reader"]
pub type R = crate::R<ColorCtrlSpec>;
#[doc = "Register `COLOR_CTRL` writer"]
pub type W = crate::W<ColorCtrlSpec>;
#[doc = "Field `COLOR_SATURATION` reader - this field configures the color saturation value"]
pub type ColorSaturationR = crate::FieldReader;
#[doc = "Field `COLOR_SATURATION` writer - this field configures the color saturation value"]
pub type ColorSaturationW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_HUE` reader - this field configures the color hue angle"]
pub type ColorHueR = crate::FieldReader;
#[doc = "Field `COLOR_HUE` writer - this field configures the color hue angle"]
pub type ColorHueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_CONTRAST` reader - this field configures the color contrast value"]
pub type ColorContrastR = crate::FieldReader;
#[doc = "Field `COLOR_CONTRAST` writer - this field configures the color contrast value"]
pub type ColorContrastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_BRIGHTNESS` reader - this field configures the color brightness value, signed 2's complement"]
pub type ColorBrightnessR = crate::FieldReader;
#[doc = "Field `COLOR_BRIGHTNESS` writer - this field configures the color brightness value, signed 2's complement"]
pub type ColorBrightnessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the color saturation value"]
    #[inline(always)]
    pub fn color_saturation(&self) -> ColorSaturationR {
        ColorSaturationR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the color hue angle"]
    #[inline(always)]
    pub fn color_hue(&self) -> ColorHueR {
        ColorHueR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the color contrast value"]
    #[inline(always)]
    pub fn color_contrast(&self) -> ColorContrastR {
        ColorContrastR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the color brightness value, signed 2's complement"]
    #[inline(always)]
    pub fn color_brightness(&self) -> ColorBrightnessR {
        ColorBrightnessR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the color saturation value"]
    #[inline(always)]
    pub fn color_saturation(&mut self) -> ColorSaturationW<'_, ColorCtrlSpec> {
        ColorSaturationW::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the color hue angle"]
    #[inline(always)]
    pub fn color_hue(&mut self) -> ColorHueW<'_, ColorCtrlSpec> {
        ColorHueW::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the color contrast value"]
    #[inline(always)]
    pub fn color_contrast(&mut self) -> ColorContrastW<'_, ColorCtrlSpec> {
        ColorContrastW::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the color brightness value, signed 2's complement"]
    #[inline(always)]
    pub fn color_brightness(&mut self) -> ColorBrightnessW<'_, ColorCtrlSpec> {
        ColorBrightnessW::new(self, 24)
    }
}
#[doc = "color control register\n\nYou can [`read`](crate::Reg::read) this register and get [`color_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`color_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ColorCtrlSpec;
impl crate::RegisterSpec for ColorCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`color_ctrl::R`](R) reader structure"]
impl crate::Readable for ColorCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`color_ctrl::W`](W) writer structure"]
impl crate::Writable for ColorCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COLOR_CTRL to value 0x0080_0080"]
impl crate::Resettable for ColorCtrlSpec {
    const RESET_VALUE: u32 = 0x0080_0080;
}
