#[doc = "Register `SIGMADELTA_MISC` reader"]
pub type R = crate::R<SigmadeltaMiscSpec>;
#[doc = "Register `SIGMADELTA_MISC` writer"]
pub type W = crate::W<SigmadeltaMiscSpec>;
#[doc = "Field `FUNCTION_CLK_EN` reader - Clock enable bit of sigma delta modulation."]
pub type FunctionClkEnR = crate::BitReader;
#[doc = "Field `FUNCTION_CLK_EN` writer - Clock enable bit of sigma delta modulation."]
pub type FunctionClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SWAP` reader - Reserved."]
pub type SpiSwapR = crate::BitReader;
#[doc = "Field `SPI_SWAP` writer - Reserved."]
pub type SpiSwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Clock enable bit of sigma delta modulation."]
    #[inline(always)]
    pub fn function_clk_en(&self) -> FunctionClkEnR {
        FunctionClkEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    pub fn spi_swap(&self) -> SpiSwapR {
        SpiSwapR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Clock enable bit of sigma delta modulation."]
    #[inline(always)]
    pub fn function_clk_en(&mut self) -> FunctionClkEnW<'_, SigmadeltaMiscSpec> {
        FunctionClkEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    pub fn spi_swap(&mut self) -> SpiSwapW<'_, SigmadeltaMiscSpec> {
        SpiSwapW::new(self, 31)
    }
}
#[doc = "MISC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sigmadelta_misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigmadelta_misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SigmadeltaMiscSpec;
impl crate::RegisterSpec for SigmadeltaMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta_misc::R`](R) reader structure"]
impl crate::Readable for SigmadeltaMiscSpec {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta_misc::W`](W) writer structure"]
impl crate::Writable for SigmadeltaMiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SIGMADELTA_MISC to value 0"]
impl crate::Resettable for SigmadeltaMiscSpec {}
