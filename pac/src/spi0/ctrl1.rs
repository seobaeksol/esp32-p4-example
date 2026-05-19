#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type ClkModeR = crate::FieldReader;
#[doc = "Field `CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type ClkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_AR_SIZE0_1_SUPPORT_EN` reader - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
pub type SpiArSize0_1SupportEnR = crate::BitReader;
#[doc = "Field `SPI_AR_SIZE0_1_SUPPORT_EN` writer - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
pub type SpiArSize0_1SupportEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_AW_SIZE0_1_SUPPORT_EN` reader - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
pub type SpiAwSize0_1SupportEnR = crate::BitReader;
#[doc = "Field `SPI_AW_SIZE0_1_SUPPORT_EN` writer - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
pub type SpiAwSize0_1SupportEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRESP_ECC_ERR_EN` reader - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
pub type RrespEccErrEnR = crate::BitReader;
#[doc = "Field `RRESP_ECC_ERR_EN` writer - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
pub type RrespEccErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR_SPLICE_EN` reader - Set this bit to enable AXI Read Splice-transfer."]
pub type ArSpliceEnR = crate::BitReader;
#[doc = "Field `AR_SPLICE_EN` writer - Set this bit to enable AXI Read Splice-transfer."]
pub type ArSpliceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW_SPLICE_EN` reader - Set this bit to enable AXI Write Splice-transfer."]
pub type AwSpliceEnR = crate::BitReader;
#[doc = "Field `AW_SPLICE_EN` writer - Set this bit to enable AXI Write Splice-transfer."]
pub type AwSpliceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM0_EN` reader - When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
pub type Ram0EnR = crate::BitReader;
#[doc = "Field `DUAL_RAM_EN` reader - Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
pub type DualRamEnR = crate::BitReader;
#[doc = "Field `FAST_WRITE_EN` reader - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
pub type FastWriteEnR = crate::BitReader;
#[doc = "Field `FAST_WRITE_EN` writer - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
pub type FastWriteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_RST` writer - The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO."]
pub type RxfifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_RST` writer - The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO."]
pub type TxfifoRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn clk_mode(&self) -> ClkModeR {
        ClkModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 21 - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    pub fn spi_ar_size0_1_support_en(&self) -> SpiArSize0_1SupportEnR {
        SpiArSize0_1SupportEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    pub fn spi_aw_size0_1_support_en(&self) -> SpiAwSize0_1SupportEnR {
        SpiAwSize0_1SupportEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
    #[inline(always)]
    pub fn rresp_ecc_err_en(&self) -> RrespEccErrEnR {
        RrespEccErrEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable AXI Read Splice-transfer."]
    #[inline(always)]
    pub fn ar_splice_en(&self) -> ArSpliceEnR {
        ArSpliceEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable AXI Write Splice-transfer."]
    #[inline(always)]
    pub fn aw_splice_en(&self) -> AwSpliceEnR {
        AwSpliceEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
    #[inline(always)]
    pub fn ram0_en(&self) -> Ram0EnR {
        Ram0EnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
    #[inline(always)]
    pub fn dual_ram_en(&self) -> DualRamEnR {
        DualRamEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
    #[inline(always)]
    pub fn fast_write_en(&self) -> FastWriteEnR {
        FastWriteEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> ClkModeW<'_, Ctrl1Spec> {
        ClkModeW::new(self, 0)
    }
    #[doc = "Bit 21 - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    pub fn spi_ar_size0_1_support_en(&mut self) -> SpiArSize0_1SupportEnW<'_, Ctrl1Spec> {
        SpiArSize0_1SupportEnW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    pub fn spi_aw_size0_1_support_en(&mut self) -> SpiAwSize0_1SupportEnW<'_, Ctrl1Spec> {
        SpiAwSize0_1SupportEnW::new(self, 22)
    }
    #[doc = "Bit 24 - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
    #[inline(always)]
    pub fn rresp_ecc_err_en(&mut self) -> RrespEccErrEnW<'_, Ctrl1Spec> {
        RrespEccErrEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to enable AXI Read Splice-transfer."]
    #[inline(always)]
    pub fn ar_splice_en(&mut self) -> ArSpliceEnW<'_, Ctrl1Spec> {
        ArSpliceEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to enable AXI Write Splice-transfer."]
    #[inline(always)]
    pub fn aw_splice_en(&mut self) -> AwSpliceEnW<'_, Ctrl1Spec> {
        AwSpliceEnW::new(self, 26)
    }
    #[doc = "Bit 29 - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
    #[inline(always)]
    pub fn fast_write_en(&mut self) -> FastWriteEnW<'_, Ctrl1Spec> {
        FastWriteEnW::new(self, 29)
    }
    #[doc = "Bit 30 - The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO."]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RxfifoRstW<'_, Ctrl1Spec> {
        RxfifoRstW::new(self, 30)
    }
    #[doc = "Bit 31 - The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO."]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TxfifoRstW<'_, Ctrl1Spec> {
        TxfifoRstW::new(self, 31)
    }
}
#[doc = "SPI0 control1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0x2860_0000"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0x2860_0000;
}
