#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `ISP_DATA_TYPE_ERR` reader - write 1 to enable input data type error"]
pub type IspDataTypeErrR = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_ERR` writer - write 1 to enable input data type error"]
pub type IspDataTypeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_ASYNC_FIFO_OVF` reader - write 1 to enable isp input fifo overflow"]
pub type IspAsyncFifoOvfR = crate::BitReader;
#[doc = "Field `ISP_ASYNC_FIFO_OVF` writer - write 1 to enable isp input fifo overflow"]
pub type IspAsyncFifoOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_BUF_FULL` reader - write 1 to enable isp input buffer full"]
pub type IspBufFullR = crate::BitReader;
#[doc = "Field `ISP_BUF_FULL` writer - write 1 to enable isp input buffer full"]
pub type IspBufFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_HVNUM_SETTING_ERR` reader - write 1 to enable hnum and vnum setting format error"]
pub type IspHvnumSettingErrR = crate::BitReader;
#[doc = "Field `ISP_HVNUM_SETTING_ERR` writer - write 1 to enable hnum and vnum setting format error"]
pub type IspHvnumSettingErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR` reader - write 1 to enable setting invalid reg_data_type"]
pub type IspDataTypeSettingErrR = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR` writer - write 1 to enable setting invalid reg_data_type"]
pub type IspDataTypeSettingErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH` reader - write 1 to enable hnum setting unmatch with mipi input"]
pub type IspMipiHnumUnmatchR = crate::BitReader;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH` writer - write 1 to enable hnum setting unmatch with mipi input"]
pub type IspMipiHnumUnmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_CHECK_DONE` reader - write 1 to enable dpc check done"]
pub type DpcCheckDoneR = crate::BitReader;
#[doc = "Field `DPC_CHECK_DONE` writer - write 1 to enable dpc check done"]
pub type DpcCheckDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_XCOORD_ERR` reader - write 1 to enable gamma setting error"]
pub type GammaXcoordErrR = crate::BitReader;
#[doc = "Field `GAMMA_XCOORD_ERR` writer - write 1 to enable gamma setting error"]
pub type GammaXcoordErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_MONITOR` reader - write 1 to enable ae monitor"]
pub type AeMonitorR = crate::BitReader;
#[doc = "Field `AE_MONITOR` writer - write 1 to enable ae monitor"]
pub type AeMonitorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE_FRAME_DONE` reader - write 1 to enable ae"]
pub type AeFrameDoneR = crate::BitReader;
#[doc = "Field `AE_FRAME_DONE` writer - write 1 to enable ae"]
pub type AeFrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_FDONE` reader - write 1 to enable af statistic"]
pub type AfFdoneR = crate::BitReader;
#[doc = "Field `AF_FDONE` writer - write 1 to enable af statistic"]
pub type AfFdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_ENV` reader - write 1 to enable af monitor"]
pub type AfEnvR = crate::BitReader;
#[doc = "Field `AF_ENV` writer - write 1 to enable af monitor"]
pub type AfEnvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWB_FDONE` reader - write 1 to enable awb"]
pub type AwbFdoneR = crate::BitReader;
#[doc = "Field `AWB_FDONE` writer - write 1 to enable awb"]
pub type AwbFdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIST_FDONE` reader - write 1 to enable histogram"]
pub type HistFdoneR = crate::BitReader;
#[doc = "Field `HIST_FDONE` writer - write 1 to enable histogram"]
pub type HistFdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` reader - write 1 to enable isp frame end"]
pub type FrameR = crate::BitReader;
#[doc = "Field `FRAME` writer - write 1 to enable isp frame end"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLC_FRAME` reader - write 1 to enable blc frame done"]
pub type BlcFrameR = crate::BitReader;
#[doc = "Field `BLC_FRAME` writer - write 1 to enable blc frame done"]
pub type BlcFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSC_FRAME` reader - write 1 to enable lsc frame done"]
pub type LscFrameR = crate::BitReader;
#[doc = "Field `LSC_FRAME` writer - write 1 to enable lsc frame done"]
pub type LscFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_FRAME` reader - write 1 to enable dpc frame done"]
pub type DpcFrameR = crate::BitReader;
#[doc = "Field `DPC_FRAME` writer - write 1 to enable dpc frame done"]
pub type DpcFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BF_FRAME` reader - write 1 to enable bf frame done"]
pub type BfFrameR = crate::BitReader;
#[doc = "Field `BF_FRAME` writer - write 1 to enable bf frame done"]
pub type BfFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEMOSAIC_FRAME` reader - write 1 to enable demosaic frame done"]
pub type DemosaicFrameR = crate::BitReader;
#[doc = "Field `DEMOSAIC_FRAME` writer - write 1 to enable demosaic frame done"]
pub type DemosaicFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEDIAN_FRAME` reader - write 1 to enable median frame done"]
pub type MedianFrameR = crate::BitReader;
#[doc = "Field `MEDIAN_FRAME` writer - write 1 to enable median frame done"]
pub type MedianFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM_FRAME` reader - write 1 to enable ccm frame done"]
pub type CcmFrameR = crate::BitReader;
#[doc = "Field `CCM_FRAME` writer - write 1 to enable ccm frame done"]
pub type CcmFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAMMA_FRAME` reader - write 1 to enable gamma frame done"]
pub type GammaFrameR = crate::BitReader;
#[doc = "Field `GAMMA_FRAME` writer - write 1 to enable gamma frame done"]
pub type GammaFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGB2YUV_FRAME` reader - write 1 to enable rgb2yuv frame done"]
pub type Rgb2yuvFrameR = crate::BitReader;
#[doc = "Field `RGB2YUV_FRAME` writer - write 1 to enable rgb2yuv frame done"]
pub type Rgb2yuvFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARP_FRAME` reader - write 1 to enable sharp frame done"]
pub type SharpFrameR = crate::BitReader;
#[doc = "Field `SHARP_FRAME` writer - write 1 to enable sharp frame done"]
pub type SharpFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOR_FRAME` reader - write 1 to enable color frame done"]
pub type ColorFrameR = crate::BitReader;
#[doc = "Field `COLOR_FRAME` writer - write 1 to enable color frame done"]
pub type ColorFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV2RGB_FRAME` reader - write 1 to enable yuv2rgb frame done"]
pub type Yuv2rgbFrameR = crate::BitReader;
#[doc = "Field `YUV2RGB_FRAME` writer - write 1 to enable yuv2rgb frame done"]
pub type Yuv2rgbFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAIL_IDI_FRAME` reader - write 1 to enable isp_tail idi frame_end"]
pub type TailIdiFrameR = crate::BitReader;
#[doc = "Field `TAIL_IDI_FRAME` writer - write 1 to enable isp_tail idi frame_end"]
pub type TailIdiFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEADER_IDI_FRAME` reader - write 1 to enable real input frame end of isp_input"]
pub type HeaderIdiFrameR = crate::BitReader;
#[doc = "Field `HEADER_IDI_FRAME` writer - write 1 to enable real input frame end of isp_input"]
pub type HeaderIdiFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CROP_FRAME` reader - write 1 to enable crop frame done"]
pub type CropFrameR = crate::BitReader;
#[doc = "Field `CROP_FRAME` writer - write 1 to enable crop frame done"]
pub type CropFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBG_FRAME` reader - write 1 to enable wbg frame done"]
pub type WbgFrameR = crate::BitReader;
#[doc = "Field `WBG_FRAME` writer - write 1 to enable wbg frame done"]
pub type WbgFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CROP_ERR` reader - write 1 to enable crop error"]
pub type CropErrR = crate::BitReader;
#[doc = "Field `CROP_ERR` writer - write 1 to enable crop error"]
pub type CropErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to enable input data type error"]
    #[inline(always)]
    pub fn isp_data_type_err(&self) -> IspDataTypeErrR {
        IspDataTypeErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - write 1 to enable isp input fifo overflow"]
    #[inline(always)]
    pub fn isp_async_fifo_ovf(&self) -> IspAsyncFifoOvfR {
        IspAsyncFifoOvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write 1 to enable isp input buffer full"]
    #[inline(always)]
    pub fn isp_buf_full(&self) -> IspBufFullR {
        IspBufFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write 1 to enable hnum and vnum setting format error"]
    #[inline(always)]
    pub fn isp_hvnum_setting_err(&self) -> IspHvnumSettingErrR {
        IspHvnumSettingErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - write 1 to enable setting invalid reg_data_type"]
    #[inline(always)]
    pub fn isp_data_type_setting_err(&self) -> IspDataTypeSettingErrR {
        IspDataTypeSettingErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - write 1 to enable hnum setting unmatch with mipi input"]
    #[inline(always)]
    pub fn isp_mipi_hnum_unmatch(&self) -> IspMipiHnumUnmatchR {
        IspMipiHnumUnmatchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - write 1 to enable dpc check done"]
    #[inline(always)]
    pub fn dpc_check_done(&self) -> DpcCheckDoneR {
        DpcCheckDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - write 1 to enable gamma setting error"]
    #[inline(always)]
    pub fn gamma_xcoord_err(&self) -> GammaXcoordErrR {
        GammaXcoordErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - write 1 to enable ae monitor"]
    #[inline(always)]
    pub fn ae_monitor(&self) -> AeMonitorR {
        AeMonitorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - write 1 to enable ae"]
    #[inline(always)]
    pub fn ae_frame_done(&self) -> AeFrameDoneR {
        AeFrameDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - write 1 to enable af statistic"]
    #[inline(always)]
    pub fn af_fdone(&self) -> AfFdoneR {
        AfFdoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - write 1 to enable af monitor"]
    #[inline(always)]
    pub fn af_env(&self) -> AfEnvR {
        AfEnvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - write 1 to enable awb"]
    #[inline(always)]
    pub fn awb_fdone(&self) -> AwbFdoneR {
        AwbFdoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - write 1 to enable histogram"]
    #[inline(always)]
    pub fn hist_fdone(&self) -> HistFdoneR {
        HistFdoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - write 1 to enable isp frame end"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - write 1 to enable blc frame done"]
    #[inline(always)]
    pub fn blc_frame(&self) -> BlcFrameR {
        BlcFrameR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - write 1 to enable lsc frame done"]
    #[inline(always)]
    pub fn lsc_frame(&self) -> LscFrameR {
        LscFrameR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - write 1 to enable dpc frame done"]
    #[inline(always)]
    pub fn dpc_frame(&self) -> DpcFrameR {
        DpcFrameR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - write 1 to enable bf frame done"]
    #[inline(always)]
    pub fn bf_frame(&self) -> BfFrameR {
        BfFrameR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - write 1 to enable demosaic frame done"]
    #[inline(always)]
    pub fn demosaic_frame(&self) -> DemosaicFrameR {
        DemosaicFrameR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - write 1 to enable median frame done"]
    #[inline(always)]
    pub fn median_frame(&self) -> MedianFrameR {
        MedianFrameR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - write 1 to enable ccm frame done"]
    #[inline(always)]
    pub fn ccm_frame(&self) -> CcmFrameR {
        CcmFrameR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - write 1 to enable gamma frame done"]
    #[inline(always)]
    pub fn gamma_frame(&self) -> GammaFrameR {
        GammaFrameR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - write 1 to enable rgb2yuv frame done"]
    #[inline(always)]
    pub fn rgb2yuv_frame(&self) -> Rgb2yuvFrameR {
        Rgb2yuvFrameR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - write 1 to enable sharp frame done"]
    #[inline(always)]
    pub fn sharp_frame(&self) -> SharpFrameR {
        SharpFrameR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write 1 to enable color frame done"]
    #[inline(always)]
    pub fn color_frame(&self) -> ColorFrameR {
        ColorFrameR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - write 1 to enable yuv2rgb frame done"]
    #[inline(always)]
    pub fn yuv2rgb_frame(&self) -> Yuv2rgbFrameR {
        Yuv2rgbFrameR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - write 1 to enable isp_tail idi frame_end"]
    #[inline(always)]
    pub fn tail_idi_frame(&self) -> TailIdiFrameR {
        TailIdiFrameR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - write 1 to enable real input frame end of isp_input"]
    #[inline(always)]
    pub fn header_idi_frame(&self) -> HeaderIdiFrameR {
        HeaderIdiFrameR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - write 1 to enable crop frame done"]
    #[inline(always)]
    pub fn crop_frame(&self) -> CropFrameR {
        CropFrameR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - write 1 to enable wbg frame done"]
    #[inline(always)]
    pub fn wbg_frame(&self) -> WbgFrameR {
        WbgFrameR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to enable crop error"]
    #[inline(always)]
    pub fn crop_err(&self) -> CropErrR {
        CropErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to enable input data type error"]
    #[inline(always)]
    pub fn isp_data_type_err(&mut self) -> IspDataTypeErrW<'_, IntEnaSpec> {
        IspDataTypeErrW::new(self, 0)
    }
    #[doc = "Bit 1 - write 1 to enable isp input fifo overflow"]
    #[inline(always)]
    pub fn isp_async_fifo_ovf(&mut self) -> IspAsyncFifoOvfW<'_, IntEnaSpec> {
        IspAsyncFifoOvfW::new(self, 1)
    }
    #[doc = "Bit 2 - write 1 to enable isp input buffer full"]
    #[inline(always)]
    pub fn isp_buf_full(&mut self) -> IspBufFullW<'_, IntEnaSpec> {
        IspBufFullW::new(self, 2)
    }
    #[doc = "Bit 3 - write 1 to enable hnum and vnum setting format error"]
    #[inline(always)]
    pub fn isp_hvnum_setting_err(&mut self) -> IspHvnumSettingErrW<'_, IntEnaSpec> {
        IspHvnumSettingErrW::new(self, 3)
    }
    #[doc = "Bit 4 - write 1 to enable setting invalid reg_data_type"]
    #[inline(always)]
    pub fn isp_data_type_setting_err(&mut self) -> IspDataTypeSettingErrW<'_, IntEnaSpec> {
        IspDataTypeSettingErrW::new(self, 4)
    }
    #[doc = "Bit 5 - write 1 to enable hnum setting unmatch with mipi input"]
    #[inline(always)]
    pub fn isp_mipi_hnum_unmatch(&mut self) -> IspMipiHnumUnmatchW<'_, IntEnaSpec> {
        IspMipiHnumUnmatchW::new(self, 5)
    }
    #[doc = "Bit 6 - write 1 to enable dpc check done"]
    #[inline(always)]
    pub fn dpc_check_done(&mut self) -> DpcCheckDoneW<'_, IntEnaSpec> {
        DpcCheckDoneW::new(self, 6)
    }
    #[doc = "Bit 7 - write 1 to enable gamma setting error"]
    #[inline(always)]
    pub fn gamma_xcoord_err(&mut self) -> GammaXcoordErrW<'_, IntEnaSpec> {
        GammaXcoordErrW::new(self, 7)
    }
    #[doc = "Bit 8 - write 1 to enable ae monitor"]
    #[inline(always)]
    pub fn ae_monitor(&mut self) -> AeMonitorW<'_, IntEnaSpec> {
        AeMonitorW::new(self, 8)
    }
    #[doc = "Bit 9 - write 1 to enable ae"]
    #[inline(always)]
    pub fn ae_frame_done(&mut self) -> AeFrameDoneW<'_, IntEnaSpec> {
        AeFrameDoneW::new(self, 9)
    }
    #[doc = "Bit 10 - write 1 to enable af statistic"]
    #[inline(always)]
    pub fn af_fdone(&mut self) -> AfFdoneW<'_, IntEnaSpec> {
        AfFdoneW::new(self, 10)
    }
    #[doc = "Bit 11 - write 1 to enable af monitor"]
    #[inline(always)]
    pub fn af_env(&mut self) -> AfEnvW<'_, IntEnaSpec> {
        AfEnvW::new(self, 11)
    }
    #[doc = "Bit 12 - write 1 to enable awb"]
    #[inline(always)]
    pub fn awb_fdone(&mut self) -> AwbFdoneW<'_, IntEnaSpec> {
        AwbFdoneW::new(self, 12)
    }
    #[doc = "Bit 13 - write 1 to enable histogram"]
    #[inline(always)]
    pub fn hist_fdone(&mut self) -> HistFdoneW<'_, IntEnaSpec> {
        HistFdoneW::new(self, 13)
    }
    #[doc = "Bit 14 - write 1 to enable isp frame end"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<'_, IntEnaSpec> {
        FrameW::new(self, 14)
    }
    #[doc = "Bit 15 - write 1 to enable blc frame done"]
    #[inline(always)]
    pub fn blc_frame(&mut self) -> BlcFrameW<'_, IntEnaSpec> {
        BlcFrameW::new(self, 15)
    }
    #[doc = "Bit 16 - write 1 to enable lsc frame done"]
    #[inline(always)]
    pub fn lsc_frame(&mut self) -> LscFrameW<'_, IntEnaSpec> {
        LscFrameW::new(self, 16)
    }
    #[doc = "Bit 17 - write 1 to enable dpc frame done"]
    #[inline(always)]
    pub fn dpc_frame(&mut self) -> DpcFrameW<'_, IntEnaSpec> {
        DpcFrameW::new(self, 17)
    }
    #[doc = "Bit 18 - write 1 to enable bf frame done"]
    #[inline(always)]
    pub fn bf_frame(&mut self) -> BfFrameW<'_, IntEnaSpec> {
        BfFrameW::new(self, 18)
    }
    #[doc = "Bit 19 - write 1 to enable demosaic frame done"]
    #[inline(always)]
    pub fn demosaic_frame(&mut self) -> DemosaicFrameW<'_, IntEnaSpec> {
        DemosaicFrameW::new(self, 19)
    }
    #[doc = "Bit 20 - write 1 to enable median frame done"]
    #[inline(always)]
    pub fn median_frame(&mut self) -> MedianFrameW<'_, IntEnaSpec> {
        MedianFrameW::new(self, 20)
    }
    #[doc = "Bit 21 - write 1 to enable ccm frame done"]
    #[inline(always)]
    pub fn ccm_frame(&mut self) -> CcmFrameW<'_, IntEnaSpec> {
        CcmFrameW::new(self, 21)
    }
    #[doc = "Bit 22 - write 1 to enable gamma frame done"]
    #[inline(always)]
    pub fn gamma_frame(&mut self) -> GammaFrameW<'_, IntEnaSpec> {
        GammaFrameW::new(self, 22)
    }
    #[doc = "Bit 23 - write 1 to enable rgb2yuv frame done"]
    #[inline(always)]
    pub fn rgb2yuv_frame(&mut self) -> Rgb2yuvFrameW<'_, IntEnaSpec> {
        Rgb2yuvFrameW::new(self, 23)
    }
    #[doc = "Bit 24 - write 1 to enable sharp frame done"]
    #[inline(always)]
    pub fn sharp_frame(&mut self) -> SharpFrameW<'_, IntEnaSpec> {
        SharpFrameW::new(self, 24)
    }
    #[doc = "Bit 25 - write 1 to enable color frame done"]
    #[inline(always)]
    pub fn color_frame(&mut self) -> ColorFrameW<'_, IntEnaSpec> {
        ColorFrameW::new(self, 25)
    }
    #[doc = "Bit 26 - write 1 to enable yuv2rgb frame done"]
    #[inline(always)]
    pub fn yuv2rgb_frame(&mut self) -> Yuv2rgbFrameW<'_, IntEnaSpec> {
        Yuv2rgbFrameW::new(self, 26)
    }
    #[doc = "Bit 27 - write 1 to enable isp_tail idi frame_end"]
    #[inline(always)]
    pub fn tail_idi_frame(&mut self) -> TailIdiFrameW<'_, IntEnaSpec> {
        TailIdiFrameW::new(self, 27)
    }
    #[doc = "Bit 28 - write 1 to enable real input frame end of isp_input"]
    #[inline(always)]
    pub fn header_idi_frame(&mut self) -> HeaderIdiFrameW<'_, IntEnaSpec> {
        HeaderIdiFrameW::new(self, 28)
    }
    #[doc = "Bit 29 - write 1 to enable crop frame done"]
    #[inline(always)]
    pub fn crop_frame(&mut self) -> CropFrameW<'_, IntEnaSpec> {
        CropFrameW::new(self, 29)
    }
    #[doc = "Bit 30 - write 1 to enable wbg frame done"]
    #[inline(always)]
    pub fn wbg_frame(&mut self) -> WbgFrameW<'_, IntEnaSpec> {
        WbgFrameW::new(self, 30)
    }
    #[doc = "Bit 31 - write 1 to enable crop error"]
    #[inline(always)]
    pub fn crop_err(&mut self) -> CropErrW<'_, IntEnaSpec> {
        CropErrW::new(self, 31)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0xc3"]
impl crate::Resettable for IntEnaSpec {
    const RESET_VALUE: u32 = 0xc3;
}
