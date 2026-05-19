#[doc = "Register `SPI_SMEM_DIN_HEX_NUM` reader"]
pub type R = crate::R<SpiSmemDinHexNumSpec>;
#[doc = "Register `SPI_SMEM_DIN_HEX_NUM` writer"]
pub type W = crate::W<SpiSmemDinHexNumSpec>;
#[doc = "Field `SPI_SMEM_DIN08_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin08NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN08_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin08NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN09_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin09NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN09_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin09NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN10_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin10NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN10_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin10NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN11_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin11NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN11_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin11NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN12_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin12NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN12_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin12NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN13_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin13NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN13_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin13NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN14_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin14NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN14_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin14NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DIN15_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin15NumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DIN15_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDin15NumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_SMEM_DINS_HEX_NUM` reader - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDinsHexNumR = crate::FieldReader;
#[doc = "Field `SPI_SMEM_DINS_HEX_NUM` writer - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
pub type SpiSmemDinsHexNumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din08_num(&self) -> SpiSmemDin08NumR {
        SpiSmemDin08NumR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din09_num(&self) -> SpiSmemDin09NumR {
        SpiSmemDin09NumR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din10_num(&self) -> SpiSmemDin10NumR {
        SpiSmemDin10NumR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din11_num(&self) -> SpiSmemDin11NumR {
        SpiSmemDin11NumR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din12_num(&self) -> SpiSmemDin12NumR {
        SpiSmemDin12NumR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din13_num(&self) -> SpiSmemDin13NumR {
        SpiSmemDin13NumR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din14_num(&self) -> SpiSmemDin14NumR {
        SpiSmemDin14NumR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din15_num(&self) -> SpiSmemDin15NumR {
        SpiSmemDin15NumR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_dins_hex_num(&self) -> SpiSmemDinsHexNumR {
        SpiSmemDinsHexNumR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din08_num(&mut self) -> SpiSmemDin08NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin08NumW::new(self, 0)
    }
    #[doc = "Bits 2:3 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din09_num(&mut self) -> SpiSmemDin09NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin09NumW::new(self, 2)
    }
    #[doc = "Bits 4:5 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din10_num(&mut self) -> SpiSmemDin10NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin10NumW::new(self, 4)
    }
    #[doc = "Bits 6:7 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din11_num(&mut self) -> SpiSmemDin11NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin11NumW::new(self, 6)
    }
    #[doc = "Bits 8:9 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din12_num(&mut self) -> SpiSmemDin12NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin12NumW::new(self, 8)
    }
    #[doc = "Bits 10:11 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din13_num(&mut self) -> SpiSmemDin13NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin13NumW::new(self, 10)
    }
    #[doc = "Bits 12:13 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din14_num(&mut self) -> SpiSmemDin14NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin14NumW::new(self, 12)
    }
    #[doc = "Bits 14:15 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_din15_num(&mut self) -> SpiSmemDin15NumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDin15NumW::new(self, 14)
    }
    #[doc = "Bits 16:17 - the input signals are delayed by system clock cycles, 0: delayed by 1 cycle, 1: delayed by 2 cycles,..."]
    #[inline(always)]
    pub fn spi_smem_dins_hex_num(&mut self) -> SpiSmemDinsHexNumW<'_, SpiSmemDinHexNumSpec> {
        SpiSmemDinsHexNumW::new(self, 16)
    }
}
#[doc = "MSPI 16x external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_din_hex_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_smem_din_hex_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiSmemDinHexNumSpec;
impl crate::RegisterSpec for SpiSmemDinHexNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_din_hex_num::R`](R) reader structure"]
impl crate::Readable for SpiSmemDinHexNumSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_smem_din_hex_num::W`](W) writer structure"]
impl crate::Writable for SpiSmemDinHexNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_SMEM_DIN_HEX_NUM to value 0"]
impl crate::Resettable for SpiSmemDinHexNumSpec {}
