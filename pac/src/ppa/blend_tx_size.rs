#[doc = "Register `BLEND_TX_SIZE` reader"]
pub type R = crate::R<BlendTxSizeSpec>;
#[doc = "Register `BLEND_TX_SIZE` writer"]
pub type W = crate::W<BlendTxSizeSpec>;
#[doc = "Field `BLEND_HB` reader - The horizontal width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV422 or YUV420"]
pub type BlendHbR = crate::FieldReader<u16>;
#[doc = "Field `BLEND_HB` writer - The horizontal width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV422 or YUV420"]
pub type BlendHbW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `BLEND_VB` reader - The vertical width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV420"]
pub type BlendVbR = crate::FieldReader<u16>;
#[doc = "Field `BLEND_VB` writer - The vertical width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV420"]
pub type BlendVbW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - The horizontal width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV422 or YUV420"]
    #[inline(always)]
    pub fn blend_hb(&self) -> BlendHbR {
        BlendHbR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:27 - The vertical width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV420"]
    #[inline(always)]
    pub fn blend_vb(&self) -> BlendVbR {
        BlendVbR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - The horizontal width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV422 or YUV420"]
    #[inline(always)]
    pub fn blend_hb(&mut self) -> BlendHbW<'_, BlendTxSizeSpec> {
        BlendHbW::new(self, 0)
    }
    #[doc = "Bits 14:27 - The vertical width of image block that would be filled in fix pixel filling mode or blend mode. The unit is pixel. Must be even num when YUV420"]
    #[inline(always)]
    pub fn blend_vb(&mut self) -> BlendVbW<'_, BlendTxSizeSpec> {
        BlendVbW::new(self, 14)
    }
}
#[doc = "Fix pixel filling mode image size register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_tx_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_tx_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlendTxSizeSpec;
impl crate::RegisterSpec for BlendTxSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_tx_size::R`](R) reader structure"]
impl crate::Readable for BlendTxSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`blend_tx_size::W`](W) writer structure"]
impl crate::Writable for BlendTxSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_TX_SIZE to value 0"]
impl crate::Resettable for BlendTxSizeSpec {}
