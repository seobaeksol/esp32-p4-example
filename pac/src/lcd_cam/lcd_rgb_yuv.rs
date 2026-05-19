#[doc = "Register `LCD_RGB_YUV` reader"]
pub type R = crate::R<LcdRgbYuvSpec>;
#[doc = "Register `LCD_RGB_YUV` writer"]
pub type W = crate::W<LcdRgbYuvSpec>;
#[doc = "Field `LCD_CONV_RGB2RGB_MODE` reader - 0:rgb888 trans to rgb565. 1:rgb565 trans to rgb888.2,3:disabled"]
pub type LcdConvRgb2rgbModeR = crate::FieldReader;
#[doc = "Field `LCD_CONV_RGB2RGB_MODE` writer - 0:rgb888 trans to rgb565. 1:rgb565 trans to rgb888.2,3:disabled"]
pub type LcdConvRgb2rgbModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` reader - 1:invert every two 8bits input data. 2. disabled."]
pub type LcdConv8bitsDataInvR = crate::BitReader;
#[doc = "Field `LCD_CONV_8BITS_DATA_INV` writer - 1:invert every two 8bits input data. 2. disabled."]
pub type LcdConv8bitsDataInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_TXTORX` reader - 0: txtorx mode off. 1: txtorx mode on."]
pub type LcdConvTxtorxR = crate::BitReader;
#[doc = "Field `LCD_CONV_TXTORX` writer - 0: txtorx mode off. 1: txtorx mode on."]
pub type LcdConvTxtorxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` reader - 0: to yuv422. 2: to yuv411. 1,3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type LcdConvYuv2yuvModeR = crate::FieldReader;
#[doc = "Field `LCD_CONV_YUV2YUV_MODE` writer - 0: to yuv422. 2: to yuv411. 1,3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type LcdConvYuv2yuvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CONV_YUV_MODE` reader - 0: yuv422. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type LcdConvYuvModeR = crate::FieldReader;
#[doc = "Field `LCD_CONV_YUV_MODE` writer - 0: yuv422. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type LcdConvYuvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` reader - 0:BT601. 1:BT709."]
pub type LcdConvProtocolModeR = crate::BitReader;
#[doc = "Field `LCD_CONV_PROTOCOL_MODE` writer - 0:BT601. 1:BT709."]
pub type LcdConvProtocolModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` reader - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type LcdConvDataOutModeR = crate::BitReader;
#[doc = "Field `LCD_CONV_DATA_OUT_MODE` writer - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type LcdConvDataOutModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_DATA_IN_MODE` reader - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type LcdConvDataInModeR = crate::BitReader;
#[doc = "Field `LCD_CONV_DATA_IN_MODE` writer - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type LcdConvDataInModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` reader - 0: 16bits mode. 1: 8bits mode."]
pub type LcdConvMode8bitsOnR = crate::BitReader;
#[doc = "Field `LCD_CONV_MODE_8BITS_ON` writer - 0: 16bits mode. 1: 8bits mode."]
pub type LcdConvMode8bitsOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_TRANS_MODE` reader - 0: YUV to RGB. 1: RGB to YUV."]
pub type LcdConvTransModeR = crate::BitReader;
#[doc = "Field `LCD_CONV_TRANS_MODE` writer - 0: YUV to RGB. 1: RGB to YUV."]
pub type LcdConvTransModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CONV_ENABLE` reader - 0: Bypass converter. 1: Enable converter."]
pub type LcdConvEnableR = crate::BitReader;
#[doc = "Field `LCD_CONV_ENABLE` writer - 0: Bypass converter. 1: Enable converter."]
pub type LcdConvEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 18:19 - 0:rgb888 trans to rgb565. 1:rgb565 trans to rgb888.2,3:disabled"]
    #[inline(always)]
    pub fn lcd_conv_rgb2rgb_mode(&self) -> LcdConvRgb2rgbModeR {
        LcdConvRgb2rgbModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn lcd_conv_8bits_data_inv(&self) -> LcdConv8bitsDataInvR {
        LcdConv8bitsDataInvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 0: txtorx mode off. 1: txtorx mode on."]
    #[inline(always)]
    pub fn lcd_conv_txtorx(&self) -> LcdConvTxtorxR {
        LcdConvTxtorxR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 2: to yuv411. 1,3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn lcd_conv_yuv2yuv_mode(&self) -> LcdConvYuv2yuvModeR {
        LcdConvYuv2yuvModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn lcd_conv_yuv_mode(&self) -> LcdConvYuvModeR {
        LcdConvYuvModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn lcd_conv_protocol_mode(&self) -> LcdConvProtocolModeR {
        LcdConvProtocolModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_out_mode(&self) -> LcdConvDataOutModeR {
        LcdConvDataOutModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_in_mode(&self) -> LcdConvDataInModeR {
        LcdConvDataInModeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn lcd_conv_mode_8bits_on(&self) -> LcdConvMode8bitsOnR {
        LcdConvMode8bitsOnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn lcd_conv_trans_mode(&self) -> LcdConvTransModeR {
        LcdConvTransModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn lcd_conv_enable(&self) -> LcdConvEnableR {
        LcdConvEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:19 - 0:rgb888 trans to rgb565. 1:rgb565 trans to rgb888.2,3:disabled"]
    #[inline(always)]
    pub fn lcd_conv_rgb2rgb_mode(&mut self) -> LcdConvRgb2rgbModeW<'_, LcdRgbYuvSpec> {
        LcdConvRgb2rgbModeW::new(self, 18)
    }
    #[doc = "Bit 20 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn lcd_conv_8bits_data_inv(&mut self) -> LcdConv8bitsDataInvW<'_, LcdRgbYuvSpec> {
        LcdConv8bitsDataInvW::new(self, 20)
    }
    #[doc = "Bit 21 - 0: txtorx mode off. 1: txtorx mode on."]
    #[inline(always)]
    pub fn lcd_conv_txtorx(&mut self) -> LcdConvTxtorxW<'_, LcdRgbYuvSpec> {
        LcdConvTxtorxW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 2: to yuv411. 1,3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn lcd_conv_yuv2yuv_mode(&mut self) -> LcdConvYuv2yuvModeW<'_, LcdRgbYuvSpec> {
        LcdConvYuv2yuvModeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn lcd_conv_yuv_mode(&mut self) -> LcdConvYuvModeW<'_, LcdRgbYuvSpec> {
        LcdConvYuvModeW::new(self, 24)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn lcd_conv_protocol_mode(&mut self) -> LcdConvProtocolModeW<'_, LcdRgbYuvSpec> {
        LcdConvProtocolModeW::new(self, 26)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_out_mode(&mut self) -> LcdConvDataOutModeW<'_, LcdRgbYuvSpec> {
        LcdConvDataOutModeW::new(self, 27)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn lcd_conv_data_in_mode(&mut self) -> LcdConvDataInModeW<'_, LcdRgbYuvSpec> {
        LcdConvDataInModeW::new(self, 28)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn lcd_conv_mode_8bits_on(&mut self) -> LcdConvMode8bitsOnW<'_, LcdRgbYuvSpec> {
        LcdConvMode8bitsOnW::new(self, 29)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn lcd_conv_trans_mode(&mut self) -> LcdConvTransModeW<'_, LcdRgbYuvSpec> {
        LcdConvTransModeW::new(self, 30)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn lcd_conv_enable(&mut self) -> LcdConvEnableW<'_, LcdRgbYuvSpec> {
        LcdConvEnableW::new(self, 31)
    }
}
#[doc = "LCD YUV/RGB converter configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_rgb_yuv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_rgb_yuv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdRgbYuvSpec;
impl crate::RegisterSpec for LcdRgbYuvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_rgb_yuv::R`](R) reader structure"]
impl crate::Readable for LcdRgbYuvSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_rgb_yuv::W`](W) writer structure"]
impl crate::Writable for LcdRgbYuvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_RGB_YUV to value 0x00cc_0000"]
impl crate::Resettable for LcdRgbYuvSpec {
    const RESET_VALUE: u32 = 0x00cc_0000;
}
