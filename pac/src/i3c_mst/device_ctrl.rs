#[doc = "Register `DEVICE_CTRL` reader"]
pub type R = crate::R<DeviceCtrlSpec>;
#[doc = "Register `DEVICE_CTRL` writer"]
pub type W = crate::W<DeviceCtrlSpec>;
#[doc = "Field `REG_BA_INCLUDE` reader - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
pub type RegBaIncludeR = crate::BitReader;
#[doc = "Field `REG_BA_INCLUDE` writer - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
pub type RegBaIncludeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TRANS_START` reader - Transfer Start"]
pub type RegTransStartR = crate::BitReader;
#[doc = "Field `REG_TRANS_START` writer - Transfer Start"]
pub type RegTransStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CLK_EN` reader - NA"]
pub type RegClkEnR = crate::BitReader;
#[doc = "Field `REG_CLK_EN` writer - NA"]
pub type RegClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_IBI_RSTART_TRANS_EN` reader - NA"]
pub type RegIbiRstartTransEnR = crate::BitReader;
#[doc = "Field `REG_IBI_RSTART_TRANS_EN` writer - NA"]
pub type RegIbiRstartTransEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_AUTO_DIS_IBI_EN` reader - NA"]
pub type RegAutoDisIbiEnR = crate::BitReader;
#[doc = "Field `REG_AUTO_DIS_IBI_EN` writer - NA"]
pub type RegAutoDisIbiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DMA_RX_EN` reader - NA"]
pub type RegDmaRxEnR = crate::BitReader;
#[doc = "Field `REG_DMA_RX_EN` writer - NA"]
pub type RegDmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DMA_TX_EN` reader - NA"]
pub type RegDmaTxEnR = crate::BitReader;
#[doc = "Field `REG_DMA_TX_EN` writer - NA"]
pub type RegDmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MULTI_SLV_SINGLE_CCC_EN` reader - 0: rx high bit first, 1: rx low bit first"]
pub type RegMultiSlvSingleCccEnR = crate::BitReader;
#[doc = "Field `REG_MULTI_SLV_SINGLE_CCC_EN` writer - 0: rx high bit first, 1: rx low bit first"]
pub type RegMultiSlvSingleCccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RX_BIT_ORDER` reader - 0: rx low byte fist, 1: rx high byte first"]
pub type RegRxBitOrderR = crate::BitReader;
#[doc = "Field `REG_RX_BIT_ORDER` writer - 0: rx low byte fist, 1: rx high byte first"]
pub type RegRxBitOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RX_BYTE_ORDER` reader - NA"]
pub type RegRxByteOrderR = crate::BitReader;
#[doc = "Field `REG_RX_BYTE_ORDER` writer - NA"]
pub type RegRxByteOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SCL_PULLUP_FORCE_EN` reader - This bit is used to force scl_pullup_en"]
pub type RegSclPullupForceEnR = crate::BitReader;
#[doc = "Field `REG_SCL_PULLUP_FORCE_EN` writer - This bit is used to force scl_pullup_en"]
pub type RegSclPullupForceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SCL_OE_FORCE_EN` reader - This bit is used to force scl_oe"]
pub type RegSclOeForceEnR = crate::BitReader;
#[doc = "Field `REG_SCL_OE_FORCE_EN` writer - This bit is used to force scl_oe"]
pub type RegSclOeForceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDA_PP_RD_PULLUP_EN` reader - NA"]
pub type RegSdaPpRdPullupEnR = crate::BitReader;
#[doc = "Field `REG_SDA_PP_RD_PULLUP_EN` writer - NA"]
pub type RegSdaPpRdPullupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDA_RD_TBIT_HLVL_PULLUP_EN` reader - NA"]
pub type RegSdaRdTbitHlvlPullupEnR = crate::BitReader;
#[doc = "Field `REG_SDA_RD_TBIT_HLVL_PULLUP_EN` writer - NA"]
pub type RegSdaRdTbitHlvlPullupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDA_PP_WR_PULLUP_EN` reader - NA"]
pub type RegSdaPpWrPullupEnR = crate::BitReader;
#[doc = "Field `REG_SDA_PP_WR_PULLUP_EN` writer - NA"]
pub type RegSdaPpWrPullupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DATA_BYTE_CNT_UNLATCH` reader - 1: read current real-time updated value 0: read latch data byte cnt value"]
pub type RegDataByteCntUnlatchR = crate::BitReader;
#[doc = "Field `REG_DATA_BYTE_CNT_UNLATCH` writer - 1: read current real-time updated value 0: read latch data byte cnt value"]
pub type RegDataByteCntUnlatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MEM_CLK_FORCE_ON` reader - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
pub type RegMemClkForceOnR = crate::BitReader;
#[doc = "Field `REG_MEM_CLK_FORCE_ON` writer - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
pub type RegMemClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
    #[inline(always)]
    pub fn reg_ba_include(&self) -> RegBaIncludeR {
        RegBaIncludeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Start"]
    #[inline(always)]
    pub fn reg_trans_start(&self) -> RegTransStartR {
        RegTransStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_clk_en(&self) -> RegClkEnR {
        RegClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_ibi_rstart_trans_en(&self) -> RegIbiRstartTransEnR {
        RegIbiRstartTransEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_auto_dis_ibi_en(&self) -> RegAutoDisIbiEnR {
        RegAutoDisIbiEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn reg_dma_rx_en(&self) -> RegDmaRxEnR {
        RegDmaRxEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn reg_dma_tx_en(&self) -> RegDmaTxEnR {
        RegDmaTxEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: rx high bit first, 1: rx low bit first"]
    #[inline(always)]
    pub fn reg_multi_slv_single_ccc_en(&self) -> RegMultiSlvSingleCccEnR {
        RegMultiSlvSingleCccEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: rx low byte fist, 1: rx high byte first"]
    #[inline(always)]
    pub fn reg_rx_bit_order(&self) -> RegRxBitOrderR {
        RegRxBitOrderR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn reg_rx_byte_order(&self) -> RegRxByteOrderR {
        RegRxByteOrderR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is used to force scl_pullup_en"]
    #[inline(always)]
    pub fn reg_scl_pullup_force_en(&self) -> RegSclPullupForceEnR {
        RegSclPullupForceEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is used to force scl_oe"]
    #[inline(always)]
    pub fn reg_scl_oe_force_en(&self) -> RegSclOeForceEnR {
        RegSclOeForceEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn reg_sda_pp_rd_pullup_en(&self) -> RegSdaPpRdPullupEnR {
        RegSdaPpRdPullupEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn reg_sda_rd_tbit_hlvl_pullup_en(&self) -> RegSdaRdTbitHlvlPullupEnR {
        RegSdaRdTbitHlvlPullupEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn reg_sda_pp_wr_pullup_en(&self) -> RegSdaPpWrPullupEnR {
        RegSdaPpWrPullupEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: read current real-time updated value 0: read latch data byte cnt value"]
    #[inline(always)]
    pub fn reg_data_byte_cnt_unlatch(&self) -> RegDataByteCntUnlatchR {
        RegDataByteCntUnlatchR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
    #[inline(always)]
    pub fn reg_mem_clk_force_on(&self) -> RegMemClkForceOnR {
        RegMemClkForceOnR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
    #[inline(always)]
    pub fn reg_ba_include(&mut self) -> RegBaIncludeW<'_, DeviceCtrlSpec> {
        RegBaIncludeW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Start"]
    #[inline(always)]
    pub fn reg_trans_start(&mut self) -> RegTransStartW<'_, DeviceCtrlSpec> {
        RegTransStartW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_clk_en(&mut self) -> RegClkEnW<'_, DeviceCtrlSpec> {
        RegClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_ibi_rstart_trans_en(&mut self) -> RegIbiRstartTransEnW<'_, DeviceCtrlSpec> {
        RegIbiRstartTransEnW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_auto_dis_ibi_en(&mut self) -> RegAutoDisIbiEnW<'_, DeviceCtrlSpec> {
        RegAutoDisIbiEnW::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn reg_dma_rx_en(&mut self) -> RegDmaRxEnW<'_, DeviceCtrlSpec> {
        RegDmaRxEnW::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn reg_dma_tx_en(&mut self) -> RegDmaTxEnW<'_, DeviceCtrlSpec> {
        RegDmaTxEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 0: rx high bit first, 1: rx low bit first"]
    #[inline(always)]
    pub fn reg_multi_slv_single_ccc_en(&mut self) -> RegMultiSlvSingleCccEnW<'_, DeviceCtrlSpec> {
        RegMultiSlvSingleCccEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 0: rx low byte fist, 1: rx high byte first"]
    #[inline(always)]
    pub fn reg_rx_bit_order(&mut self) -> RegRxBitOrderW<'_, DeviceCtrlSpec> {
        RegRxBitOrderW::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn reg_rx_byte_order(&mut self) -> RegRxByteOrderW<'_, DeviceCtrlSpec> {
        RegRxByteOrderW::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is used to force scl_pullup_en"]
    #[inline(always)]
    pub fn reg_scl_pullup_force_en(&mut self) -> RegSclPullupForceEnW<'_, DeviceCtrlSpec> {
        RegSclPullupForceEnW::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is used to force scl_oe"]
    #[inline(always)]
    pub fn reg_scl_oe_force_en(&mut self) -> RegSclOeForceEnW<'_, DeviceCtrlSpec> {
        RegSclOeForceEnW::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn reg_sda_pp_rd_pullup_en(&mut self) -> RegSdaPpRdPullupEnW<'_, DeviceCtrlSpec> {
        RegSdaPpRdPullupEnW::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn reg_sda_rd_tbit_hlvl_pullup_en(
        &mut self,
    ) -> RegSdaRdTbitHlvlPullupEnW<'_, DeviceCtrlSpec> {
        RegSdaRdTbitHlvlPullupEnW::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn reg_sda_pp_wr_pullup_en(&mut self) -> RegSdaPpWrPullupEnW<'_, DeviceCtrlSpec> {
        RegSdaPpWrPullupEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 1: read current real-time updated value 0: read latch data byte cnt value"]
    #[inline(always)]
    pub fn reg_data_byte_cnt_unlatch(&mut self) -> RegDataByteCntUnlatchW<'_, DeviceCtrlSpec> {
        RegDataByteCntUnlatchW::new(self, 16)
    }
    #[doc = "Bit 17 - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
    #[inline(always)]
    pub fn reg_mem_clk_force_on(&mut self) -> RegMemClkForceOnW<'_, DeviceCtrlSpec> {
        RegMemClkForceOnW::new(self, 17)
    }
}
#[doc = "DEVICE_CTRL register controls the transfer properties and disposition of controllers capabilities.\n\nYou can [`read`](crate::Reg::read) this register and get [`device_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`device_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceCtrlSpec;
impl crate::RegisterSpec for DeviceCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_ctrl::R`](R) reader structure"]
impl crate::Readable for DeviceCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`device_ctrl::W`](W) writer structure"]
impl crate::Writable for DeviceCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVICE_CTRL to value 0x1020"]
impl crate::Resettable for DeviceCtrlSpec {
    const RESET_VALUE: u32 = 0x1020;
}
