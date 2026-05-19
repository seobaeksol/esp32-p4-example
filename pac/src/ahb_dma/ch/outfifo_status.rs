#[doc = "Register `OUTFIFO_STATUS` reader"]
pub type R = crate::R<OutfifoStatusSpec>;
#[doc = "Field `OUTFIFO_FULL_CH0` reader - Represents whether L1 TX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
pub type OutfifoFullCh0R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_CH0` reader - Represents whether L1 TX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
pub type OutfifoEmptyCh0R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_CH0` reader - Represents the number of data bytes in L1 TX FIFO for TX channel 0"]
pub type OutfifoCntCh0R = crate::FieldReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH0` reader - reserved"]
pub type OutRemainUnder1bCh0R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH0` reader - reserved"]
pub type OutRemainUnder2bCh0R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH0` reader - reserved"]
pub type OutRemainUnder3bCh0R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH0` reader - reserved"]
pub type OutRemainUnder4bCh0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether L1 TX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
    #[inline(always)]
    pub fn outfifo_full_ch0(&self) -> OutfifoFullCh0R {
        OutfifoFullCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether L1 TX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
    #[inline(always)]
    pub fn outfifo_empty_ch0(&self) -> OutfifoEmptyCh0R {
        OutfifoEmptyCh0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Represents the number of data bytes in L1 TX FIFO for TX channel 0"]
    #[inline(always)]
    pub fn outfifo_cnt_ch0(&self) -> OutfifoCntCh0R {
        OutfifoCntCh0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b_ch0(&self) -> OutRemainUnder1bCh0R {
        OutRemainUnder1bCh0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b_ch0(&self) -> OutRemainUnder2bCh0R {
        OutRemainUnder2bCh0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b_ch0(&self) -> OutRemainUnder3bCh0R {
        OutRemainUnder3bCh0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b_ch0(&self) -> OutRemainUnder4bCh0R {
        OutRemainUnder4bCh0R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutfifoStatusSpec;
impl crate::RegisterSpec for OutfifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status::R`](R) reader structure"]
impl crate::Readable for OutfifoStatusSpec {}
#[doc = "`reset()` method sets OUTFIFO_STATUS to value 0x0780_0002"]
impl crate::Resettable for OutfifoStatusSpec {
    const RESET_VALUE: u32 = 0x0780_0002;
}
