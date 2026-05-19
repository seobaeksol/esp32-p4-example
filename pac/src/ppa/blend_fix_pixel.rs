#[doc = "Register `BLEND_FIX_PIXEL` reader"]
pub type R = crate::R<BlendFixPixelSpec>;
#[doc = "Register `BLEND_FIX_PIXEL` writer"]
pub type W = crate::W<BlendFixPixelSpec>;
#[doc = "Field `BLEND_TX_FIX_PIXEL` reader - The configure fix pixel in fix pixel filling mode for blender engine."]
pub type BlendTxFixPixelR = crate::FieldReader<u32>;
#[doc = "Field `BLEND_TX_FIX_PIXEL` writer - The configure fix pixel in fix pixel filling mode for blender engine."]
pub type BlendTxFixPixelW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The configure fix pixel in fix pixel filling mode for blender engine."]
    #[inline(always)]
    pub fn blend_tx_fix_pixel(&self) -> BlendTxFixPixelR {
        BlendTxFixPixelR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The configure fix pixel in fix pixel filling mode for blender engine."]
    #[inline(always)]
    pub fn blend_tx_fix_pixel(&mut self) -> BlendTxFixPixelW<'_, BlendFixPixelSpec> {
        BlendTxFixPixelW::new(self, 0)
    }
}
#[doc = "Blending engine fix pixel register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_fix_pixel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_fix_pixel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlendFixPixelSpec;
impl crate::RegisterSpec for BlendFixPixelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_fix_pixel::R`](R) reader structure"]
impl crate::Readable for BlendFixPixelSpec {}
#[doc = "`write(|w| ..)` method takes [`blend_fix_pixel::W`](W) writer structure"]
impl crate::Writable for BlendFixPixelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_FIX_PIXEL to value 0"]
impl crate::Resettable for BlendFixPixelSpec {}
