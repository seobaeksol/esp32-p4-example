#[doc = "Register `RX_CONF` reader"]
pub type R = crate::R<RxConfSpec>;
#[doc = "Register `RX_CONF` writer"]
pub type W = crate::W<RxConfSpec>;
#[doc = "Field `RX_RESET` writer - Set this bit to reset receiver"]
pub type RxResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RESET` writer - Set this bit to reset Rx AFIFO"]
pub type RxFifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_START` reader - Set this bit to start receiving data"]
pub type RxStartR = crate::BitReader;
#[doc = "Field `RX_START` writer - Set this bit to start receiving data"]
pub type RxStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SLAVE_MOD` reader - Set this bit to enable slave receiver mode"]
pub type RxSlaveModR = crate::BitReader;
#[doc = "Field `RX_SLAVE_MOD` writer - Set this bit to enable slave receiver mode"]
pub type RxSlaveModW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFOMEM_RESET` writer - Set this bit to reset Rx Syncfifomem"]
pub type RxFifomemResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MONO` reader - Set this bit to enable receiver in mono mode"]
pub type RxMonoR = crate::BitReader;
#[doc = "Field `RX_MONO` writer - Set this bit to enable receiver in mono mode"]
pub type RxMonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BIG_ENDIAN` reader - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
pub type RxBigEndianR = crate::BitReader;
#[doc = "Field `RX_BIG_ENDIAN` writer - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
pub type RxBigEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_UPDATE` reader - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
pub type RxUpdateR = crate::BitReader;
#[doc = "Field `RX_UPDATE` writer - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
pub type RxUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_MONO_FST_VLD` reader - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
pub type RxMonoFstVldR = crate::BitReader;
#[doc = "Field `RX_MONO_FST_VLD` writer - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
pub type RxMonoFstVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PCM_CONF` reader - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
pub type RxPcmConfR = crate::FieldReader;
#[doc = "Field `RX_PCM_CONF` writer - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
pub type RxPcmConfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_PCM_BYPASS` reader - Set this bit to bypass Compress/Decompress module for received data."]
pub type RxPcmBypassR = crate::BitReader;
#[doc = "Field `RX_PCM_BYPASS` writer - Set this bit to bypass Compress/Decompress module for received data."]
pub type RxPcmBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STOP_MODE` reader - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
pub type RxStopModeR = crate::FieldReader;
#[doc = "Field `RX_STOP_MODE` writer - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
pub type RxStopModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_LEFT_ALIGN` reader - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
pub type RxLeftAlignR = crate::BitReader;
#[doc = "Field `RX_LEFT_ALIGN` writer - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
pub type RxLeftAlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_24_FILL_EN` reader - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
pub type Rx24FillEnR = crate::BitReader;
#[doc = "Field `RX_24_FILL_EN` writer - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
pub type Rx24FillEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_WS_IDLE_POL` reader - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
pub type RxWsIdlePolR = crate::BitReader;
#[doc = "Field `RX_WS_IDLE_POL` writer - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
pub type RxWsIdlePolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BIT_ORDER` reader - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
pub type RxBitOrderR = crate::BitReader;
#[doc = "Field `RX_BIT_ORDER` writer - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
pub type RxBitOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TDM_EN` reader - 1: Enable I2S TDM Rx mode . 0: Disable."]
pub type RxTdmEnR = crate::BitReader;
#[doc = "Field `RX_TDM_EN` writer - 1: Enable I2S TDM Rx mode . 0: Disable."]
pub type RxTdmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PDM_EN` reader - 1: Enable I2S PDM Rx mode . 0: Disable."]
pub type RxPdmEnR = crate::BitReader;
#[doc = "Field `RX_PDM_EN` writer - 1: Enable I2S PDM Rx mode . 0: Disable."]
pub type RxPdmEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Set this bit to start receiving data"]
    #[inline(always)]
    pub fn rx_start(&self) -> RxStartR {
        RxStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable slave receiver mode"]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RxSlaveModR {
        RxSlaveModR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable receiver in mono mode"]
    #[inline(always)]
    pub fn rx_mono(&self) -> RxMonoR {
        RxMonoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    #[inline(always)]
    pub fn rx_big_endian(&self) -> RxBigEndianR {
        RxBigEndianR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
    #[inline(always)]
    pub fn rx_update(&self) -> RxUpdateR {
        RxUpdateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&self) -> RxMonoFstVldR {
        RxMonoFstVldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> RxPcmConfR {
        RxPcmConfR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> RxPcmBypassR {
        RxPcmBypassR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
    #[inline(always)]
    pub fn rx_stop_mode(&self) -> RxStopModeR {
        RxStopModeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
    #[inline(always)]
    pub fn rx_left_align(&self) -> RxLeftAlignR {
        RxLeftAlignR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
    #[inline(always)]
    pub fn rx_24_fill_en(&self) -> Rx24FillEnR {
        Rx24FillEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&self) -> RxWsIdlePolR {
        RxWsIdlePolR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
    #[inline(always)]
    pub fn rx_bit_order(&self) -> RxBitOrderR {
        RxBitOrderR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: Enable I2S TDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_tdm_en(&self) -> RxTdmEnR {
        RxTdmEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: Enable I2S PDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_pdm_en(&self) -> RxPdmEnR {
        RxPdmEnR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset receiver"]
    #[inline(always)]
    pub fn rx_reset(&mut self) -> RxResetW<'_, RxConfSpec> {
        RxResetW::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset Rx AFIFO"]
    #[inline(always)]
    pub fn rx_fifo_reset(&mut self) -> RxFifoResetW<'_, RxConfSpec> {
        RxFifoResetW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to start receiving data"]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RxStartW<'_, RxConfSpec> {
        RxStartW::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable slave receiver mode"]
    #[inline(always)]
    pub fn rx_slave_mod(&mut self) -> RxSlaveModW<'_, RxConfSpec> {
        RxSlaveModW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to reset Rx Syncfifomem"]
    #[inline(always)]
    pub fn rx_fifomem_reset(&mut self) -> RxFifomemResetW<'_, RxConfSpec> {
        RxFifomemResetW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable receiver in mono mode"]
    #[inline(always)]
    pub fn rx_mono(&mut self) -> RxMonoW<'_, RxConfSpec> {
        RxMonoW::new(self, 5)
    }
    #[doc = "Bit 7 - I2S Rx byte endian, 1: low addr value to high addr. 0: low addr with low addr value."]
    #[inline(always)]
    pub fn rx_big_endian(&mut self) -> RxBigEndianW<'_, RxConfSpec> {
        RxBigEndianW::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to update I2S RX registers from APB clock domain to I2S RX clock domain. This bit will be cleared by hardware after update register done."]
    #[inline(always)]
    pub fn rx_update(&mut self) -> RxUpdateW<'_, RxConfSpec> {
        RxUpdateW::new(self, 8)
    }
    #[doc = "Bit 9 - 1: The first channel data value is valid in I2S RX mono mode. 0: The second channel data value is valid in I2S RX mono mode."]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&mut self) -> RxMonoFstVldW<'_, RxConfSpec> {
        RxMonoFstVldW::new(self, 9)
    }
    #[doc = "Bits 10:11 - I2S RX compress/decompress configuration bit. & 0 (atol): A-Law decompress, 1 (ltoa) : A-Law compress, 2 (utol) : u-Law decompress, 3 (ltou) : u-Law compress. &"]
    #[inline(always)]
    pub fn rx_pcm_conf(&mut self) -> RxPcmConfW<'_, RxConfSpec> {
        RxPcmConfW::new(self, 10)
    }
    #[doc = "Bit 12 - Set this bit to bypass Compress/Decompress module for received data."]
    #[inline(always)]
    pub fn rx_pcm_bypass(&mut self) -> RxPcmBypassW<'_, RxConfSpec> {
        RxPcmBypassW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 0 : I2S Rx only stop when reg_rx_start is cleared. 1: Stop when reg_rx_start is 0 or in_suc_eof is 1. 2: Stop I2S RX when reg_rx_start is 0 or RX FIFO is full."]
    #[inline(always)]
    pub fn rx_stop_mode(&mut self) -> RxStopModeW<'_, RxConfSpec> {
        RxStopModeW::new(self, 13)
    }
    #[doc = "Bit 15 - 1: I2S RX left alignment mode. 0: I2S RX right alignment mode."]
    #[inline(always)]
    pub fn rx_left_align(&mut self) -> RxLeftAlignW<'_, RxConfSpec> {
        RxLeftAlignW::new(self, 15)
    }
    #[doc = "Bit 16 - 1: store 24 channel bits to 32 bits. 0:store 24 channel bits to 24 bits."]
    #[inline(always)]
    pub fn rx_24_fill_en(&mut self) -> Rx24FillEnW<'_, RxConfSpec> {
        Rx24FillEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 0: WS should be 0 when receiving left channel data, and WS is 1in right channel. 1: WS should be 1 when receiving left channel data, and WS is 0in right channel."]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&mut self) -> RxWsIdlePolW<'_, RxConfSpec> {
        RxWsIdlePolW::new(self, 17)
    }
    #[doc = "Bit 18 - I2S Rx bit endian. 1:small endian, the LSB is received first. 0:big endian, the MSB is received first."]
    #[inline(always)]
    pub fn rx_bit_order(&mut self) -> RxBitOrderW<'_, RxConfSpec> {
        RxBitOrderW::new(self, 18)
    }
    #[doc = "Bit 19 - 1: Enable I2S TDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_tdm_en(&mut self) -> RxTdmEnW<'_, RxConfSpec> {
        RxTdmEnW::new(self, 19)
    }
    #[doc = "Bit 20 - 1: Enable I2S PDM Rx mode . 0: Disable."]
    #[inline(always)]
    pub fn rx_pdm_en(&mut self) -> RxPdmEnW<'_, RxConfSpec> {
        RxPdmEnW::new(self, 20)
    }
}
#[doc = "I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxConfSpec;
impl crate::RegisterSpec for RxConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_conf::R`](R) reader structure"]
impl crate::Readable for RxConfSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_conf::W`](W) writer structure"]
impl crate::Writable for RxConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CONF to value 0x9600"]
impl crate::Resettable for RxConfSpec {
    const RESET_VALUE: u32 = 0x9600;
}
