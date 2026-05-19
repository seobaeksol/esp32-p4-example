#[doc = "Register `CLOCK` reader"]
pub type R = crate::R<ClockSpec>;
#[doc = "Register `CLOCK` writer"]
pub type W = crate::W<ClockSpec>;
#[doc = "Field `CLKCNT_L` reader - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
pub type ClkcntLR = crate::FieldReader;
#[doc = "Field `CLKCNT_L` writer - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
pub type ClkcntLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKCNT_H` reader - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub type ClkcntHR = crate::FieldReader;
#[doc = "Field `CLKCNT_H` writer - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
pub type ClkcntHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKCNT_N` reader - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
pub type ClkcntNR = crate::FieldReader;
#[doc = "Field `CLKCNT_N` writer - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
pub type ClkcntNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_EQU_SYSCLK` reader - reserved"]
pub type ClkEquSysclkR = crate::BitReader;
#[doc = "Field `CLK_EQU_SYSCLK` writer - reserved"]
pub type ClkEquSysclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> ClkcntLR {
        ClkcntLR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> ClkcntHR {
        ClkcntHR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> ClkcntNR {
        ClkcntNR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> ClkEquSysclkR {
        ClkEquSysclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - In the master mode it must be equal to SPI_MEM_CLKCNT_N."]
    #[inline(always)]
    pub fn clkcnt_l(&mut self) -> ClkcntLW<'_, ClockSpec> {
        ClkcntLW::new(self, 0)
    }
    #[doc = "Bits 8:15 - In the master mode it must be floor((SPI_MEM_CLKCNT_N+1)/2-1)."]
    #[inline(always)]
    pub fn clkcnt_h(&mut self) -> ClkcntHW<'_, ClockSpec> {
        ClkcntHW::new(self, 8)
    }
    #[doc = "Bits 16:23 - In the master mode it is the divider of spi_mem_clk. So spi_mem_clk frequency is system/(SPI_MEM_CLKCNT_N+1)"]
    #[inline(always)]
    pub fn clkcnt_n(&mut self) -> ClkcntNW<'_, ClockSpec> {
        ClkcntNW::new(self, 16)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn clk_equ_sysclk(&mut self) -> ClkEquSysclkW<'_, ClockSpec> {
        ClkEquSysclkW::new(self, 31)
    }
}
#[doc = "SPI1 clock division control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockSpec;
impl crate::RegisterSpec for ClockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock::R`](R) reader structure"]
impl crate::Readable for ClockSpec {}
#[doc = "`write(|w| ..)` method takes [`clock::W`](W) writer structure"]
impl crate::Writable for ClockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK to value 0x0003_0103"]
impl crate::Resettable for ClockSpec {
    const RESET_VALUE: u32 = 0x0003_0103;
}
