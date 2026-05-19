#[doc = "Register `ENA` reader"]
pub type R = crate::R<EnaSpec>;
#[doc = "Register `ENA` writer"]
pub type W = crate::W<EnaSpec>;
#[doc = "Field `IN_DONE_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_DONE_CH0_INT"]
pub type InDoneCh0IntEnaR = crate::BitReader;
#[doc = "Field `IN_DONE_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_DONE_CH0_INT"]
pub type InDoneCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH0_INT"]
pub type InSucEofCh0IntEnaR = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH0_INT"]
pub type InSucEofCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH0_INT"]
pub type InErrEofCh0IntEnaR = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH0_INT"]
pub type InErrEofCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH0_INT"]
pub type InDscrErrCh0IntEnaR = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH0_INT"]
pub type InDscrErrCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
pub type InDscrEmptyCh0IntEnaR = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
pub type InDscrEmptyCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_INFIFO_OVF_CH0_INT"]
pub type InfifoOvfCh0IntEnaR = crate::BitReader;
#[doc = "Field `INFIFO_OVF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_INFIFO_OVF_CH0_INT"]
pub type InfifoOvfCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_INFIFO_UDF_CH0_INT"]
pub type InfifoUdfCh0IntEnaR = crate::BitReader;
#[doc = "Field `INFIFO_UDF_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_INFIFO_UDF_CH0_INT"]
pub type InfifoUdfCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH0_INT_ENA` reader - Write 1 to enable AHB_DMA_IN_RESP_ERR_CH0_INT"]
pub type InAhbinfRespErrCh0IntEnaR = crate::BitReader;
#[doc = "Field `IN_AHBINF_RESP_ERR_CH0_INT_ENA` writer - Write 1 to enable AHB_DMA_IN_RESP_ERR_CH0_INT"]
pub type InAhbinfRespErrCh0IntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_IN_DONE_CH0_INT"]
    #[inline(always)]
    pub fn in_done_ch0_int_ena(&self) -> InDoneCh0IntEnaR {
        InDoneCh0IntEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_ena(&self) -> InSucEofCh0IntEnaR {
        InSucEofCh0IntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_ena(&self) -> InErrEofCh0IntEnaR {
        InErrEofCh0IntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_ena(&self) -> InDscrErrCh0IntEnaR {
        InDscrErrCh0IntEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_ena(&self) -> InDscrEmptyCh0IntEnaR {
        InDscrEmptyCh0IntEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_INFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_ena(&self) -> InfifoOvfCh0IntEnaR {
        InfifoOvfCh0IntEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_INFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_ena(&self) -> InfifoUdfCh0IntEnaR {
        InfifoUdfCh0IntEnaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to enable AHB_DMA_IN_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch0_int_ena(&self) -> InAhbinfRespErrCh0IntEnaR {
        InAhbinfRespErrCh0IntEnaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable AHB_DMA_IN_DONE_CH0_INT"]
    #[inline(always)]
    pub fn in_done_ch0_int_ena(&mut self) -> InDoneCh0IntEnaW<'_, EnaSpec> {
        InDoneCh0IntEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable AHB_DMA_IN_SUC_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_ena(&mut self) -> InSucEofCh0IntEnaW<'_, EnaSpec> {
        InSucEofCh0IntEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable AHB_DMA_IN_ERR_EOF_CH0_INT"]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_ena(&mut self) -> InErrEofCh0IntEnaW<'_, EnaSpec> {
        InErrEofCh0IntEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable AHB_DMA_IN_DSCR_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_ena(&mut self) -> InDscrErrCh0IntEnaW<'_, EnaSpec> {
        InDscrErrCh0IntEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable AHB_DMA_IN_DSCR_EMPTY_CH0_INT"]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_ena(&mut self) -> InDscrEmptyCh0IntEnaW<'_, EnaSpec> {
        InDscrEmptyCh0IntEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable AHB_DMA_INFIFO_OVF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_ena(&mut self) -> InfifoOvfCh0IntEnaW<'_, EnaSpec> {
        InfifoOvfCh0IntEnaW::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable AHB_DMA_INFIFO_UDF_CH0_INT"]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_ena(&mut self) -> InfifoUdfCh0IntEnaW<'_, EnaSpec> {
        InfifoUdfCh0IntEnaW::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to enable AHB_DMA_IN_RESP_ERR_CH0_INT"]
    #[inline(always)]
    pub fn in_ahbinf_resp_err_ch0_int_ena(&mut self) -> InAhbinfRespErrCh0IntEnaW<'_, EnaSpec> {
        InAhbinfRespErrCh0IntEnaW::new(self, 7)
    }
}
#[doc = "Interrupt enable bits of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnaSpec;
impl crate::RegisterSpec for EnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ena::R`](R) reader structure"]
impl crate::Readable for EnaSpec {}
#[doc = "`write(|w| ..)` method takes [`ena::W`](W) writer structure"]
impl crate::Writable for EnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENA to value 0"]
impl crate::Resettable for EnaSpec {}
