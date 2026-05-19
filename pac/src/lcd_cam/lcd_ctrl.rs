#[doc = "Register `LCD_CTRL` reader"]
pub type R = crate::R<LcdCtrlSpec>;
#[doc = "Register `LCD_CTRL` writer"]
pub type W = crate::W<LcdCtrlSpec>;
#[doc = "Field `LCD_HB_FRONT` reader - It is the horizontal blank front porch of a frame."]
pub type LcdHbFrontR = crate::FieldReader<u16>;
#[doc = "Field `LCD_HB_FRONT` writer - It is the horizontal blank front porch of a frame."]
pub type LcdHbFrontW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `LCD_VA_HEIGHT` reader - It is the vertical active height of a frame."]
pub type LcdVaHeightR = crate::FieldReader<u16>;
#[doc = "Field `LCD_VA_HEIGHT` writer - It is the vertical active height of a frame."]
pub type LcdVaHeightW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LCD_VT_HEIGHT` reader - It is the vertical total height of a frame."]
pub type LcdVtHeightR = crate::FieldReader<u16>;
#[doc = "Field `LCD_VT_HEIGHT` writer - It is the vertical total height of a frame."]
pub type LcdVtHeightW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LCD_RGB_MODE_EN` reader - 1: Enable LCD RGB mode. 0: Disable LCD RGB mode."]
pub type LcdRgbModeEnR = crate::BitReader;
#[doc = "Field `LCD_RGB_MODE_EN` writer - 1: Enable LCD RGB mode. 0: Disable LCD RGB mode."]
pub type LcdRgbModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_hb_front(&self) -> LcdHbFrontR {
        LcdHbFrontR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame."]
    #[inline(always)]
    pub fn lcd_va_height(&self) -> LcdVaHeightR {
        LcdVaHeightR::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame."]
    #[inline(always)]
    pub fn lcd_vt_height(&self) -> LcdVtHeightR {
        LcdVtHeightR::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 1: Enable LCD RGB mode. 0: Disable LCD RGB mode."]
    #[inline(always)]
    pub fn lcd_rgb_mode_en(&self) -> LcdRgbModeEnR {
        LcdRgbModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - It is the horizontal blank front porch of a frame."]
    #[inline(always)]
    pub fn lcd_hb_front(&mut self) -> LcdHbFrontW<'_, LcdCtrlSpec> {
        LcdHbFrontW::new(self, 0)
    }
    #[doc = "Bits 11:20 - It is the vertical active height of a frame."]
    #[inline(always)]
    pub fn lcd_va_height(&mut self) -> LcdVaHeightW<'_, LcdCtrlSpec> {
        LcdVaHeightW::new(self, 11)
    }
    #[doc = "Bits 21:30 - It is the vertical total height of a frame."]
    #[inline(always)]
    pub fn lcd_vt_height(&mut self) -> LcdVtHeightW<'_, LcdCtrlSpec> {
        LcdVtHeightW::new(self, 21)
    }
    #[doc = "Bit 31 - 1: Enable LCD RGB mode. 0: Disable LCD RGB mode."]
    #[inline(always)]
    pub fn lcd_rgb_mode_en(&mut self) -> LcdRgbModeEnW<'_, LcdCtrlSpec> {
        LcdRgbModeEnW::new(self, 31)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdCtrlSpec;
impl crate::RegisterSpec for LcdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_ctrl::R`](R) reader structure"]
impl crate::Readable for LcdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_ctrl::W`](W) writer structure"]
impl crate::Writable for LcdCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_CTRL to value 0"]
impl crate::Resettable for LcdCtrlSpec {}
