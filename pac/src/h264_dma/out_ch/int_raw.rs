#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `OUT_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OutDoneR = crate::BitReader;
#[doc = "Field `OUT_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OutDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OutEofR = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OutEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OutDscrErrR = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OutDscrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OutTotalEofR = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` writer - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OutTotalEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L1` reader - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OutfifoOvfL1R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1` writer - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OutfifoOvfL1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L1` reader - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OutfifoUdfL1R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1` writer - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OutfifoUdfL1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L2` reader - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OutfifoOvfL2R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L2` writer - The raw interrupt bit turns to high level when fifo is overflow."]
pub type OutfifoOvfL2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L2` reader - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OutfifoUdfL2R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L2` writer - The raw interrupt bit turns to high level when fifo is underflow."]
pub type OutfifoUdfL2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_TASK_OVF` reader - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type OutDscrTaskOvfR = crate::BitReader;
#[doc = "Field `OUT_DSCR_TASK_OVF` writer - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
pub type OutDscrTaskOvfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done(&self) -> OutDoneR {
        OutDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof(&self) -> OutEofR {
        OutEofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OutDscrErrR {
        OutDscrErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OutTotalEofR {
        OutTotalEofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&self) -> OutfifoOvfL1R {
        OutfifoOvfL1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&self) -> OutfifoUdfL1R {
        OutfifoUdfL1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l2(&self) -> OutfifoOvfL2R {
        OutfifoOvfL2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l2(&self) -> OutfifoUdfL2R {
        OutfifoUdfL2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    pub fn out_dscr_task_ovf(&self) -> OutDscrTaskOvfR {
        OutDscrTaskOvfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done(&mut self) -> OutDoneW<'_, IntRawSpec> {
        OutDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OutEofW<'_, IntRawSpec> {
        OutEofW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OutDscrErrW<'_, IntRawSpec> {
        OutDscrErrW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OutTotalEofW<'_, IntRawSpec> {
        OutTotalEofW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&mut self) -> OutfifoOvfL1W<'_, IntRawSpec> {
        OutfifoOvfL1W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&mut self) -> OutfifoUdfL1W<'_, IntRawSpec> {
        OutfifoUdfL1W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt bit turns to high level when fifo is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l2(&mut self) -> OutfifoOvfL2W<'_, IntRawSpec> {
        OutfifoOvfL2W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt bit turns to high level when fifo is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l2(&mut self) -> OutfifoUdfL2W<'_, IntRawSpec> {
        OutfifoUdfL2W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt bit turns to high level when dscr ready task fifo is overflow."]
    #[inline(always)]
    pub fn out_dscr_task_ovf(&mut self) -> OutDscrTaskOvfW<'_, IntRawSpec> {
        OutDscrTaskOvfW::new(self, 8)
    }
}
#[doc = "TX CHx interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
