#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Field `ISP_DATA_TYPE_ERR` reader - the raw interrupt status of input data type error. isp only support RGB bayer data type, other type will report type_err_int"]
pub type IspDataTypeErrR = crate::BitReader;
#[doc = "Field `ISP_ASYNC_FIFO_OVF` reader - the raw interrupt status of isp input fifo overflow"]
pub type IspAsyncFifoOvfR = crate::BitReader;
#[doc = "Field `ISP_BUF_FULL` reader - the raw interrupt status of isp input buffer full"]
pub type IspBufFullR = crate::BitReader;
#[doc = "Field `ISP_HVNUM_SETTING_ERR` reader - the raw interrupt status of hnum and vnum setting format error"]
pub type IspHvnumSettingErrR = crate::BitReader;
#[doc = "Field `ISP_DATA_TYPE_SETTING_ERR` reader - the raw interrupt status of setting invalid reg_data_type"]
pub type IspDataTypeSettingErrR = crate::BitReader;
#[doc = "Field `ISP_MIPI_HNUM_UNMATCH` reader - the raw interrupt status of hnum setting unmatch with mipi input"]
pub type IspMipiHnumUnmatchR = crate::BitReader;
#[doc = "Field `DPC_CHECK_DONE` reader - the raw interrupt status of dpc check done"]
pub type DpcCheckDoneR = crate::BitReader;
#[doc = "Field `GAMMA_XCOORD_ERR` reader - the raw interrupt status of gamma setting error. it report the sum of the lengths represented by reg_gamma_x00~x0F isn't equal to 256"]
pub type GammaXcoordErrR = crate::BitReader;
#[doc = "Field `AE_MONITOR` reader - the raw interrupt status of ae monitor"]
pub type AeMonitorR = crate::BitReader;
#[doc = "Field `AE_FRAME_DONE` reader - the raw interrupt status of ae."]
pub type AeFrameDoneR = crate::BitReader;
#[doc = "Field `AF_FDONE` reader - the raw interrupt status of af statistic. when auto_update enable, each frame done will send one int pulse when manual_update, each time when write 1 to reg_manual_update will send a int pulse when next frame done"]
pub type AfFdoneR = crate::BitReader;
#[doc = "Field `AF_ENV` reader - the raw interrupt status of af monitor. send a int pulse when env_det function enabled and environment changes detected"]
pub type AfEnvR = crate::BitReader;
#[doc = "Field `AWB_FDONE` reader - the raw interrupt status of awb. send a int pulse when statistic of one awb frame done"]
pub type AwbFdoneR = crate::BitReader;
#[doc = "Field `HIST_FDONE` reader - the raw interrupt status of histogram. send a int pulse when statistic of one frame histogram done"]
pub type HistFdoneR = crate::BitReader;
#[doc = "Field `FRAME` reader - the raw interrupt status of isp frame end"]
pub type FrameR = crate::BitReader;
#[doc = "Field `BLC_FRAME` reader - the raw interrupt status of blc frame done"]
pub type BlcFrameR = crate::BitReader;
#[doc = "Field `LSC_FRAME` reader - the raw interrupt status of lsc frame done"]
pub type LscFrameR = crate::BitReader;
#[doc = "Field `DPC_FRAME` reader - the raw interrupt status of dpc frame done"]
pub type DpcFrameR = crate::BitReader;
#[doc = "Field `BF_FRAME` reader - the raw interrupt status of bf frame done"]
pub type BfFrameR = crate::BitReader;
#[doc = "Field `DEMOSAIC_FRAME` reader - the raw interrupt status of demosaic frame done"]
pub type DemosaicFrameR = crate::BitReader;
#[doc = "Field `MEDIAN_FRAME` reader - the raw interrupt status of median frame done"]
pub type MedianFrameR = crate::BitReader;
#[doc = "Field `CCM_FRAME` reader - the raw interrupt status of ccm frame done"]
pub type CcmFrameR = crate::BitReader;
#[doc = "Field `GAMMA_FRAME` reader - the raw interrupt status of gamma frame done"]
pub type GammaFrameR = crate::BitReader;
#[doc = "Field `RGB2YUV_FRAME` reader - the raw interrupt status of rgb2yuv frame done"]
pub type Rgb2yuvFrameR = crate::BitReader;
#[doc = "Field `SHARP_FRAME` reader - the raw interrupt status of sharp frame done"]
pub type SharpFrameR = crate::BitReader;
#[doc = "Field `COLOR_FRAME` reader - the raw interrupt status of color frame done"]
pub type ColorFrameR = crate::BitReader;
#[doc = "Field `YUV2RGB_FRAME` reader - the raw interrupt status of yuv2rgb frame done"]
pub type Yuv2rgbFrameR = crate::BitReader;
#[doc = "Field `TAIL_IDI_FRAME` reader - the raw interrupt status of isp_tail idi frame_end"]
pub type TailIdiFrameR = crate::BitReader;
#[doc = "Field `HEADER_IDI_FRAME` reader - the raw interrupt status of real input frame end of isp_input"]
pub type HeaderIdiFrameR = crate::BitReader;
#[doc = "Field `CROP_FRAME` reader - the raw interrupt status of crop frame done"]
pub type CropFrameR = crate::BitReader;
#[doc = "Field `WBG_FRAME` reader - the raw interrupt status of wbg frame done"]
pub type WbgFrameR = crate::BitReader;
#[doc = "Field `CROP_ERR` reader - the raw interrupt status of crop error"]
pub type CropErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of input data type error. isp only support RGB bayer data type, other type will report type_err_int"]
    #[inline(always)]
    pub fn isp_data_type_err(&self) -> IspDataTypeErrR {
        IspDataTypeErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - the raw interrupt status of isp input fifo overflow"]
    #[inline(always)]
    pub fn isp_async_fifo_ovf(&self) -> IspAsyncFifoOvfR {
        IspAsyncFifoOvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - the raw interrupt status of isp input buffer full"]
    #[inline(always)]
    pub fn isp_buf_full(&self) -> IspBufFullR {
        IspBufFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - the raw interrupt status of hnum and vnum setting format error"]
    #[inline(always)]
    pub fn isp_hvnum_setting_err(&self) -> IspHvnumSettingErrR {
        IspHvnumSettingErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the raw interrupt status of setting invalid reg_data_type"]
    #[inline(always)]
    pub fn isp_data_type_setting_err(&self) -> IspDataTypeSettingErrR {
        IspDataTypeSettingErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the raw interrupt status of hnum setting unmatch with mipi input"]
    #[inline(always)]
    pub fn isp_mipi_hnum_unmatch(&self) -> IspMipiHnumUnmatchR {
        IspMipiHnumUnmatchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the raw interrupt status of dpc check done"]
    #[inline(always)]
    pub fn dpc_check_done(&self) -> DpcCheckDoneR {
        DpcCheckDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - the raw interrupt status of gamma setting error. it report the sum of the lengths represented by reg_gamma_x00~x0F isn't equal to 256"]
    #[inline(always)]
    pub fn gamma_xcoord_err(&self) -> GammaXcoordErrR {
        GammaXcoordErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the raw interrupt status of ae monitor"]
    #[inline(always)]
    pub fn ae_monitor(&self) -> AeMonitorR {
        AeMonitorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the raw interrupt status of ae."]
    #[inline(always)]
    pub fn ae_frame_done(&self) -> AeFrameDoneR {
        AeFrameDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the raw interrupt status of af statistic. when auto_update enable, each frame done will send one int pulse when manual_update, each time when write 1 to reg_manual_update will send a int pulse when next frame done"]
    #[inline(always)]
    pub fn af_fdone(&self) -> AfFdoneR {
        AfFdoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the raw interrupt status of af monitor. send a int pulse when env_det function enabled and environment changes detected"]
    #[inline(always)]
    pub fn af_env(&self) -> AfEnvR {
        AfEnvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the raw interrupt status of awb. send a int pulse when statistic of one awb frame done"]
    #[inline(always)]
    pub fn awb_fdone(&self) -> AwbFdoneR {
        AwbFdoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the raw interrupt status of histogram. send a int pulse when statistic of one frame histogram done"]
    #[inline(always)]
    pub fn hist_fdone(&self) -> HistFdoneR {
        HistFdoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the raw interrupt status of isp frame end"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - the raw interrupt status of blc frame done"]
    #[inline(always)]
    pub fn blc_frame(&self) -> BlcFrameR {
        BlcFrameR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - the raw interrupt status of lsc frame done"]
    #[inline(always)]
    pub fn lsc_frame(&self) -> LscFrameR {
        LscFrameR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - the raw interrupt status of dpc frame done"]
    #[inline(always)]
    pub fn dpc_frame(&self) -> DpcFrameR {
        DpcFrameR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - the raw interrupt status of bf frame done"]
    #[inline(always)]
    pub fn bf_frame(&self) -> BfFrameR {
        BfFrameR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - the raw interrupt status of demosaic frame done"]
    #[inline(always)]
    pub fn demosaic_frame(&self) -> DemosaicFrameR {
        DemosaicFrameR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - the raw interrupt status of median frame done"]
    #[inline(always)]
    pub fn median_frame(&self) -> MedianFrameR {
        MedianFrameR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - the raw interrupt status of ccm frame done"]
    #[inline(always)]
    pub fn ccm_frame(&self) -> CcmFrameR {
        CcmFrameR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - the raw interrupt status of gamma frame done"]
    #[inline(always)]
    pub fn gamma_frame(&self) -> GammaFrameR {
        GammaFrameR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - the raw interrupt status of rgb2yuv frame done"]
    #[inline(always)]
    pub fn rgb2yuv_frame(&self) -> Rgb2yuvFrameR {
        Rgb2yuvFrameR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - the raw interrupt status of sharp frame done"]
    #[inline(always)]
    pub fn sharp_frame(&self) -> SharpFrameR {
        SharpFrameR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - the raw interrupt status of color frame done"]
    #[inline(always)]
    pub fn color_frame(&self) -> ColorFrameR {
        ColorFrameR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - the raw interrupt status of yuv2rgb frame done"]
    #[inline(always)]
    pub fn yuv2rgb_frame(&self) -> Yuv2rgbFrameR {
        Yuv2rgbFrameR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - the raw interrupt status of isp_tail idi frame_end"]
    #[inline(always)]
    pub fn tail_idi_frame(&self) -> TailIdiFrameR {
        TailIdiFrameR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the raw interrupt status of real input frame end of isp_input"]
    #[inline(always)]
    pub fn header_idi_frame(&self) -> HeaderIdiFrameR {
        HeaderIdiFrameR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - the raw interrupt status of crop frame done"]
    #[inline(always)]
    pub fn crop_frame(&self) -> CropFrameR {
        CropFrameR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - the raw interrupt status of wbg frame done"]
    #[inline(always)]
    pub fn wbg_frame(&self) -> WbgFrameR {
        WbgFrameR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - the raw interrupt status of crop error"]
    #[inline(always)]
    pub fn crop_err(&self) -> CropErrR {
        CropErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "raw interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
