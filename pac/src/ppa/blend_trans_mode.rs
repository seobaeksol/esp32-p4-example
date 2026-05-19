#[doc = "Register `BLEND_TRANS_MODE` reader"]
pub type R = crate::R<BlendTransModeSpec>;
#[doc = "Register `BLEND_TRANS_MODE` writer"]
pub type W = crate::W<BlendTransModeSpec>;
#[doc = "Field `BLEND_EN` reader - Set this bit to enable alpha blending."]
pub type BlendEnR = crate::BitReader;
#[doc = "Field `BLEND_EN` writer - Set this bit to enable alpha blending."]
pub type BlendEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_BYPASS` reader - Set this bit to bypass blender. Then background date would be output."]
pub type BlendBypassR = crate::BitReader;
#[doc = "Field `BLEND_BYPASS` writer - Set this bit to bypass blender. Then background date would be output."]
pub type BlendBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_FIX_PIXEL_FILL_EN` reader - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
pub type BlendFixPixelFillEnR = crate::BitReader;
#[doc = "Field `BLEND_FIX_PIXEL_FILL_EN` writer - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
pub type BlendFixPixelFillEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE` writer - Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid."]
pub type UpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_RST` reader - write 1 then write 0 to reset blending engine."]
pub type BlendRstR = crate::BitReader;
#[doc = "Field `BLEND_RST` writer - write 1 then write 0 to reset blending engine."]
pub type BlendRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_TX_INF_SEL` reader - unused ! Configures blend tx interface. 0: dma2d only, 1: le_enc only, 2: dma2d and ls_enc"]
pub type BlendTxInfSelR = crate::FieldReader;
#[doc = "Field `BLEND_TX_INF_SEL` writer - unused ! Configures blend tx interface. 0: dma2d only, 1: le_enc only, 2: dma2d and ls_enc"]
pub type BlendTxInfSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable alpha blending."]
    #[inline(always)]
    pub fn blend_en(&self) -> BlendEnR {
        BlendEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass blender. Then background date would be output."]
    #[inline(always)]
    pub fn blend_bypass(&self) -> BlendBypassR {
        BlendBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
    #[inline(always)]
    pub fn blend_fix_pixel_fill_en(&self) -> BlendFixPixelFillEnR {
        BlendFixPixelFillEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - write 1 then write 0 to reset blending engine."]
    #[inline(always)]
    pub fn blend_rst(&self) -> BlendRstR {
        BlendRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - unused ! Configures blend tx interface. 0: dma2d only, 1: le_enc only, 2: dma2d and ls_enc"]
    #[inline(always)]
    pub fn blend_tx_inf_sel(&self) -> BlendTxInfSelR {
        BlendTxInfSelR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable alpha blending."]
    #[inline(always)]
    pub fn blend_en(&mut self) -> BlendEnW<'_, BlendTransModeSpec> {
        BlendEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to bypass blender. Then background date would be output."]
    #[inline(always)]
    pub fn blend_bypass(&mut self) -> BlendBypassW<'_, BlendTransModeSpec> {
        BlendBypassW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to enable fix pixel filling. When this mode is enable only Tx channel is work and the output pixel is configured by PPA_OUT_FIX_PIXEL."]
    #[inline(always)]
    pub fn blend_fix_pixel_fill_en(&mut self) -> BlendFixPixelFillEnW<'_, BlendTransModeSpec> {
        BlendFixPixelFillEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to update the transfer mode. Only the bit is set the transfer mode is valid."]
    #[inline(always)]
    pub fn update(&mut self) -> UpdateW<'_, BlendTransModeSpec> {
        UpdateW::new(self, 3)
    }
    #[doc = "Bit 4 - write 1 then write 0 to reset blending engine."]
    #[inline(always)]
    pub fn blend_rst(&mut self) -> BlendRstW<'_, BlendTransModeSpec> {
        BlendRstW::new(self, 4)
    }
    #[doc = "Bits 5:6 - unused ! Configures blend tx interface. 0: dma2d only, 1: le_enc only, 2: dma2d and ls_enc"]
    #[inline(always)]
    pub fn blend_tx_inf_sel(&mut self) -> BlendTxInfSelW<'_, BlendTransModeSpec> {
        BlendTxInfSelW::new(self, 5)
    }
}
#[doc = "Blending engine mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_trans_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_trans_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlendTransModeSpec;
impl crate::RegisterSpec for BlendTransModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_trans_mode::R`](R) reader structure"]
impl crate::Readable for BlendTransModeSpec {}
#[doc = "`write(|w| ..)` method takes [`blend_trans_mode::W`](W) writer structure"]
impl crate::Writable for BlendTransModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_TRANS_MODE to value 0"]
impl crate::Resettable for BlendTransModeSpec {}
