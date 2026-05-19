#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `IN_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
pub type InDoneR = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
pub type InDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
pub type InSucEofR = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
pub type InSucEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1` reader - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type InfifoOvfL1R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1` writer - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type InfifoOvfL1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1` reader - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type InfifoUdfL1R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1` writer - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type InfifoUdfL1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_MB_COL_CNT_OVF` reader - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type FetchMbColCntOvfR = crate::BitReader;
#[doc = "Field `FETCH_MB_COL_CNT_OVF` writer - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type FetchMbColCntOvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
    #[inline(always)]
    pub fn in_done(&self) -> InDoneR {
        InDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> InSucEofR {
        InSucEofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> InfifoOvfL1R {
        InfifoOvfL1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> InfifoUdfL1R {
        InfifoUdfL1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn fetch_mb_col_cnt_ovf(&self) -> FetchMbColCntOvfR {
        FetchMbColCntOvfR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
    #[inline(always)]
    pub fn in_done(&mut self) -> InDoneW<'_, IntRawSpec> {
        InDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> InSucEofW<'_, IntRawSpec> {
        InSucEofW::new(self, 1)
    }
    #[doc = "Bit 2 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&mut self) -> InfifoOvfL1W<'_, IntRawSpec> {
        InfifoOvfL1W::new(self, 2)
    }
    #[doc = "Bit 3 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1(&mut self) -> InfifoUdfL1W<'_, IntRawSpec> {
        InfifoUdfL1W::new(self, 3)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn fetch_mb_col_cnt_ovf(&mut self) -> FetchMbColCntOvfW<'_, IntRawSpec> {
        FetchMbColCntOvfW::new(self, 4)
    }
}
#[doc = "RX CH5 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
