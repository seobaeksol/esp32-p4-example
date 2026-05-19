#[doc = "Register `SRAM_CMD` reader"]
pub type R = crate::R<SramCmdSpec>;
#[doc = "Register `SRAM_CMD` writer"]
pub type W = crate::W<SramCmdSpec>;
#[doc = "Field `SCLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
pub type SclkModeR = crate::FieldReader;
#[doc = "Field `SCLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
pub type SclkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWB_MODE` reader - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SwbModeR = crate::FieldReader;
#[doc = "Field `SWB_MODE` writer - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SwbModeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDIN_DUAL` reader - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SdinDualR = crate::BitReader;
#[doc = "Field `SDIN_DUAL` writer - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SdinDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_DUAL` reader - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SdoutDualR = crate::BitReader;
#[doc = "Field `SDOUT_DUAL` writer - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SdoutDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_DUAL` reader - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SaddrDualR = crate::BitReader;
#[doc = "Field `SADDR_DUAL` writer - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SaddrDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_QUAD` reader - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SdinQuadR = crate::BitReader;
#[doc = "Field `SDIN_QUAD` writer - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SdinQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_QUAD` reader - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SdoutQuadR = crate::BitReader;
#[doc = "Field `SDOUT_QUAD` writer - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SdoutQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_QUAD` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SaddrQuadR = crate::BitReader;
#[doc = "Field `SADDR_QUAD` writer - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SaddrQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMD_QUAD` reader - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type ScmdQuadR = crate::BitReader;
#[doc = "Field `SCMD_QUAD` writer - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type ScmdQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_OCT` reader - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
pub type SdinOctR = crate::BitReader;
#[doc = "Field `SDIN_OCT` writer - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
pub type SdinOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_OCT` reader - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
pub type SdoutOctR = crate::BitReader;
#[doc = "Field `SDOUT_OCT` writer - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
pub type SdoutOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_OCT` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
pub type SaddrOctR = crate::BitReader;
#[doc = "Field `SADDR_OCT` writer - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
pub type SaddrOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMD_OCT` reader - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
pub type ScmdOctR = crate::BitReader;
#[doc = "Field `SCMD_OCT` writer - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
pub type ScmdOctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDUMMY_RIN` reader - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SdummyRinR = crate::BitReader;
#[doc = "Field `SDUMMY_RIN` writer - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SdummyRinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDUMMY_WOUT` reader - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SdummyWoutR = crate::BitReader;
#[doc = "Field `SDUMMY_WOUT` writer - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SdummyWoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
pub type SpiSmemWdummyDqsAlwaysOutR = crate::BitReader;
#[doc = "Field `SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
pub type SpiSmemWdummyDqsAlwaysOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_WDUMMY_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SpiSmemWdummyAlwaysOutR = crate::BitReader;
#[doc = "Field `SPI_SMEM_WDUMMY_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SpiSmemWdummyAlwaysOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_HEX` reader - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
pub type SdinHexR = crate::BitReader;
#[doc = "Field `SDIN_HEX` writer - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
pub type SdinHexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_HEX` reader - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
pub type SdoutHexR = crate::BitReader;
#[doc = "Field `SDOUT_HEX` writer - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
pub type SdoutHexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_DQS_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SpiSmemDqsIeAlwaysOnR = crate::BitReader;
#[doc = "Field `SPI_SMEM_DQS_IE_ALWAYS_ON` writer - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SpiSmemDqsIeAlwaysOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_DATA_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SpiSmemDataIeAlwaysOnR = crate::BitReader;
#[doc = "Field `SPI_SMEM_DATA_IE_ALWAYS_ON` writer - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SpiSmemDataIeAlwaysOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
    #[inline(always)]
    pub fn sclk_mode(&self) -> SclkModeR {
        SclkModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn swb_mode(&self) -> SwbModeR {
        SwbModeR::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdin_dual(&self) -> SdinDualR {
        SdinDualR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdout_dual(&self) -> SdoutDualR {
        SdoutDualR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn saddr_dual(&self) -> SaddrDualR {
        SaddrDualR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdin_quad(&self) -> SdinQuadR {
        SdinQuadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdout_quad(&self) -> SdoutQuadR {
        SdoutQuadR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn saddr_quad(&self) -> SaddrQuadR {
        SaddrQuadR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn scmd_quad(&self) -> ScmdQuadR {
        ScmdQuadR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_oct(&self) -> SdinOctR {
        SdinOctR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_oct(&self) -> SdoutOctR {
        SdoutOctR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn saddr_oct(&self) -> SaddrOctR {
        SaddrOctR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn scmd_oct(&self) -> ScmdOctR {
        ScmdOctR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_rin(&self) -> SdummyRinR {
        SdummyRinR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_wout(&self) -> SdummyWoutR {
        SdummyWoutR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_dqs_always_out(&self) -> SpiSmemWdummyDqsAlwaysOutR {
        SpiSmemWdummyDqsAlwaysOutR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_always_out(&self) -> SpiSmemWdummyAlwaysOutR {
        SpiSmemWdummyAlwaysOutR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_hex(&self) -> SdinHexR {
        SdinHexR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_hex(&self) -> SdoutHexR {
        SdoutHexR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_dqs_ie_always_on(&self) -> SpiSmemDqsIeAlwaysOnR {
        SpiSmemDqsIeAlwaysOnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_data_ie_always_on(&self) -> SpiSmemDataIeAlwaysOnR {
        SpiSmemDataIeAlwaysOnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
    #[inline(always)]
    pub fn sclk_mode(&mut self) -> SclkModeW<'_, SramCmdSpec> {
        SclkModeW::new(self, 0)
    }
    #[doc = "Bits 2:9 - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn swb_mode(&mut self) -> SwbModeW<'_, SramCmdSpec> {
        SwbModeW::new(self, 2)
    }
    #[doc = "Bit 10 - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdin_dual(&mut self) -> SdinDualW<'_, SramCmdSpec> {
        SdinDualW::new(self, 10)
    }
    #[doc = "Bit 11 - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdout_dual(&mut self) -> SdoutDualW<'_, SramCmdSpec> {
        SdoutDualW::new(self, 11)
    }
    #[doc = "Bit 12 - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn saddr_dual(&mut self) -> SaddrDualW<'_, SramCmdSpec> {
        SaddrDualW::new(self, 12)
    }
    #[doc = "Bit 14 - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdin_quad(&mut self) -> SdinQuadW<'_, SramCmdSpec> {
        SdinQuadW::new(self, 14)
    }
    #[doc = "Bit 15 - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdout_quad(&mut self) -> SdoutQuadW<'_, SramCmdSpec> {
        SdoutQuadW::new(self, 15)
    }
    #[doc = "Bit 16 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn saddr_quad(&mut self) -> SaddrQuadW<'_, SramCmdSpec> {
        SaddrQuadW::new(self, 16)
    }
    #[doc = "Bit 17 - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn scmd_quad(&mut self) -> ScmdQuadW<'_, SramCmdSpec> {
        ScmdQuadW::new(self, 17)
    }
    #[doc = "Bit 18 - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_oct(&mut self) -> SdinOctW<'_, SramCmdSpec> {
        SdinOctW::new(self, 18)
    }
    #[doc = "Bit 19 - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_oct(&mut self) -> SdoutOctW<'_, SramCmdSpec> {
        SdoutOctW::new(self, 19)
    }
    #[doc = "Bit 20 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn saddr_oct(&mut self) -> SaddrOctW<'_, SramCmdSpec> {
        SaddrOctW::new(self, 20)
    }
    #[doc = "Bit 21 - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn scmd_oct(&mut self) -> ScmdOctW<'_, SramCmdSpec> {
        ScmdOctW::new(self, 21)
    }
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_rin(&mut self) -> SdummyRinW<'_, SramCmdSpec> {
        SdummyRinW::new(self, 22)
    }
    #[doc = "Bit 23 - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_wout(&mut self) -> SdummyWoutW<'_, SramCmdSpec> {
        SdummyWoutW::new(self, 23)
    }
    #[doc = "Bit 24 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_dqs_always_out(
        &mut self,
    ) -> SpiSmemWdummyDqsAlwaysOutW<'_, SramCmdSpec> {
        SpiSmemWdummyDqsAlwaysOutW::new(self, 24)
    }
    #[doc = "Bit 25 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_always_out(&mut self) -> SpiSmemWdummyAlwaysOutW<'_, SramCmdSpec> {
        SpiSmemWdummyAlwaysOutW::new(self, 25)
    }
    #[doc = "Bit 26 - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_hex(&mut self) -> SdinHexW<'_, SramCmdSpec> {
        SdinHexW::new(self, 26)
    }
    #[doc = "Bit 27 - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_hex(&mut self) -> SdoutHexW<'_, SramCmdSpec> {
        SdoutHexW::new(self, 27)
    }
    #[doc = "Bit 30 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_dqs_ie_always_on(&mut self) -> SpiSmemDqsIeAlwaysOnW<'_, SramCmdSpec> {
        SpiSmemDqsIeAlwaysOnW::new(self, 30)
    }
    #[doc = "Bit 31 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_data_ie_always_on(&mut self) -> SpiSmemDataIeAlwaysOnW<'_, SramCmdSpec> {
        SpiSmemDataIeAlwaysOnW::new(self, 31)
    }
}
#[doc = "SPI0 external RAM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCmdSpec;
impl crate::RegisterSpec for SramCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cmd::R`](R) reader structure"]
impl crate::Readable for SramCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_cmd::W`](W) writer structure"]
impl crate::Writable for SramCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CMD to value 0x80c0_0000"]
impl crate::Resettable for SramCmdSpec {
    const RESET_VALUE: u32 = 0x80c0_0000;
}
