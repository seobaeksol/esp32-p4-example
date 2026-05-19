#[doc = "Register `FIFO_CONF` reader"]
pub type R = crate::R<FifoConfSpec>;
#[doc = "Register `FIFO_CONF` writer"]
pub type W = crate::W<FifoConfSpec>;
#[doc = "Field `RXFIFO_WM_THRHD` reader - Configures the water mark threshold of RXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid. \\tododone{For CJ, please check this description. I habe doubt about reg_reg_fifo_prt_en.CJ: modified}"]
pub type RxfifoWmThrhdR = crate::FieldReader;
#[doc = "Field `RXFIFO_WM_THRHD` writer - Configures the water mark threshold of RXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid. \\tododone{For CJ, please check this description. I habe doubt about reg_reg_fifo_prt_en.CJ: modified}"]
pub type RxfifoWmThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TXFIFO_WM_THRHD` reader - Configures the water mark threshold of TXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and TC FIFO counter is bigger than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
pub type TxfifoWmThrhdR = crate::FieldReader;
#[doc = "Field `TXFIFO_WM_THRHD` writer - Configures the water mark threshold of TXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and TC FIFO counter is bigger than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
pub type TxfifoWmThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NONFIFO_EN` reader - Configures to enable APB nonfifo access."]
pub type NonfifoEnR = crate::BitReader;
#[doc = "Field `NONFIFO_EN` writer - Configures to enable APB nonfifo access."]
pub type NonfifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_ADDR_CFG_EN` reader - Configures the slave to enable dual address mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM. \\\\ 0: Disable\\\\ 1: Enable \\\\"]
pub type FifoAddrCfgEnR = crate::BitReader;
#[doc = "Field `FIFO_ADDR_CFG_EN` writer - Configures the slave to enable dual address mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM. \\\\ 0: Disable\\\\ 1: Enable \\\\"]
pub type FifoAddrCfgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_RST` reader - Configures to reset RXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
pub type RxFifoRstR = crate::BitReader;
#[doc = "Field `RX_FIFO_RST` writer - Configures to reset RXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
pub type RxFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_RST` reader - Configures to reset TXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
pub type TxFifoRstR = crate::BitReader;
#[doc = "Field `TX_FIFO_RST` writer - Configures to reset TXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
pub type TxFifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_PRT_EN` reader - Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type FifoPrtEnR = crate::BitReader;
#[doc = "Field `FIFO_PRT_EN` writer - Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type FifoPrtEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Configures the water mark threshold of RXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid. \\tododone{For CJ, please check this description. I habe doubt about reg_reg_fifo_prt_en.CJ: modified}"]
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&self) -> RxfifoWmThrhdR {
        RxfifoWmThrhdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Configures the water mark threshold of TXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and TC FIFO counter is bigger than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&self) -> TxfifoWmThrhdR {
        TxfifoWmThrhdR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Configures to enable APB nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&self) -> NonfifoEnR {
        NonfifoEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configures the slave to enable dual address mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM. \\\\ 0: Disable\\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&self) -> FifoAddrCfgEnR {
        FifoAddrCfgEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures to reset RXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
    #[inline(always)]
    pub fn rx_fifo_rst(&self) -> RxFifoRstR {
        RxFifoRstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures to reset TXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
    #[inline(always)]
    pub fn tx_fifo_rst(&self) -> TxFifoRstR {
        TxFifoRstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn fifo_prt_en(&self) -> FifoPrtEnR {
        FifoPrtEnR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the water mark threshold of RXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and RX FIFO counter is bigger than I2C_RXFIFO_WM_THRHD\\[4:0\\], I2C_RXFIFO_WM_INT_RAW bit will be valid. \\tododone{For CJ, please check this description. I habe doubt about reg_reg_fifo_prt_en.CJ: modified}"]
    #[inline(always)]
    pub fn rxfifo_wm_thrhd(&mut self) -> RxfifoWmThrhdW<'_, FifoConfSpec> {
        RxfifoWmThrhdW::new(self, 0)
    }
    #[doc = "Bits 5:9 - Configures the water mark threshold of TXFIFO in nonfifo access mode. When I2C_FIFO_PRT_EN is 1 and TC FIFO counter is bigger than I2C_TXFIFO_WM_THRHD\\[4:0\\], I2C_TXFIFO_WM_INT_RAW bit will be valid."]
    #[inline(always)]
    pub fn txfifo_wm_thrhd(&mut self) -> TxfifoWmThrhdW<'_, FifoConfSpec> {
        TxfifoWmThrhdW::new(self, 5)
    }
    #[doc = "Bit 10 - Configures to enable APB nonfifo access."]
    #[inline(always)]
    pub fn nonfifo_en(&mut self) -> NonfifoEnW<'_, FifoConfSpec> {
        NonfifoEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Configures the slave to enable dual address mode. When this mode is enabled, the byte received after the I2C address byte represents the offset address in the I2C Slave RAM. \\\\ 0: Disable\\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn fifo_addr_cfg_en(&mut self) -> FifoAddrCfgEnW<'_, FifoConfSpec> {
        FifoAddrCfgEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Configures to reset RXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
    #[inline(always)]
    pub fn rx_fifo_rst(&mut self) -> RxFifoRstW<'_, FifoConfSpec> {
        RxFifoRstW::new(self, 12)
    }
    #[doc = "Bit 13 - Configures to reset TXFIFO.\\\\ 0: No effect \\\\ 1: Reset"]
    #[inline(always)]
    pub fn tx_fifo_rst(&mut self) -> TxFifoRstW<'_, FifoConfSpec> {
        TxFifoRstW::new(self, 13)
    }
    #[doc = "Bit 14 - Configures to enable FIFO pointer in non-fifo access mode. This bit controls the valid bits and the TX/RX FIFO overflow, underflow, full and empty interrupts.\\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn fifo_prt_en(&mut self) -> FifoPrtEnW<'_, FifoConfSpec> {
        FifoPrtEnW::new(self, 14)
    }
}
#[doc = "FIFO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoConfSpec;
impl crate::RegisterSpec for FifoConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_conf::R`](R) reader structure"]
impl crate::Readable for FifoConfSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_conf::W`](W) writer structure"]
impl crate::Writable for FifoConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x408b"]
impl crate::Resettable for FifoConfSpec {
    const RESET_VALUE: u32 = 0x408b;
}
