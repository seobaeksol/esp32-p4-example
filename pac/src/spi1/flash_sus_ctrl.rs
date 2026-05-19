#[doc = "Register `FLASH_SUS_CTRL` reader"]
pub type R = crate::R<FlashSusCtrlSpec>;
#[doc = "Register `FLASH_SUS_CTRL` writer"]
pub type W = crate::W<FlashSusCtrlSpec>;
#[doc = "Field `FLASH_PER` reader - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashPerR = crate::BitReader;
#[doc = "Field `FLASH_PER` writer - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashPerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES` reader - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashPesR = crate::BitReader;
#[doc = "Field `FLASH_PES` writer - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
pub type FlashPesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PER_WAIT_EN` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
pub type FlashPerWaitEnR = crate::BitReader;
#[doc = "Field `FLASH_PER_WAIT_EN` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
pub type FlashPerWaitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_WAIT_EN` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
pub type FlashPesWaitEnR = crate::BitReader;
#[doc = "Field `FLASH_PES_WAIT_EN` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
pub type FlashPesWaitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_PER_EN` reader - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
pub type PesPerEnR = crate::BitReader;
#[doc = "Field `PES_PER_EN` writer - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
pub type PesPerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_PES_EN` reader - Set this bit to enable Auto-suspending function."]
pub type FlashPesEnR = crate::BitReader;
#[doc = "Field `FLASH_PES_EN` writer - Set this bit to enable Auto-suspending function."]
pub type FlashPesEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PESR_END_MSK` reader - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
pub type PesrEndMskR = crate::FieldReader<u16>;
#[doc = "Field `PESR_END_MSK` writer - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
pub type PesrEndMskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SPI_FMEM_RD_SUS_2B` reader - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
pub type SpiFmemRdSus2bR = crate::BitReader;
#[doc = "Field `SPI_FMEM_RD_SUS_2B` writer - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
pub type SpiFmemRdSus2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
pub type PerEndEnR = crate::BitReader;
#[doc = "Field `PER_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
pub type PerEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END_EN` reader - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
pub type PesEndEnR = crate::BitReader;
#[doc = "Field `PES_END_EN` writer - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
pub type PesEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUS_TIMEOUT_CNT` reader - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
pub type SusTimeoutCntR = crate::FieldReader;
#[doc = "Field `SUS_TIMEOUT_CNT` writer - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
pub type SusTimeoutCntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&self) -> FlashPerR {
        FlashPerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&self) -> FlashPesR {
        FlashPesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
    #[inline(always)]
    pub fn flash_per_wait_en(&self) -> FlashPerWaitEnR {
        FlashPerWaitEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
    #[inline(always)]
    pub fn flash_pes_wait_en(&self) -> FlashPesWaitEnR {
        FlashPesWaitEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
    #[inline(always)]
    pub fn pes_per_en(&self) -> PesPerEnR {
        PesPerEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    pub fn flash_pes_en(&self) -> FlashPesEnR {
        FlashPesEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
    #[inline(always)]
    pub fn pesr_end_msk(&self) -> PesrEndMskR {
        PesrEndMskR::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
    #[inline(always)]
    pub fn spi_fmem_rd_sus_2b(&self) -> SpiFmemRdSus2bR {
        SpiFmemRdSus2bR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn per_end_en(&self) -> PerEndEnR {
        PerEndEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn pes_end_en(&self) -> PesEndEnR {
        PesEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
    #[inline(always)]
    pub fn sus_timeout_cnt(&self) -> SusTimeoutCntR {
        SusTimeoutCntR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - program erase resume bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_per(&mut self) -> FlashPerW<'_, FlashSusCtrlSpec> {
        FlashPerW::new(self, 0)
    }
    #[doc = "Bit 1 - program erase suspend bit, program erase suspend operation will be triggered when the bit is set. The bit will be cleared once the operation done.1: enable 0: disable."]
    #[inline(always)]
    pub fn flash_pes(&mut self) -> FlashPesW<'_, FlashSusCtrlSpec> {
        FlashPesW::new(self, 1)
    }
    #[doc = "Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase resume command is sent. 0: SPI1 does not wait after program erase resume command is sent."]
    #[inline(always)]
    pub fn flash_per_wait_en(&mut self) -> FlashPerWaitEnW<'_, FlashSusCtrlSpec> {
        FlashPerWaitEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\] * 4 or *128) SPI_CLK cycles after program erase suspend command is sent. 0: SPI1 does not wait after program erase suspend command is sent."]
    #[inline(always)]
    pub fn flash_pes_wait_en(&mut self) -> FlashPesWaitEnW<'_, FlashSusCtrlSpec> {
        FlashPesWaitEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable PES end triggers PER transfer option. If this bit is 0, application should send PER after PES is done."]
    #[inline(always)]
    pub fn pes_per_en(&mut self) -> PesPerEnW<'_, FlashSusCtrlSpec> {
        PesPerEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable Auto-suspending function."]
    #[inline(always)]
    pub fn flash_pes_en(&mut self) -> FlashPesEnW<'_, FlashSusCtrlSpec> {
        FlashPesEnW::new(self, 5)
    }
    #[doc = "Bits 6:21 - The mask value when check SUS/SUS1/SUS2 status bit. If the read status value is status_in\\[15:0\\](only status_in\\[7:0\\] is valid when only one byte of data is read out, status_in\\[15:0\\] is valid when two bytes of data are read out), SUS/SUS1/SUS2 = status_in\\[15:0\\]^ SPI_MEM_PESR_END_MSK\\[15:0\\]."]
    #[inline(always)]
    pub fn pesr_end_msk(&mut self) -> PesrEndMskW<'_, FlashSusCtrlSpec> {
        PesrEndMskW::new(self, 6)
    }
    #[doc = "Bit 22 - 1: Read two bytes when check flash SUS/SUS1/SUS2 status bit. 0: Read one byte when check flash SUS/SUS1/SUS2 status bit"]
    #[inline(always)]
    pub fn spi_fmem_rd_sus_2b(&mut self) -> SpiFmemRdSus2bW<'_, FlashSusCtrlSpec> {
        SpiFmemRdSus2bW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the resume status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn per_end_en(&mut self) -> PerEndEnW<'_, FlashSusCtrlSpec> {
        PerEndEnW::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Both WIP and SUS/SUS1/SUS2 bits should be checked to insure the suspend status of flash. 0: Only need to check WIP is 0."]
    #[inline(always)]
    pub fn pes_end_en(&mut self) -> PesEndEnW<'_, FlashSusCtrlSpec> {
        PesEndEnW::new(self, 24)
    }
    #[doc = "Bits 25:31 - When SPI1 checks SUS/SUS1/SUS2 bits fail for SPI_MEM_SUS_TIMEOUT_CNT\\[6:0\\] times, it will be treated as check pass."]
    #[inline(always)]
    pub fn sus_timeout_cnt(&mut self) -> SusTimeoutCntW<'_, FlashSusCtrlSpec> {
        SusTimeoutCntW::new(self, 25)
    }
}
#[doc = "SPI1 flash suspend control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_sus_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sus_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashSusCtrlSpec;
impl crate::RegisterSpec for FlashSusCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_sus_ctrl::R`](R) reader structure"]
impl crate::Readable for FlashSusCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_sus_ctrl::W`](W) writer structure"]
impl crate::Writable for FlashSusCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_SUS_CTRL to value 0x0800_2000"]
impl crate::Resettable for FlashSusCtrlSpec {
    const RESET_VALUE: u32 = 0x0800_2000;
}
