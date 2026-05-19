#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FDUMMY_RIN` reader - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type FdummyRinR = crate::BitReader;
#[doc = "Field `FDUMMY_RIN` writer - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type FdummyRinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDUMMY_WOUT` reader - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type FdummyWoutR = crate::BitReader;
#[doc = "Field `FDUMMY_WOUT` writer - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type FdummyWoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOUT_OCT` reader - Apply 8 signals during write-data phase 1:enable 0: disable"]
pub type FdoutOctR = crate::BitReader;
#[doc = "Field `FDOUT_OCT` writer - Apply 8 signals during write-data phase 1:enable 0: disable"]
pub type FdoutOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDIN_OCT` reader - Apply 8 signals during read-data phase 1:enable 0: disable"]
pub type FdinOctR = crate::BitReader;
#[doc = "Field `FDIN_OCT` writer - Apply 8 signals during read-data phase 1:enable 0: disable"]
pub type FdinOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_OCT` reader - Apply 8 signals during address phase 1:enable 0: disable"]
pub type FaddrOctR = crate::BitReader;
#[doc = "Field `FADDR_OCT` writer - Apply 8 signals during address phase 1:enable 0: disable"]
pub type FaddrOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_QUAD` reader - Apply 4 signals during command phase 1:enable 0: disable"]
pub type FcmdQuadR = crate::BitReader;
#[doc = "Field `FCMD_QUAD` writer - Apply 4 signals during command phase 1:enable 0: disable"]
pub type FcmdQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_OCT` reader - Apply 8 signals during command phase 1:enable 0: disable"]
pub type FcmdOctR = crate::BitReader;
#[doc = "Field `FCMD_OCT` writer - Apply 8 signals during command phase 1:enable 0: disable"]
pub type FcmdOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCS_CRC_EN` reader - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FcsCrcEnR = crate::BitReader;
#[doc = "Field `FCS_CRC_EN` writer - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FcsCrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CRC_EN` reader - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TxCrcEnR = crate::BitReader;
#[doc = "Field `TX_CRC_EN` writer - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TxCrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTRD_MODE` reader - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub type FastrdModeR = crate::BitReader;
#[doc = "Field `FASTRD_MODE` writer - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub type FastrdModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DUAL` reader - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FreadDualR = crate::BitReader;
#[doc = "Field `FREAD_DUAL` writer - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FreadDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESANDRES` reader - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type ResandresR = crate::BitReader;
#[doc = "Field `RESANDRES` writer - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type ResandresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type QPolR = crate::BitReader;
#[doc = "Field `Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type QPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type DPolR = crate::BitReader;
#[doc = "Field `D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type DPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FreadQuadR = crate::BitReader;
#[doc = "Field `FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FreadQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type WpR = crate::BitReader;
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRSR_2B` reader - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type Wrsr2bR = crate::BitReader;
#[doc = "Field `WRSR_2B` writer - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type Wrsr2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DIO` reader - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FreadDioR = crate::BitReader;
#[doc = "Field `FREAD_DIO` writer - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FreadDioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QIO` reader - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FreadQioR = crate::BitReader;
#[doc = "Field `FREAD_QIO` writer - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FreadQioW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn fdummy_rin(&self) -> FdummyRinR {
        FdummyRinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn fdummy_wout(&self) -> FdummyWoutR {
        FdummyWoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Apply 8 signals during write-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fdout_oct(&self) -> FdoutOctR {
        FdoutOctR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Apply 8 signals during read-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fdin_oct(&self) -> FdinOctR {
        FdinOctR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Apply 8 signals during address phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn faddr_oct(&self) -> FaddrOctR {
        FaddrOctR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FcmdQuadR {
        FcmdQuadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Apply 8 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_oct(&self) -> FcmdOctR {
        FcmdOctR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    pub fn fcs_crc_en(&self) -> FcsCrcEnR {
        FcsCrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    pub fn tx_crc_en(&self) -> TxCrcEnR {
        TxCrcEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FastrdModeR {
        FastrdModeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&self) -> FreadDualR {
        FreadDualR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    pub fn resandres(&self) -> ResandresR {
        ResandresR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn q_pol(&self) -> QPolR {
        QPolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn d_pol(&self) -> DPolR {
        DPolR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FreadQuadR {
        FreadQuadR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    pub fn wrsr_2b(&self) -> Wrsr2bR {
        Wrsr2bR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&self) -> FreadDioR {
        FreadDioR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_qio(&self) -> FreadQioR {
        FreadQioR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn fdummy_rin(&mut self) -> FdummyRinW<'_, CtrlSpec> {
        FdummyRinW::new(self, 2)
    }
    #[doc = "Bit 3 - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn fdummy_wout(&mut self) -> FdummyWoutW<'_, CtrlSpec> {
        FdummyWoutW::new(self, 3)
    }
    #[doc = "Bit 4 - Apply 8 signals during write-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fdout_oct(&mut self) -> FdoutOctW<'_, CtrlSpec> {
        FdoutOctW::new(self, 4)
    }
    #[doc = "Bit 5 - Apply 8 signals during read-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fdin_oct(&mut self) -> FdinOctW<'_, CtrlSpec> {
        FdinOctW::new(self, 5)
    }
    #[doc = "Bit 6 - Apply 8 signals during address phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn faddr_oct(&mut self) -> FaddrOctW<'_, CtrlSpec> {
        FaddrOctW::new(self, 6)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_quad(&mut self) -> FcmdQuadW<'_, CtrlSpec> {
        FcmdQuadW::new(self, 8)
    }
    #[doc = "Bit 9 - Apply 8 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_oct(&mut self) -> FcmdOctW<'_, CtrlSpec> {
        FcmdOctW::new(self, 9)
    }
    #[doc = "Bit 10 - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    pub fn fcs_crc_en(&mut self) -> FcsCrcEnW<'_, CtrlSpec> {
        FcsCrcEnW::new(self, 10)
    }
    #[doc = "Bit 11 - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    pub fn tx_crc_en(&mut self) -> TxCrcEnW<'_, CtrlSpec> {
        TxCrcEnW::new(self, 11)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fastrd_mode(&mut self) -> FastrdModeW<'_, CtrlSpec> {
        FastrdModeW::new(self, 13)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&mut self) -> FreadDualW<'_, CtrlSpec> {
        FreadDualW::new(self, 14)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    pub fn resandres(&mut self) -> ResandresW<'_, CtrlSpec> {
        ResandresW::new(self, 15)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn q_pol(&mut self) -> QPolW<'_, CtrlSpec> {
        QPolW::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn d_pol(&mut self) -> DPolW<'_, CtrlSpec> {
        DPolW::new(self, 19)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&mut self) -> FreadQuadW<'_, CtrlSpec> {
        FreadQuadW::new(self, 20)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<'_, CtrlSpec> {
        WpW::new(self, 21)
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    pub fn wrsr_2b(&mut self) -> Wrsr2bW<'_, CtrlSpec> {
        Wrsr2bW::new(self, 22)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&mut self) -> FreadDioW<'_, CtrlSpec> {
        FreadDioW::new(self, 23)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_qio(&mut self) -> FreadQioW<'_, CtrlSpec> {
        FreadQioW::new(self, 24)
    }
}
#[doc = "SPI1 control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x002c_a00c"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x002c_a00c;
}
