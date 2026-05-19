#[doc = "Register `GLITCH_FILTER_CH%s` reader"]
pub type R = crate::R<GlitchFilterChSpec>;
#[doc = "Register `GLITCH_FILTER_CH%s` writer"]
pub type W = crate::W<GlitchFilterChSpec>;
#[doc = "Field `EN` reader - Glitch Filter channel enable bit."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Glitch Filter channel enable bit."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_IO_NUM` reader - Glitch Filter input io number."]
pub type InputIoNumR = crate::FieldReader;
#[doc = "Field `INPUT_IO_NUM` writer - Glitch Filter input io number."]
pub type InputIoNumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WINDOW_THRES` reader - Glitch Filter window threshold."]
pub type WindowThresR = crate::FieldReader;
#[doc = "Field `WINDOW_THRES` writer - Glitch Filter window threshold."]
pub type WindowThresW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WINDOW_WIDTH` reader - Glitch Filter window width."]
pub type WindowWidthR = crate::FieldReader;
#[doc = "Field `WINDOW_WIDTH` writer - Glitch Filter window width."]
pub type WindowWidthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Glitch Filter channel enable bit."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Glitch Filter input io number."]
    #[inline(always)]
    pub fn input_io_num(&self) -> InputIoNumR {
        InputIoNumR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:12 - Glitch Filter window threshold."]
    #[inline(always)]
    pub fn window_thres(&self) -> WindowThresR {
        WindowThresR::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:18 - Glitch Filter window width."]
    #[inline(always)]
    pub fn window_width(&self) -> WindowWidthR {
        WindowWidthR::new(((self.bits >> 13) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Glitch Filter channel enable bit."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, GlitchFilterChSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:6 - Glitch Filter input io number."]
    #[inline(always)]
    pub fn input_io_num(&mut self) -> InputIoNumW<'_, GlitchFilterChSpec> {
        InputIoNumW::new(self, 1)
    }
    #[doc = "Bits 7:12 - Glitch Filter window threshold."]
    #[inline(always)]
    pub fn window_thres(&mut self) -> WindowThresW<'_, GlitchFilterChSpec> {
        WindowThresW::new(self, 7)
    }
    #[doc = "Bits 13:18 - Glitch Filter window width."]
    #[inline(always)]
    pub fn window_width(&mut self) -> WindowWidthW<'_, GlitchFilterChSpec> {
        WindowWidthW::new(self, 13)
    }
}
#[doc = "Glitch Filter Configure Register of Channel%s\n\nYou can [`read`](crate::Reg::read) this register and get [`glitch_filter_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glitch_filter_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlitchFilterChSpec;
impl crate::RegisterSpec for GlitchFilterChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glitch_filter_ch::R`](R) reader structure"]
impl crate::Readable for GlitchFilterChSpec {}
#[doc = "`write(|w| ..)` method takes [`glitch_filter_ch::W`](W) writer structure"]
impl crate::Writable for GlitchFilterChSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GLITCH_FILTER_CH%s to value 0"]
impl crate::Resettable for GlitchFilterChSpec {}
