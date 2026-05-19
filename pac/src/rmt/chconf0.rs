#[doc = "Register `CH%sCONF0` reader"]
pub type R = crate::R<Chconf0Spec>;
#[doc = "Register `CH%sCONF0` writer"]
pub type W = crate::W<Chconf0Spec>;
#[doc = "Field `TX_START_CH0` writer - Set this bit to start sending data on CHANNEL%s."]
pub type TxStartCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_RD_RST_CH0` writer - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
pub type MemRdRstCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_MEM_RST_CH0` writer - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
pub type ApbMemRstCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CONTI_MODE_CH0` reader - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
pub type TxContiModeCh0R = crate::BitReader;
#[doc = "Field `TX_CONTI_MODE_CH0` writer - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
pub type TxContiModeCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_TX_WRAP_EN_CH0` reader - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MemTxWrapEnCh0R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN_CH0` writer - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MemTxWrapEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_LV_CH0` reader - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
pub type IdleOutLvCh0R = crate::BitReader;
#[doc = "Field `IDLE_OUT_LV_CH0` writer - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
pub type IdleOutLvCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_OUT_EN_CH0` reader - This is the output enable-control bit for CHANNEL%s in IDLE state."]
pub type IdleOutEnCh0R = crate::BitReader;
#[doc = "Field `IDLE_OUT_EN_CH0` writer - This is the output enable-control bit for CHANNEL%s in IDLE state."]
pub type IdleOutEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STOP_CH0` reader - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
pub type TxStopCh0R = crate::BitReader;
#[doc = "Field `TX_STOP_CH0` writer - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
pub type TxStopCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV_CNT_CH0` reader - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DivCntCh0R = crate::FieldReader;
#[doc = "Field `DIV_CNT_CH0` writer - This register is used to configure the divider for clock of CHANNEL%s."]
pub type DivCntCh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEM_SIZE_CH0` reader - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MemSizeCh0R = crate::FieldReader;
#[doc = "Field `MEM_SIZE_CH0` writer - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
pub type MemSizeCh0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CARRIER_EFF_EN_CH0` reader - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub type CarrierEffEnCh0R = crate::BitReader;
#[doc = "Field `CARRIER_EFF_EN_CH0` writer - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
pub type CarrierEffEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_EN_CH0` reader - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CarrierEnCh0R = crate::BitReader;
#[doc = "Field `CARRIER_EN_CH0` writer - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
pub type CarrierEnCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER_OUT_LV_CH0` reader - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
pub type CarrierOutLvCh0R = crate::BitReader;
#[doc = "Field `CARRIER_OUT_LV_CH0` writer - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
pub type CarrierOutLvCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFIFO_RST_CH0` writer - Reserved"]
pub type AfifoRstCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONF_UPDATE_CH0` writer - synchronization bit for CHANNEL%s"]
pub type ConfUpdateCh0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&self) -> TxContiModeCh0R {
        TxContiModeCh0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch0(&self) -> MemTxWrapEnCh0R {
        MemTxWrapEnCh0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv_ch0(&self) -> IdleOutLvCh0R {
        IdleOutLvCh0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en_ch0(&self) -> IdleOutEnCh0R {
        IdleOutEnCh0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    pub fn tx_stop_ch0(&self) -> TxStopCh0R {
        TxStopCh0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt_ch0(&self) -> DivCntCh0R {
        DivCntCh0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size_ch0(&self) -> MemSizeCh0R {
        MemSizeCh0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    pub fn carrier_eff_en_ch0(&self) -> CarrierEffEnCh0R {
        CarrierEffEnCh0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en_ch0(&self) -> CarrierEnCh0R {
        CarrierEnCh0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch0(&self) -> CarrierOutLvCh0R {
        CarrierOutLvCh0R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to start sending data on CHANNEL%s."]
    #[inline(always)]
    pub fn tx_start_ch0(&mut self) -> TxStartCh0W<'_, Chconf0Spec> {
        TxStartCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset read ram address for CHANNEL%s by accessing transmitter."]
    #[inline(always)]
    pub fn mem_rd_rst_ch0(&mut self) -> MemRdRstCh0W<'_, Chconf0Spec> {
        MemRdRstCh0W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset W/R ram address for CHANNEL%s by accessing apb fifo."]
    #[inline(always)]
    pub fn apb_mem_rst_ch0(&mut self) -> ApbMemRstCh0W<'_, Chconf0Spec> {
        ApbMemRstCh0W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to restart transmission from the first data to the last data in CHANNEL%s."]
    #[inline(always)]
    pub fn tx_conti_mode_ch0(&mut self) -> TxContiModeCh0W<'_, Chconf0Spec> {
        TxContiModeCh0W::new(self, 3)
    }
    #[doc = "Bit 4 - This is the channel %s enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch0(&mut self) -> MemTxWrapEnCh0W<'_, Chconf0Spec> {
        MemTxWrapEnCh0W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit configures the level of output signal in CHANNEL%s when the latter is in IDLE state."]
    #[inline(always)]
    pub fn idle_out_lv_ch0(&mut self) -> IdleOutLvCh0W<'_, Chconf0Spec> {
        IdleOutLvCh0W::new(self, 5)
    }
    #[doc = "Bit 6 - This is the output enable-control bit for CHANNEL%s in IDLE state."]
    #[inline(always)]
    pub fn idle_out_en_ch0(&mut self) -> IdleOutEnCh0W<'_, Chconf0Spec> {
        IdleOutEnCh0W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to stop the transmitter of CHANNEL%s sending data out."]
    #[inline(always)]
    pub fn tx_stop_ch0(&mut self) -> TxStopCh0W<'_, Chconf0Spec> {
        TxStopCh0W::new(self, 7)
    }
    #[doc = "Bits 8:15 - This register is used to configure the divider for clock of CHANNEL%s."]
    #[inline(always)]
    pub fn div_cnt_ch0(&mut self) -> DivCntCh0W<'_, Chconf0Spec> {
        DivCntCh0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - This register is used to configure the maximum size of memory allocated to CHANNEL%s."]
    #[inline(always)]
    pub fn mem_size_ch0(&mut self) -> MemSizeCh0W<'_, Chconf0Spec> {
        MemSizeCh0W::new(self, 16)
    }
    #[doc = "Bit 20 - 1: Add carrier modulation on the output signal only at the send data state for CHANNEL%s. 0: Add carrier modulation on the output signal at all state for CHANNEL%s. Only valid when RMT_CARRIER_EN_CH%s is 1."]
    #[inline(always)]
    pub fn carrier_eff_en_ch0(&mut self) -> CarrierEffEnCh0W<'_, Chconf0Spec> {
        CarrierEffEnCh0W::new(self, 20)
    }
    #[doc = "Bit 21 - This is the carrier modulation enable-control bit for CHANNEL%s. 1: Add carrier modulation in the output signal. 0: No carrier modulation in sig_out."]
    #[inline(always)]
    pub fn carrier_en_ch0(&mut self) -> CarrierEnCh0W<'_, Chconf0Spec> {
        CarrierEnCh0W::new(self, 21)
    }
    #[doc = "Bit 22 - This bit is used to configure the position of carrier wave for CHANNEL%s.1'h0: add carrier wave on low level.1'h1: add carrier wave on high level."]
    #[inline(always)]
    pub fn carrier_out_lv_ch0(&mut self) -> CarrierOutLvCh0W<'_, Chconf0Spec> {
        CarrierOutLvCh0W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn afifo_rst_ch0(&mut self) -> AfifoRstCh0W<'_, Chconf0Spec> {
        AfifoRstCh0W::new(self, 23)
    }
    #[doc = "Bit 24 - synchronization bit for CHANNEL%s"]
    #[inline(always)]
    pub fn conf_update_ch0(&mut self) -> ConfUpdateCh0W<'_, Chconf0Spec> {
        ConfUpdateCh0W::new(self, 24)
    }
}
#[doc = "Channel %s configure register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chconf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chconf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chconf0Spec;
impl crate::RegisterSpec for Chconf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chconf0::R`](R) reader structure"]
impl crate::Readable for Chconf0Spec {}
#[doc = "`write(|w| ..)` method takes [`chconf0::W`](W) writer structure"]
impl crate::Writable for Chconf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%sCONF0 to value 0x0071_0200"]
impl crate::Resettable for Chconf0Spec {
    const RESET_VALUE: u32 = 0x0071_0200;
}
