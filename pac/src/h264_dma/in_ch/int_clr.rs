#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<IntClrSpec>;
#[doc = "Field `IN_DONE` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type InDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_SUC_EOF` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type InSucEofW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_ERR_EOF` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
pub type InErrEofW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` writer - Set this bit to clear the INDSCR_ERR_CH_INT interrupt."]
pub type InDscrErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type InfifoOvfL1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type InfifoUdfL1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INFIFO_OVF_L2` writer - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
pub type InfifoOvfL2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INFIFO_UDF_L2` writer - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
pub type InfifoUdfL2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type InDscrEmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_OVF` writer - Set this bit to clear the IN_DSCR_TASK_OVF_CH_INT interrupt."]
pub type InDscrTaskOvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&mut self) -> InDoneW<'_, IntClrSpec> {
        InDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> InSucEofW<'_, IntClrSpec> {
        InSucEofW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> InErrEofW<'_, IntClrSpec> {
        InErrEofW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the INDSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> InDscrErrW<'_, IntClrSpec> {
        InDscrErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&mut self) -> InfifoOvfL1W<'_, IntClrSpec> {
        InfifoOvfL1W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1(&mut self) -> InfifoUdfL1W<'_, IntClrSpec> {
        InfifoUdfL1W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l2(&mut self) -> InfifoOvfL2W<'_, IntClrSpec> {
        InfifoOvfL2W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l2(&mut self) -> InfifoUdfL2W<'_, IntClrSpec> {
        InfifoUdfL2W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> InDscrEmptyW<'_, IntClrSpec> {
        InDscrEmptyW::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the IN_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_task_ovf(&mut self) -> InDscrTaskOvfW<'_, IntClrSpec> {
        InDscrTaskOvfW::new(self, 9)
    }
}
#[doc = "RX CHx interrupt clr register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClrSpec;
impl crate::RegisterSpec for IntClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for IntClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03ff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for IntClrSpec {}
