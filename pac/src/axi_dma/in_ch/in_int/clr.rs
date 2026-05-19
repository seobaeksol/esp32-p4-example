#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `IN_DONE` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type InDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type InSucEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
pub type InErrEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` writer - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
pub type InDscrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type InDscrEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_OVF` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type InfifoL1OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_UDF` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type InfifoL1UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_OVF` writer - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
pub type InfifoL2OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_UDF` writer - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
pub type InfifoL2UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_OVF` writer - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt."]
pub type InfifoL3OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_UDF` writer - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt."]
pub type InfifoL3UdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&mut self) -> InDoneW<'_, ClrSpec> {
        InDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> InSucEofW<'_, ClrSpec> {
        InSucEofW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> InErrEofW<'_, ClrSpec> {
        InErrEofW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> InDscrErrW<'_, ClrSpec> {
        InDscrErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> InDscrEmptyW<'_, ClrSpec> {
        InDscrEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&mut self) -> InfifoL1OvfW<'_, ClrSpec> {
        InfifoL1OvfW::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l1_udf(&mut self) -> InfifoL1UdfW<'_, ClrSpec> {
        InfifoL1UdfW::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&mut self) -> InfifoL2OvfW<'_, ClrSpec> {
        InfifoL2OvfW::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l2_udf(&mut self) -> InfifoL2UdfW<'_, ClrSpec> {
        InfifoL2UdfW::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&mut self) -> InfifoL3OvfW<'_, ClrSpec> {
        InfifoL3OvfW::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_l3_udf(&mut self) -> InfifoL3UdfW<'_, ClrSpec> {
        InfifoL3UdfW::new(self, 10)
    }
}
#[doc = "Interrupt clear bits of channel 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {}
