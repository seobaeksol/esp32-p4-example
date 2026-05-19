#[doc = "Register `RAW` reader"]
pub type R = crate::R<RawSpec>;
#[doc = "Register `RAW` writer"]
pub type W = crate::W<RawSpec>;
#[doc = "Field `IN_DONE_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
pub type InDoneCh0IntRawR = crate::BitReader;
#[doc = "Field `IN_DONE_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
pub type InDoneCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
pub type InSucEofCh0IntRawR = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
pub type InSucEofCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
pub type InErrEofCh0IntRawR = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
pub type InErrEofCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
pub type InDscrErrCh0IntRawR = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
pub type InDscrErrCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
pub type InDscrEmptyCh0IntRawR = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
pub type InDscrEmptyCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
pub type InfifoOvfCh0IntRawR = crate::BitReader;
#[doc = "Field `INFIFO_OVF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
pub type InfifoOvfCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
pub type InfifoUdfCh0IntRawR = crate::BitReader;
#[doc = "Field `INFIFO_UDF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
pub type InfifoUdfCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
pub type InAhbinfRespErrCh0IntRawR = crate::BitReader;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
pub type InAhbinfRespErrCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
    #[inline(always)]
    pub fn in_done_ch0_int_raw(&self) -> InDoneCh0IntRawR {
        InDoneCh0IntRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_raw(&self) -> InSucEofCh0IntRawR {
        InSucEofCh0IntRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_raw(&self) -> InErrEofCh0IntRawR {
        InErrEofCh0IntRawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_raw(&self) -> InDscrErrCh0IntRawR {
        InDscrErrCh0IntRawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_raw(&self) -> InDscrEmptyCh0IntRawR {
        InDscrEmptyCh0IntRawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_raw(&self) -> InfifoOvfCh0IntRawR {
        InfifoOvfCh0IntRawR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_raw(&self) -> InfifoUdfCh0IntRawR {
        InfifoUdfCh0IntRawR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch0_int_raw(&self) -> InAhbinfRespErrCh0IntRawR {
        InAhbinfRespErrCh0IntRawR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_IN_DONE_CH0_INT"]
    #[inline(always)]
    pub fn in_done_ch0_int_raw(&mut self) -> InDoneCh0IntRawW<'_, RawSpec> {
        InDoneCh0IntRawW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_IN_SUC_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_raw(&mut self) -> InSucEofCh0IntRawW<'_, RawSpec> {
        InSucEofCh0IntRawW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_IN_ERR_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_raw(&mut self) -> InErrEofCh0IntRawW<'_, RawSpec> {
        InErrEofCh0IntRawW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_IN_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_raw(&mut self) -> InDscrErrCh0IntRawW<'_, RawSpec> {
        InDscrErrCh0IntRawW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_raw(&mut self) -> InDscrEmptyCh0IntRawW<'_, RawSpec> {
        InDscrEmptyCh0IntRawW::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_INFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_raw(&mut self) -> InfifoOvfCh0IntRawW<'_, RawSpec> {
        InfifoOvfCh0IntRawW::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_INFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_raw(&mut self) -> InfifoUdfCh0IntRawW<'_, RawSpec> {
        InfifoUdfCh0IntRawW::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt status of AHB_DMA_IN_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch0_int_raw(&mut self) -> InAhbinfRespErrCh0IntRawW<'_, RawSpec> {
        InAhbinfRespErrCh0IntRawW::new(self, 7)
    }
}
#[doc = "Raw interrupt status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawSpec;
impl crate::RegisterSpec for RawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw::R`](R) reader structure"]
impl crate::Readable for RawSpec {}
#[doc = "`write(|w| ..)` method takes [`raw::W`](W) writer structure"]
impl crate::Writable for RawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAW to value 0"]
impl crate::Resettable for RawSpec {}
