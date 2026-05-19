#[doc = "Register `SR_COLOR_MODE` reader"]
pub type R = crate::R<SrColorModeSpec>;
#[doc = "Register `SR_COLOR_MODE` writer"]
pub type W = crate::W<SrColorModeSpec>;
#[doc = "Field `SR_RX_CM` reader - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
pub type SrRxCmR = crate::FieldReader;
#[doc = "Field `SR_RX_CM` writer - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
pub type SrRxCmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SR_TX_CM` reader - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
pub type SrTxCmR = crate::FieldReader;
#[doc = "Field `SR_TX_CM` writer - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
pub type SrTxCmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YUV_RX_RANGE` reader - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YuvRxRangeR = crate::BitReader;
#[doc = "Field `YUV_RX_RANGE` writer - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YuvRxRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV_TX_RANGE` reader - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YuvTxRangeR = crate::BitReader;
#[doc = "Field `YUV_TX_RANGE` writer - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
pub type YuvTxRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV2RGB_PROTOCAL` reader - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type Yuv2rgbProtocalR = crate::BitReader;
#[doc = "Field `YUV2RGB_PROTOCAL` writer - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type Yuv2rgbProtocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGB2YUV_PROTOCAL` reader - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type Rgb2yuvProtocalR = crate::BitReader;
#[doc = "Field `RGB2YUV_PROTOCAL` writer - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
pub type Rgb2yuvProtocalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV422_RX_BYTE_ORDER` reader - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
pub type Yuv422RxByteOrderR = crate::FieldReader;
#[doc = "Field `YUV422_RX_BYTE_ORDER` writer - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
pub type Yuv422RxByteOrderW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
    #[inline(always)]
    pub fn sr_rx_cm(&self) -> SrRxCmR {
        SrRxCmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
    #[inline(always)]
    pub fn sr_tx_cm(&self) -> SrTxCmR {
        SrTxCmR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn yuv_rx_range(&self) -> YuvRxRangeR {
        YuvRxRangeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn yuv_tx_range(&self) -> YuvTxRangeR {
        YuvTxRangeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn yuv2rgb_protocal(&self) -> Yuv2rgbProtocalR {
        Yuv2rgbProtocalR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn rgb2yuv_protocal(&self) -> Rgb2yuvProtocalR {
        Rgb2yuvProtocalR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
    #[inline(always)]
    pub fn yuv422_rx_byte_order(&self) -> Yuv422RxByteOrderR {
        Yuv422RxByteOrderR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The source image color mode for Scaling and Rotating engine Rx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
    #[inline(always)]
    pub fn sr_rx_cm(&mut self) -> SrRxCmW<'_, SrColorModeSpec> {
        SrRxCmW::new(self, 0)
    }
    #[doc = "Bits 4:7 - The destination image color mode for Scaling and Rotating engine Tx. 0: ARGB8888. 1: RGB888. 2: RGB565. 8: YUV420. 9: YUV422. 12: GRAY. others: Reserved."]
    #[inline(always)]
    pub fn sr_tx_cm(&mut self) -> SrTxCmW<'_, SrColorModeSpec> {
        SrTxCmW::new(self, 4)
    }
    #[doc = "Bit 8 - YUV input range when reg_sr_rx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn yuv_rx_range(&mut self) -> YuvRxRangeW<'_, SrColorModeSpec> {
        YuvRxRangeW::new(self, 8)
    }
    #[doc = "Bit 9 - YUV output range when reg_sr_tx_cm is 4'd8. 0: limit range. 1: full range"]
    #[inline(always)]
    pub fn yuv_tx_range(&mut self) -> YuvTxRangeW<'_, SrColorModeSpec> {
        YuvTxRangeW::new(self, 9)
    }
    #[doc = "Bit 10 - YUV to RGB protocal when reg_sr_rx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn yuv2rgb_protocal(&mut self) -> Yuv2rgbProtocalW<'_, SrColorModeSpec> {
        Yuv2rgbProtocalW::new(self, 10)
    }
    #[doc = "Bit 11 - RGB to YUV protocal when reg_sr_tx_cm is 4'd8. 0: BT601. 1: BT709"]
    #[inline(always)]
    pub fn rgb2yuv_protocal(&mut self) -> Rgb2yuvProtocalW<'_, SrColorModeSpec> {
        Rgb2yuvProtocalW::new(self, 11)
    }
    #[doc = "Bits 12:13 - YUV422 input byte order when reg_sr_rx_cm is 4'd9. 0: YVYU, 1:YUYV, 2: VYUY, 3: UYVY"]
    #[inline(always)]
    pub fn yuv422_rx_byte_order(&mut self) -> Yuv422RxByteOrderW<'_, SrColorModeSpec> {
        Yuv422RxByteOrderW::new(self, 12)
    }
}
#[doc = "Scaling and rotating engine color mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_color_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_color_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrColorModeSpec;
impl crate::RegisterSpec for SrColorModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_color_mode::R`](R) reader structure"]
impl crate::Readable for SrColorModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sr_color_mode::W`](W) writer structure"]
impl crate::Writable for SrColorModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR_COLOR_MODE to value 0"]
impl crate::Resettable for SrColorModeSpec {}
