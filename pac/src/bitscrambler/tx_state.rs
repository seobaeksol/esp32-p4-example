#[doc = "Register `TX_STATE` reader"]
pub type R = crate::R<TxStateSpec>;
#[doc = "Register `TX_STATE` writer"]
pub type W = crate::W<TxStateSpec>;
#[doc = "Field `TX_IN_IDLE` reader - represents the bitscrambler tx core in halt mode"]
pub type TxInIdleR = crate::BitReader;
#[doc = "Field `TX_IN_RUN` reader - represents the bitscrambler tx core in run mode"]
pub type TxInRunR = crate::BitReader;
#[doc = "Field `TX_IN_WAIT` reader - represents the bitscrambler tx core in wait mode to wait write back done"]
pub type TxInWaitR = crate::BitReader;
#[doc = "Field `TX_IN_PAUSE` reader - represents the bitscrambler tx core in pause mode"]
pub type TxInPauseR = crate::BitReader;
#[doc = "Field `TX_FIFO_EMPTY` reader - represents the bitscrambler tx fifo in empty state"]
pub type TxFifoEmptyR = crate::BitReader;
#[doc = "Field `TX_EOF_GET_CNT` reader - represents the bytes numbers of bitscrambler tx core when get EOF"]
pub type TxEofGetCntR = crate::FieldReader<u16>;
#[doc = "Field `TX_EOF_OVERLOAD` reader - represents the some EOFs will be lost for bitscrambler tx core"]
pub type TxEofOverloadR = crate::BitReader;
#[doc = "Field `TX_EOF_TRACE_CLR` writer - write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers"]
pub type TxEofTraceClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - represents the bitscrambler tx core in halt mode"]
    #[inline(always)]
    pub fn tx_in_idle(&self) -> TxInIdleR {
        TxInIdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - represents the bitscrambler tx core in run mode"]
    #[inline(always)]
    pub fn tx_in_run(&self) -> TxInRunR {
        TxInRunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - represents the bitscrambler tx core in wait mode to wait write back done"]
    #[inline(always)]
    pub fn tx_in_wait(&self) -> TxInWaitR {
        TxInWaitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - represents the bitscrambler tx core in pause mode"]
    #[inline(always)]
    pub fn tx_in_pause(&self) -> TxInPauseR {
        TxInPauseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - represents the bitscrambler tx fifo in empty state"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TxFifoEmptyR {
        TxFifoEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:29 - represents the bytes numbers of bitscrambler tx core when get EOF"]
    #[inline(always)]
    pub fn tx_eof_get_cnt(&self) -> TxEofGetCntR {
        TxEofGetCntR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - represents the some EOFs will be lost for bitscrambler tx core"]
    #[inline(always)]
    pub fn tx_eof_overload(&self) -> TxEofOverloadR {
        TxEofOverloadR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers"]
    #[inline(always)]
    pub fn tx_eof_trace_clr(&mut self) -> TxEofTraceClrW<'_, TxStateSpec> {
        TxEofTraceClrW::new(self, 31)
    }
}
#[doc = "Status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxStateSpec;
impl crate::RegisterSpec for TxStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_state::R`](R) reader structure"]
impl crate::Readable for TxStateSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_state::W`](W) writer structure"]
impl crate::Writable for TxStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_STATE to value 0x01"]
impl crate::Resettable for TxStateSpec {
    const RESET_VALUE: u32 = 0x01;
}
