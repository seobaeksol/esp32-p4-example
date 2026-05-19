#[doc = "Register `SRAM_CLK` reader"]
pub type R = crate::R<SramClkSpec>;
#[doc = "Register `SRAM_CLK` writer"]
pub type W = crate::W<SramClkSpec>;
#[doc = "Field `SCLKCNT_L` reader - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
pub type SclkcntLR = crate::FieldReader;
#[doc = "Field `SCLKCNT_L` writer - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
pub type SclkcntLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLKCNT_H` reader - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub type SclkcntHR = crate::FieldReader;
#[doc = "Field `SCLKCNT_H` writer - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
pub type SclkcntHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLKCNT_N` reader - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
pub type SclkcntNR = crate::FieldReader;
#[doc = "Field `SCLKCNT_N` writer - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
pub type SclkcntNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_EQU_SYSCLK` reader - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
pub type SclkEquSysclkR = crate::BitReader;
#[doc = "Field `SCLK_EQU_SYSCLK` writer - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
pub type SclkEquSysclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn sclkcnt_l(&self) -> SclkcntLR {
        SclkcntLR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn sclkcnt_h(&self) -> SclkcntHR {
        SclkcntHR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn sclkcnt_n(&self) -> SclkcntNR {
        SclkcntNR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&self) -> SclkEquSysclkR {
        SclkEquSysclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - For SPI0 external RAM interface, it must be equal to SPI_MEM_SCLKCNT_N."]
    #[inline(always)]
    pub fn sclkcnt_l(&mut self) -> SclkcntLW<'_, SramClkSpec> {
        SclkcntLW::new(self, 0)
    }
    #[doc = "Bits 8:15 - For SPI0 external RAM interface, it must be floor((SPI_MEM_SCLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn sclkcnt_h(&mut self) -> SclkcntHW<'_, SramClkSpec> {
        SclkcntHW::new(self, 8)
    }
    #[doc = "Bits 16:23 - For SPI0 external RAM interface, it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_SCLKCNT_N+1)"]
    #[inline(always)]
    pub fn sclkcnt_n(&mut self) -> SclkcntNW<'_, SramClkSpec> {
        SclkcntNW::new(self, 16)
    }
    #[doc = "Bit 31 - For SPI0 external RAM interface, 1: spi_mem_clk is eqaul to system 0: spi_mem_clk is divided from system clock."]
    #[inline(always)]
    pub fn sclk_equ_sysclk(&mut self) -> SclkEquSysclkW<'_, SramClkSpec> {
        SclkEquSysclkW::new(self, 31)
    }
}
#[doc = "SPI0 external RAM clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramClkSpec;
impl crate::RegisterSpec for SramClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_clk::R`](R) reader structure"]
impl crate::Readable for SramClkSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_clk::W`](W) writer structure"]
impl crate::Writable for SramClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CLK to value 0x0003_0103"]
impl crate::Resettable for SramClkSpec {
    const RESET_VALUE: u32 = 0x0003_0103;
}
