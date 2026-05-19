#[doc = "Register `CH%s_TX_LIM` reader"]
pub type R = crate::R<ChTxLimSpec>;
#[doc = "Register `CH%s_TX_LIM` writer"]
pub type W = crate::W<ChTxLimSpec>;
#[doc = "Field `TX_LIM_CH` reader - This register is used to configure the maximum entries that CHANNEL%s can send out."]
pub type TxLimChR = crate::FieldReader<u16>;
#[doc = "Field `TX_LIM_CH` writer - This register is used to configure the maximum entries that CHANNEL%s can send out."]
pub type TxLimChW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TX_LOOP_NUM_CH` reader - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
pub type TxLoopNumChR = crate::FieldReader<u16>;
#[doc = "Field `TX_LOOP_NUM_CH` writer - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
pub type TxLoopNumChW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_LOOP_CNT_EN_CH` reader - This register is the enabled bit for loop count."]
pub type TxLoopCntEnChR = crate::BitReader;
#[doc = "Field `TX_LOOP_CNT_EN_CH` writer - This register is the enabled bit for loop count."]
pub type TxLoopCntEnChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP_COUNT_RESET_CH` writer - This register is used to reset the loop count when tx_conti_mode is valid."]
pub type LoopCountResetChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP_STOP_EN_CH` reader - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
pub type LoopStopEnChR = crate::BitReader;
#[doc = "Field `LOOP_STOP_EN_CH` writer - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
pub type LoopStopEnChW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out."]
    #[inline(always)]
    pub fn tx_lim_ch(&self) -> TxLimChR {
        TxLimChR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
    #[inline(always)]
    pub fn tx_loop_num_ch(&self) -> TxLoopNumChR {
        TxLoopNumChR::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - This register is the enabled bit for loop count."]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch(&self) -> TxLoopCntEnChR {
        TxLoopCntEnChR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
    #[inline(always)]
    pub fn loop_stop_en_ch(&self) -> LoopStopEnChR {
        LoopStopEnChR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure the maximum entries that CHANNEL%s can send out."]
    #[inline(always)]
    pub fn tx_lim_ch(&mut self) -> TxLimChW<'_, ChTxLimSpec> {
        TxLimChW::new(self, 0)
    }
    #[doc = "Bits 9:18 - This register is used to configure the maximum loop count when tx_conti_mode is valid."]
    #[inline(always)]
    pub fn tx_loop_num_ch(&mut self) -> TxLoopNumChW<'_, ChTxLimSpec> {
        TxLoopNumChW::new(self, 9)
    }
    #[doc = "Bit 19 - This register is the enabled bit for loop count."]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch(&mut self) -> TxLoopCntEnChW<'_, ChTxLimSpec> {
        TxLoopCntEnChW::new(self, 19)
    }
    #[doc = "Bit 20 - This register is used to reset the loop count when tx_conti_mode is valid."]
    #[inline(always)]
    pub fn loop_count_reset_ch(&mut self) -> LoopCountResetChW<'_, ChTxLimSpec> {
        LoopCountResetChW::new(self, 20)
    }
    #[doc = "Bit 21 - This bit is used to enable the loop send stop function after the loop counter counts to loop number for CHANNEL%s."]
    #[inline(always)]
    pub fn loop_stop_en_ch(&mut self) -> LoopStopEnChW<'_, ChTxLimSpec> {
        LoopStopEnChW::new(self, 21)
    }
}
#[doc = "Channel %s Tx event configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_tx_lim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_tx_lim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChTxLimSpec;
impl crate::RegisterSpec for ChTxLimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_tx_lim::R`](R) reader structure"]
impl crate::Readable for ChTxLimSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_tx_lim::W`](W) writer structure"]
impl crate::Writable for ChTxLimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_TX_LIM to value 0x80"]
impl crate::Resettable for ChTxLimSpec {
    const RESET_VALUE: u32 = 0x80;
}
