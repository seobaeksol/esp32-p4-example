#[doc = "Register `BLEND_RGB` reader"]
pub type R = crate::R<BlendRgbSpec>;
#[doc = "Register `BLEND_RGB` writer"]
pub type W = crate::W<BlendRgbSpec>;
#[doc = "Field `BLEND1_RX_B` reader - blue color for A4/A8 mode."]
pub type Blend1RxBR = crate::FieldReader;
#[doc = "Field `BLEND1_RX_B` writer - blue color for A4/A8 mode."]
pub type Blend1RxBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLEND1_RX_G` reader - green color for A4/A8 mode."]
pub type Blend1RxGR = crate::FieldReader;
#[doc = "Field `BLEND1_RX_G` writer - green color for A4/A8 mode."]
pub type Blend1RxGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLEND1_RX_R` reader - red color for A4/A8 mode."]
pub type Blend1RxRR = crate::FieldReader;
#[doc = "Field `BLEND1_RX_R` writer - red color for A4/A8 mode."]
pub type Blend1RxRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - blue color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_b(&self) -> Blend1RxBR {
        Blend1RxBR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - green color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_g(&self) -> Blend1RxGR {
        Blend1RxGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - red color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_r(&self) -> Blend1RxRR {
        Blend1RxRR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - blue color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_b(&mut self) -> Blend1RxBW<'_, BlendRgbSpec> {
        Blend1RxBW::new(self, 0)
    }
    #[doc = "Bits 8:15 - green color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_g(&mut self) -> Blend1RxGW<'_, BlendRgbSpec> {
        Blend1RxGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - red color for A4/A8 mode."]
    #[inline(always)]
    pub fn blend1_rx_r(&mut self) -> Blend1RxRW<'_, BlendRgbSpec> {
        Blend1RxRW::new(self, 16)
    }
}
#[doc = "RGB color register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_rgb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_rgb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlendRgbSpec;
impl crate::RegisterSpec for BlendRgbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_rgb::R`](R) reader structure"]
impl crate::Readable for BlendRgbSpec {}
#[doc = "`write(|w| ..)` method takes [`blend_rgb::W`](W) writer structure"]
impl crate::Writable for BlendRgbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_RGB to value 0x0080_8080"]
impl crate::Resettable for BlendRgbSpec {
    const RESET_VALUE: u32 = 0x0080_8080;
}
