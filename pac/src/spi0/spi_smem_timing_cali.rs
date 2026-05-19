#[doc = "Register `SPI_SMEM_TIMING_CALI` reader"]
pub type R = crate::R<SpiSmemTimingCaliSpec>;
#[doc = "Register `SPI_SMEM_TIMING_CALI` writer"]
pub type W = crate::W<SpiSmemTimingCaliSpec>;
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` reader - For sram, the bit is used to enable timing adjust clock for all reading operations."]
pub type SpiSmemTimingClkEnaR = crate::BitReader;
#[doc = "Field `SPI_SMEM_TIMING_CLK_ENA` writer - For sram, the bit is used to enable timing adjust clock for all reading operations."]
pub type SpiSmemTimingClkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_TIMING_CALI` reader - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
pub type SpiSmemTimingCaliR = crate::BitReader;
#[doc = "Field `SPI_SMEM_TIMING_CALI` writer - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
pub type SpiSmemTimingCaliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` reader - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
pub type SpiSmemExtraDummyCyclelenR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_EXTRA_DUMMY_CYCLELEN` writer - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
pub type SpiSmemExtraDummyCyclelenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI_SMEM_DLL_TIMING_CALI` reader - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
pub type SpiSmemDllTimingCaliR = crate::BitReader;
#[doc = "Field `SPI_SMEM_DLL_TIMING_CALI` writer - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
pub type SpiSmemDllTimingCaliW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For sram, the bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&self) -> SpiSmemTimingClkEnaR {
        SpiSmemTimingClkEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&self) -> SpiSmemTimingCaliR {
        SpiSmemTimingCaliR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(&self) -> SpiSmemExtraDummyCyclelenR {
        SpiSmemExtraDummyCyclelenR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
    #[inline(always)]
    pub fn spi_smem_dll_timing_cali(&self) -> SpiSmemDllTimingCaliR {
        SpiSmemDllTimingCaliR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For sram, the bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_clk_ena(&mut self) -> SpiSmemTimingClkEnaW<'_, SpiSmemTimingCaliSpec> {
        SpiSmemTimingClkEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - For sram, the bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn spi_smem_timing_cali(&mut self) -> SpiSmemTimingCaliW<'_, SpiSmemTimingCaliSpec> {
        SpiSmemTimingCaliW::new(self, 1)
    }
    #[doc = "Bits 2:4 - For sram, add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn spi_smem_extra_dummy_cyclelen(
        &mut self,
    ) -> SpiSmemExtraDummyCyclelenW<'_, SpiSmemTimingCaliSpec> {
        SpiSmemExtraDummyCyclelenW::new(self, 2)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to EXT_RAM."]
    #[inline(always)]
    pub fn spi_smem_dll_timing_cali(&mut self) -> SpiSmemDllTimingCaliW<'_, SpiSmemTimingCaliSpec> {
        SpiSmemDllTimingCaliW::new(self, 5)
    }
}
#[doc = "MSPI external RAM timing calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_timing_cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_timing_cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSmemTimingCaliSpec;
impl crate::RegisterSpec for SpiSmemTimingCaliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_timing_cali::R`](R) reader structure"]
impl crate::Readable for SpiSmemTimingCaliSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_timing_cali::W`](W) writer structure"]
impl crate::Writable for SpiSmemTimingCaliSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_TIMING_CALI to value 0x01"]
impl crate::Resettable for SpiSmemTimingCaliSpec {
    const RESET_VALUE: u32 = 0x01;
}
