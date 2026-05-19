#[doc = "Register `RAW` reader"]
pub type R = crate::R<RawSpec>;
#[doc = "Register `RAW` writer"]
pub type W = crate::W<RawSpec>;
#[doc = "Field `OUT_DONE_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
pub type OutDoneCh0IntRawR = crate::BitReader;
#[doc = "Field `OUT_DONE_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
pub type OutDoneCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
pub type OutEofCh0IntRawR = crate::BitReader;
#[doc = "Field `OUT_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
pub type OutEofCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
pub type OutDscrErrCh0IntRawR = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
pub type OutDscrErrCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
pub type OutTotalEofCh0IntRawR = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
pub type OutTotalEofCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
pub type OutfifoOvfCh0IntRawR = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
pub type OutfifoOvfCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
pub type OutfifoUdfCh0IntRawR = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
pub type OutfifoUdfCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH0_INT_RAW` reader - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
pub type OutAhbinfRespErrCh0IntRawR = crate::BitReader;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH0_INT_RAW` writer - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
pub type OutAhbinfRespErrCh0IntRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
    #[inline(always)]
    pub fn out_done_ch0_int_raw(&self) -> OutDoneCh0IntRawR {
        OutDoneCh0IntRawR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_eof_ch0_int_raw(&self) -> OutEofCh0IntRawR {
        OutEofCh0IntRawR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_raw(&self) -> OutDscrErrCh0IntRawR {
        OutDscrErrCh0IntRawR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_raw(&self) -> OutTotalEofCh0IntRawR {
        OutTotalEofCh0IntRawR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_raw(&self) -> OutfifoOvfCh0IntRawR {
        OutfifoOvfCh0IntRawR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_raw(&self) -> OutfifoUdfCh0IntRawR {
        OutfifoUdfCh0IntRawR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch0_int_raw(&self) -> OutAhbinfRespErrCh0IntRawR {
        OutAhbinfRespErrCh0IntRawR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of AHB_DMA_OUT_DONE_CH0_INT"]
    #[inline(always)]
    pub fn out_done_ch0_int_raw(&mut self) -> OutDoneCh0IntRawW<'_, RawSpec> {
        OutDoneCh0IntRawW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of AHB_DMA_OUT_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_eof_ch0_int_raw(&mut self) -> OutEofCh0IntRawW<'_, RawSpec> {
        OutEofCh0IntRawW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_raw(&mut self) -> OutDscrErrCh0IntRawW<'_, RawSpec> {
        OutDscrErrCh0IntRawW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_raw(&mut self) -> OutTotalEofCh0IntRawW<'_, RawSpec> {
        OutTotalEofCh0IntRawW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of AHB_DMA_OUTFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_raw(&mut self) -> OutfifoOvfCh0IntRawW<'_, RawSpec> {
        OutfifoOvfCh0IntRawW::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of AHB_DMA_OUTFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_raw(&mut self) -> OutfifoUdfCh0IntRawW<'_, RawSpec> {
        OutfifoUdfCh0IntRawW::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of AHB_DMA_OUT_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch0_int_raw(&mut self) -> OutAhbinfRespErrCh0IntRawW<'_, RawSpec> {
        OutAhbinfRespErrCh0IntRawW::new(self, 6)
    }
}
#[doc = "//Raw interrupt status of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
