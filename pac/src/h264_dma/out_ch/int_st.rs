#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `OUT_DONE` reader - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
pub type OutDoneR = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
pub type OutEofR = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OutDscrErrR = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OutTotalEofR = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1` reader - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OutfifoOvfL1R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1` reader - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OutfifoUdfL1R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L2` reader - The raw interrupt status bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OutfifoOvfL2R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L2` reader - The raw interrupt status bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OutfifoUdfL2R = crate::BitReader;
#[doc = "Field `OUT_DSCR_TASK_OVF` reader - The raw interrupt status bit for the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
pub type OutDscrTaskOvfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done(&self) -> OutDoneR {
        OutDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof(&self) -> OutEofR {
        OutEofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OutDscrErrR {
        OutDscrErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OutTotalEofR {
        OutTotalEofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&self) -> OutfifoOvfL1R {
        OutfifoOvfL1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&self) -> OutfifoUdfL1R {
        OutfifoUdfL1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l2(&self) -> OutfifoOvfL2R {
        OutfifoOvfL2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l2(&self) -> OutfifoUdfL2R {
        OutfifoUdfL2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status bit for the OUT_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_task_ovf(&self) -> OutDscrTaskOvfR {
        OutDscrTaskOvfR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "TX CHx interrupt st register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
