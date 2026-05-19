#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `IN_DONE` reader - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
pub type InDoneR = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type InSucEofR = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1` reader - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type InfifoOvfL1R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type InfifoUdfL1R = crate::BitReader;
#[doc = "Field `FETCH_MB_COL_CNT_OVF` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type FetchMbColCntOvfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> InDoneR {
        InDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> InSucEofR {
        InSucEofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> InfifoOvfL1R {
        InfifoOvfL1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> InfifoUdfL1R {
        InfifoUdfL1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn fetch_mb_col_cnt_ovf(&self) -> FetchMbColCntOvfR {
        FetchMbColCntOvfR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "RX CH5 interrupt st register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
