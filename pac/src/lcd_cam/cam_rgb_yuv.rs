#[doc = "Register `CAM_RGB_YUV` reader"]
pub type R = crate::R<CamRgbYuvSpec>;
#[doc = "Register `CAM_RGB_YUV` writer"]
pub type W = crate::W<CamRgbYuvSpec>;
#[doc = "Field `CAM_CONV_8BITS_DATA_INV` reader - 1:invert every two 8bits input data. 2. disabled."]
pub type CamConv8bitsDataInvR = crate::BitReader;
#[doc = "Field `CAM_CONV_8BITS_DATA_INV` writer - 1:invert every two 8bits input data. 2. disabled."]
pub type CamConv8bitsDataInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_YUV2YUV_MODE` reader - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type CamConvYuv2yuvModeR = crate::FieldReader;
#[doc = "Field `CAM_CONV_YUV2YUV_MODE` writer - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
pub type CamConvYuv2yuvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAM_CONV_YUV_MODE` reader - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type CamConvYuvModeR = crate::FieldReader;
#[doc = "Field `CAM_CONV_YUV_MODE` writer - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
pub type CamConvYuvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAM_CONV_PROTOCOL_MODE` reader - 0:BT601. 1:BT709."]
pub type CamConvProtocolModeR = crate::BitReader;
#[doc = "Field `CAM_CONV_PROTOCOL_MODE` writer - 0:BT601. 1:BT709."]
pub type CamConvProtocolModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_DATA_OUT_MODE` reader - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type CamConvDataOutModeR = crate::BitReader;
#[doc = "Field `CAM_CONV_DATA_OUT_MODE` writer - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
pub type CamConvDataOutModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_DATA_IN_MODE` reader - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type CamConvDataInModeR = crate::BitReader;
#[doc = "Field `CAM_CONV_DATA_IN_MODE` writer - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
pub type CamConvDataInModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_MODE_8BITS_ON` reader - 0: 16bits mode. 1: 8bits mode."]
pub type CamConvMode8bitsOnR = crate::BitReader;
#[doc = "Field `CAM_CONV_MODE_8BITS_ON` writer - 0: 16bits mode. 1: 8bits mode."]
pub type CamConvMode8bitsOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_TRANS_MODE` reader - 0: YUV to RGB. 1: RGB to YUV."]
pub type CamConvTransModeR = crate::BitReader;
#[doc = "Field `CAM_CONV_TRANS_MODE` writer - 0: YUV to RGB. 1: RGB to YUV."]
pub type CamConvTransModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_CONV_ENABLE` reader - 0: Bypass converter. 1: Enable converter."]
pub type CamConvEnableR = crate::BitReader;
#[doc = "Field `CAM_CONV_ENABLE` writer - 0: Bypass converter. 1: Enable converter."]
pub type CamConvEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn cam_conv_8bits_data_inv(&self) -> CamConv8bitsDataInvR {
        CamConv8bitsDataInvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn cam_conv_yuv2yuv_mode(&self) -> CamConvYuv2yuvModeR {
        CamConvYuv2yuvModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn cam_conv_yuv_mode(&self) -> CamConvYuvModeR {
        CamConvYuvModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn cam_conv_protocol_mode(&self) -> CamConvProtocolModeR {
        CamConvProtocolModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn cam_conv_data_out_mode(&self) -> CamConvDataOutModeR {
        CamConvDataOutModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn cam_conv_data_in_mode(&self) -> CamConvDataInModeR {
        CamConvDataInModeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn cam_conv_mode_8bits_on(&self) -> CamConvMode8bitsOnR {
        CamConvMode8bitsOnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn cam_conv_trans_mode(&self) -> CamConvTransModeR {
        CamConvTransModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn cam_conv_enable(&self) -> CamConvEnableR {
        CamConvEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - 1:invert every two 8bits input data. 2. disabled."]
    #[inline(always)]
    pub fn cam_conv_8bits_data_inv(&mut self) -> CamConv8bitsDataInvW<'_, CamRgbYuvSpec> {
        CamConv8bitsDataInvW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 0: to yuv422. 1: to yuv420. 2: to yuv411. 3: disabled. To enable yuv2yuv mode, trans_mode must be set to 1."]
    #[inline(always)]
    pub fn cam_conv_yuv2yuv_mode(&mut self) -> CamConvYuv2yuvModeW<'_, CamRgbYuvSpec> {
        CamConvYuv2yuvModeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - 0: yuv422. 1: yuv420. 2: yuv411. When in yuv2yuv mode, yuv_mode decides the yuv mode of Data_in"]
    #[inline(always)]
    pub fn cam_conv_yuv_mode(&mut self) -> CamConvYuvModeW<'_, CamRgbYuvSpec> {
        CamConvYuvModeW::new(self, 24)
    }
    #[doc = "Bit 26 - 0:BT601. 1:BT709."]
    #[inline(always)]
    pub fn cam_conv_protocol_mode(&mut self) -> CamConvProtocolModeW<'_, CamRgbYuvSpec> {
        CamConvProtocolModeW::new(self, 26)
    }
    #[doc = "Bit 27 - LIMIT or FULL mode of Data out. 0: limit. 1: full"]
    #[inline(always)]
    pub fn cam_conv_data_out_mode(&mut self) -> CamConvDataOutModeW<'_, CamRgbYuvSpec> {
        CamConvDataOutModeW::new(self, 27)
    }
    #[doc = "Bit 28 - LIMIT or FULL mode of Data in. 0: limit. 1: full"]
    #[inline(always)]
    pub fn cam_conv_data_in_mode(&mut self) -> CamConvDataInModeW<'_, CamRgbYuvSpec> {
        CamConvDataInModeW::new(self, 28)
    }
    #[doc = "Bit 29 - 0: 16bits mode. 1: 8bits mode."]
    #[inline(always)]
    pub fn cam_conv_mode_8bits_on(&mut self) -> CamConvMode8bitsOnW<'_, CamRgbYuvSpec> {
        CamConvMode8bitsOnW::new(self, 29)
    }
    #[doc = "Bit 30 - 0: YUV to RGB. 1: RGB to YUV."]
    #[inline(always)]
    pub fn cam_conv_trans_mode(&mut self) -> CamConvTransModeW<'_, CamRgbYuvSpec> {
        CamConvTransModeW::new(self, 30)
    }
    #[doc = "Bit 31 - 0: Bypass converter. 1: Enable converter."]
    #[inline(always)]
    pub fn cam_conv_enable(&mut self) -> CamConvEnableW<'_, CamRgbYuvSpec> {
        CamConvEnableW::new(self, 31)
    }
}
#[doc = "CAM YUV/RGB converter configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_rgb_yuv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_rgb_yuv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CamRgbYuvSpec;
impl crate::RegisterSpec for CamRgbYuvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_rgb_yuv::R`](R) reader structure"]
impl crate::Readable for CamRgbYuvSpec {}
#[doc = "`write(|w| ..)` method takes [`cam_rgb_yuv::W`](W) writer structure"]
impl crate::Writable for CamRgbYuvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAM_RGB_YUV to value 0x00c0_0000"]
impl crate::Resettable for CamRgbYuvSpec {
    const RESET_VALUE: u32 = 0x00c0_0000;
}
