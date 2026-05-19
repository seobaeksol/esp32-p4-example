#[doc = "Register `RAW` reader"]
pub type R = crate::R<RawSpec>;
#[doc = "Register `RAW` writer"]
pub type W = crate::W<RawSpec>;
#[doc = "Field `IN_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type InDoneR = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
pub type InDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type InSucEofR = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
pub type InSucEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
pub type InErrEofR = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
pub type InErrEofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
pub type InDscrErrR = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
pub type InDscrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
pub type InDscrEmptyR = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
pub type InDscrEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type InfifoL1OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L1_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type InfifoL1OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type InfifoL1UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L1_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type InfifoL1UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type InfifoL2OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L2_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type InfifoL2OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type InfifoL2UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L2_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type InfifoL2UdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type InfifoL3OvfR = crate::BitReader;
#[doc = "Field `INFIFO_L3_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
pub type InfifoL3OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type InfifoL3UdfR = crate::BitReader;
#[doc = "Field `INFIFO_L3_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
pub type InfifoL3UdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    pub fn in_done(&self) -> InDoneR {
        InDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> InSucEofR {
        InSucEofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> InErrEofR {
        InErrEofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> InDscrErrR {
        InDscrErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> InDscrEmptyR {
        InDscrEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&self) -> InfifoL1OvfR {
        InfifoL1OvfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_l1_udf(&self) -> InfifoL1UdfR {
        InfifoL1UdfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&self) -> InfifoL2OvfR {
        InfifoL2OvfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_l2_udf(&self) -> InfifoL2UdfR {
        InfifoL2UdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&self) -> InfifoL3OvfR {
        InfifoL3OvfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_l3_udf(&self) -> InfifoL3UdfR {
        InfifoL3UdfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0."]
    #[inline(always)]
    pub fn in_done(&mut self) -> InDoneW<'_, RawSpec> {
        InDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received for Rx channel 0. For UHCI0 the raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 0."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> InSucEofW<'_, RawSpec> {
        InSucEofW::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when data error is detected only in the case that the peripheral is UHCI0 for Rx channel 0. For other peripherals this raw interrupt is reserved."]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> InErrEofW<'_, RawSpec> {
        InErrEofW::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when detecting inlink descriptor error including owner error and the second and third word error of inlink descriptor for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> InDscrErrW<'_, RawSpec> {
        InDscrErrW::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt bit turns to high level when Rx buffer pointed by inlink is full and receiving data is not completed but there is no more inlink for Rx channel 0."]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> InDscrEmptyW<'_, RawSpec> {
        InDscrEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&mut self) -> InfifoL1OvfW<'_, RawSpec> {
        InfifoL1OvfW::new(self, 5)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_l1_udf(&mut self) -> InfifoL1UdfW<'_, RawSpec> {
        InfifoL1UdfW::new(self, 6)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&mut self) -> InfifoL2OvfW<'_, RawSpec> {
        InfifoL2OvfW::new(self, 7)
    }
    #[doc = "Bit 8 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_l2_udf(&mut self) -> InfifoL2UdfW<'_, RawSpec> {
        InfifoL2UdfW::new(self, 8)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is overflow."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&mut self) -> InfifoL3OvfW<'_, RawSpec> {
        InfifoL3OvfW::new(self, 9)
    }
    #[doc = "Bit 10 - This raw interrupt bit turns to high level when level 1 fifo of Rx channel 0 is underflow."]
    #[inline(always)]
    pub fn infifo_l3_udf(&mut self) -> InfifoL3UdfW<'_, RawSpec> {
        InfifoL3UdfW::new(self, 10)
    }
}
#[doc = "Raw status interrupt of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
