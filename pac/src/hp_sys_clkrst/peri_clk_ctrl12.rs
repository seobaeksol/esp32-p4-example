#[doc = "Register `PERI_CLK_CTRL12` reader"]
pub type R = crate::R<PeriClkCtrl12Spec>;
#[doc = "Register `PERI_CLK_CTRL12` writer"]
pub type W = crate::W<PeriClkCtrl12Spec>;
#[doc = "Field `I2S0_RX_DIV_N` reader - Reserved"]
pub type I2s0RxDivNR = crate::FieldReader;
#[doc = "Field `I2S0_RX_DIV_N` writer - Reserved"]
pub type I2s0RxDivNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2S0_RX_DIV_X` reader - Reserved"]
pub type I2s0RxDivXR = crate::FieldReader<u16>;
#[doc = "Field `I2S0_RX_DIV_X` writer - Reserved"]
pub type I2s0RxDivXW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S0_RX_DIV_Y` reader - Reserved"]
pub type I2s0RxDivYR = crate::FieldReader<u16>;
#[doc = "Field `I2S0_RX_DIV_Y` writer - Reserved"]
pub type I2s0RxDivYW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_n(&self) -> I2s0RxDivNR {
        I2s0RxDivNR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_x(&self) -> I2s0RxDivXR {
        I2s0RxDivXR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 17:25 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_y(&self) -> I2s0RxDivYR {
        I2s0RxDivYR::new(((self.bits >> 17) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_n(&mut self) -> I2s0RxDivNW<'_, PeriClkCtrl12Spec> {
        I2s0RxDivNW::new(self, 0)
    }
    #[doc = "Bits 8:16 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_x(&mut self) -> I2s0RxDivXW<'_, PeriClkCtrl12Spec> {
        I2s0RxDivXW::new(self, 8)
    }
    #[doc = "Bits 17:25 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_y(&mut self) -> I2s0RxDivYW<'_, PeriClkCtrl12Spec> {
        I2s0RxDivYW::new(self, 17)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl12Spec;
impl crate::RegisterSpec for PeriClkCtrl12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl12::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl12Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl12::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL12 to value 0"]
impl crate::Resettable for PeriClkCtrl12Spec {}
