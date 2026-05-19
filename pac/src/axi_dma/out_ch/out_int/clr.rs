#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `OUT_DONE` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
pub type OutDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_EOF` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
pub type OutEofW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OutDscrErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OutTotalEofW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_L1_OVF` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OutfifoL1OvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_L1_UDF` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OutfifoL1UdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_L2_OVF` writer - Set this bit to clear the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OutfifoL2OvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_L2_UDF` writer - Set this bit to clear the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OutfifoL2UdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_L3_OVF` writer - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OutfifoL3OvfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_L3_UDF` writer - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OutfifoL3UdfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_LINK_SWITCH` writer - Set this bit to clear the OUT_LINK_SWITCH_CH_INT interrupt."]
pub type OutLinkSwitchW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done(&mut self) -> OutDoneW<'_, ClrSpec> {
        OutDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OutEofW<'_, ClrSpec> {
        OutEofW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OutDscrErrW<'_, ClrSpec> {
        OutDscrErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OutTotalEofW<'_, ClrSpec> {
        OutTotalEofW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l1_ovf(&mut self) -> OutfifoL1OvfW<'_, ClrSpec> {
        OutfifoL1OvfW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l1_udf(&mut self) -> OutfifoL1UdfW<'_, ClrSpec> {
        OutfifoL1UdfW::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l2_ovf(&mut self) -> OutfifoL2OvfW<'_, ClrSpec> {
        OutfifoL2OvfW::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l2_udf(&mut self) -> OutfifoL2UdfW<'_, ClrSpec> {
        OutfifoL2UdfW::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l3_ovf(&mut self) -> OutfifoL3OvfW<'_, ClrSpec> {
        OutfifoL3OvfW::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l3_udf(&mut self) -> OutfifoL3UdfW<'_, ClrSpec> {
        OutfifoL3UdfW::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the OUT_LINK_SWITCH_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_link_switch(&mut self) -> OutLinkSwitchW<'_, ClrSpec> {
        OutLinkSwitchW::new(self, 10)
    }
}
#[doc = "Interrupt clear bits of channel0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07ff;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {}
