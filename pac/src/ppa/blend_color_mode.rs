#[doc = "Register `BLEND_COLOR_MODE` reader"]
pub type R = crate::R<BlendColorModeSpec>;
#[doc = "Register `BLEND_COLOR_MODE` writer"]
pub type W = crate::W<BlendColorModeSpec>;
#[doc = "Field `BLEND0_RX_CM` reader - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type Blend0RxCmR = crate::FieldReader;
#[doc = "Field `BLEND0_RX_CM` writer - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type Blend0RxCmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND1_RX_CM` reader - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
pub type Blend1RxCmR = crate::FieldReader;
#[doc = "Field `BLEND1_RX_CM` writer - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
pub type Blend1RxCmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND_TX_CM` reader - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type BlendTxCmR = crate::FieldReader;
#[doc = "Field `BLEND_TX_CM` writer - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
pub type BlendTxCmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BLEND0_RX_YUV_RANGE` reader - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
pub type Blend0RxYuvRangeR = crate::BitReader;
#[doc = "Field `BLEND0_RX_YUV_RANGE` writer - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
pub type Blend0RxYuvRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_TX_YUV_RANGE` reader - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
pub type BlendTxYuvRangeR = crate::BitReader;
#[doc = "Field `BLEND_TX_YUV_RANGE` writer - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
pub type BlendTxYuvRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_RX_YUV2RGB_PROTOCAL` reader - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
pub type Blend0RxYuv2rgbProtocalR = crate::BitReader;
#[doc = "Field `BLEND0_RX_YUV2RGB_PROTOCAL` writer - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
pub type Blend0RxYuv2rgbProtocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_TX_RGB2YUV_PROTOCAL` reader - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
pub type BlendTxRgb2yuvProtocalR = crate::BitReader;
#[doc = "Field `BLEND_TX_RGB2YUV_PROTOCAL` writer - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
pub type BlendTxRgb2yuvProtocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND0_RX_YUV422_BYTE_ORDER` reader - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
pub type Blend0RxYuv422ByteOrderR = crate::FieldReader;
#[doc = "Field `BLEND0_RX_YUV422_BYTE_ORDER` writer - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
pub type Blend0RxYuv422ByteOrderW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend0_rx_cm(&self) -> Blend0RxCmR {
        Blend0RxCmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
    #[inline(always)]
    pub fn blend1_rx_cm(&self) -> Blend1RxCmR {
        Blend1RxCmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend_tx_cm(&self) -> BlendTxCmR {
        BlendTxCmR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend0_rx_yuv_range(&self) -> Blend0RxYuvRangeR {
        Blend0RxYuvRangeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend_tx_yuv_range(&self) -> BlendTxYuvRangeR {
        BlendTxYuvRangeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend0_rx_yuv2rgb_protocal(&self) -> Blend0RxYuv2rgbProtocalR {
        Blend0RxYuv2rgbProtocalR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend_tx_rgb2yuv_protocal(&self) -> BlendTxRgb2yuvProtocalR {
        BlendTxRgb2yuvProtocalR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
    #[inline(always)]
    pub fn blend0_rx_yuv422_byte_order(&self) -> Blend0RxYuv422ByteOrderR {
        Blend0RxYuv422ByteOrderR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The source image color mode for background plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend0_rx_cm(&mut self) -> Blend0RxCmW<'_, BlendColorModeSpec> {
        Blend0RxCmW::new(self, 0)
    }
    #[doc = "Bits 4:7 - The source image color mode for foreground plane. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 4: L8. 5: L4. 6: A8. 7: A4."]
    #[inline(always)]
    pub fn blend1_rx_cm(&mut self) -> Blend1RxCmW<'_, BlendColorModeSpec> {
        Blend1RxCmW::new(self, 4)
    }
    #[doc = "Bits 8:11 - The destination image color mode for output of blender. 0: ARGB8888. 1: RGB888. 2: RGB565. 3: Reserved. 8: YUV420. 9: YUV422. 12:GRAY"]
    #[inline(always)]
    pub fn blend_tx_cm(&mut self) -> BlendTxCmW<'_, BlendColorModeSpec> {
        BlendTxCmW::new(self, 8)
    }
    #[doc = "Bit 12 - YUV input range when blend0 rx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend0_rx_yuv_range(&mut self) -> Blend0RxYuvRangeW<'_, BlendColorModeSpec> {
        Blend0RxYuvRangeW::new(self, 12)
    }
    #[doc = "Bit 13 - YUV output range when blend tx cm is yuv. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn blend_tx_yuv_range(&mut self) -> BlendTxYuvRangeW<'_, BlendColorModeSpec> {
        BlendTxYuvRangeW::new(self, 13)
    }
    #[doc = "Bit 14 - YUV to RGB protocal when blend0 rx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend0_rx_yuv2rgb_protocal(
        &mut self,
    ) -> Blend0RxYuv2rgbProtocalW<'_, BlendColorModeSpec> {
        Blend0RxYuv2rgbProtocalW::new(self, 14)
    }
    #[doc = "Bit 15 - RGB to YUV protocal when blend tx cm is yuv. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn blend_tx_rgb2yuv_protocal(&mut self) -> BlendTxRgb2yuvProtocalW<'_, BlendColorModeSpec> {
        BlendTxRgb2yuvProtocalW::new(self, 15)
    }
    #[doc = "Bits 16:17 - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
    #[inline(always)]
    pub fn blend0_rx_yuv422_byte_order(
        &mut self,
    ) -> Blend0RxYuv422ByteOrderW<'_, BlendColorModeSpec> {
        Blend0RxYuv422ByteOrderW::new(self, 16)
    }
}
#[doc = "blending engine color mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`blend_color_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blend_color_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlendColorModeSpec;
impl crate::RegisterSpec for BlendColorModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_color_mode::R`](R) reader structure"]
impl crate::Readable for BlendColorModeSpec {}
#[doc = "`write(|w| ..)` method takes [`blend_color_mode::W`](W) writer structure"]
impl crate::Writable for BlendColorModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEND_COLOR_MODE to value 0"]
impl crate::Resettable for BlendColorModeSpec {}
