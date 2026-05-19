#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `OUT_DONE_CH0_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_DONE_CH0_INT"]
pub type OutDoneCh0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_EOF_CH0_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_EOF_CH0_INT"]
pub type OutEofCh0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
pub type OutDscrErrCh0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
pub type OutTotalEofCh0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_CLR` writer - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH0_INT"]
pub type OutfifoOvfCh0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_CLR` writer - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH0_INT"]
pub type OutfifoUdfCh0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_AHBINF_RESP_ERR_CH0_INT_CLR` writer - Write 1 to clear AHB_DMA_OUT_RESP_ERR_CH0_INT"]
pub type OutAhbinfRespErrCh0IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear AHB_DMA_OUT_DONE_CH0_INT"]
    #[inline(always)]
    pub fn out_done_ch0_int_clr(&mut self) -> OutDoneCh0IntClrW<'_, ClrSpec> {
        OutDoneCh0IntClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear AHB_DMA_OUT_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_eof_ch0_int_clr(&mut self) -> OutEofCh0IntClrW<'_, ClrSpec> {
        OutEofCh0IntClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear AHB_DMA_OUT_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_clr(&mut self) -> OutDscrErrCh0IntClrW<'_, ClrSpec> {
        OutDscrErrCh0IntClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear AHB_DMA_OUT_TOTAL_EOF_CH0_INT"]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_clr(&mut self) -> OutTotalEofCh0IntClrW<'_, ClrSpec> {
        OutTotalEofCh0IntClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear AHB_DMA_OUTFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_clr(&mut self) -> OutfifoOvfCh0IntClrW<'_, ClrSpec> {
        OutfifoOvfCh0IntClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear AHB_DMA_OUTFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_clr(&mut self) -> OutfifoUdfCh0IntClrW<'_, ClrSpec> {
        OutfifoUdfCh0IntClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear AHB_DMA_OUT_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn out_ahbinf_resp_err_ch0_int_clr(&mut self) -> OutAhbinfRespErrCh0IntClrW<'_, ClrSpec> {
        OutAhbinfRespErrCh0IntClrW::new(self, 6)
    }
}
#[doc = "Interrupt clear bits of TX channel 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {}
