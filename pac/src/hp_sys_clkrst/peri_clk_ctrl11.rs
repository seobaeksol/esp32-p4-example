#[doc = "Register `PERI_CLK_CTRL11` reader"]
pub type R = crate::R<PeriClkCtrl11Spec>;
#[doc = "Register `PERI_CLK_CTRL11` writer"]
pub type W = crate::W<PeriClkCtrl11Spec>;
#[doc = "Field `I2C1_CLK_DIV_NUM` reader - Reserved"]
pub type I2c1ClkDivNumR = crate::FieldReader;
#[doc = "Field `I2C1_CLK_DIV_NUM` writer - Reserved"]
pub type I2c1ClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C1_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type I2c1ClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `I2C1_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type I2c1ClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C1_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type I2c1ClkDivDenominatorR = crate::FieldReader;
#[doc = "Field `I2C1_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type I2c1ClkDivDenominatorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2S0_RX_CLK_EN` reader - Reserved"]
pub type I2s0RxClkEnR = crate::BitReader;
#[doc = "Field `I2S0_RX_CLK_EN` writer - Reserved"]
pub type I2s0RxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_RX_CLK_SRC_SEL` reader - Reserved"]
pub type I2s0RxClkSrcSelR = crate::FieldReader;
#[doc = "Field `I2S0_RX_CLK_SRC_SEL` writer - Reserved"]
pub type I2s0RxClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_div_num(&self) -> I2c1ClkDivNumR {
        I2c1ClkDivNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_div_numerator(&self) -> I2c1ClkDivNumeratorR {
        I2c1ClkDivNumeratorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_div_denominator(&self) -> I2c1ClkDivDenominatorR {
        I2c1ClkDivDenominatorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_clk_en(&self) -> I2s0RxClkEnR {
        I2s0RxClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_clk_src_sel(&self) -> I2s0RxClkSrcSelR {
        I2s0RxClkSrcSelR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_div_num(&mut self) -> I2c1ClkDivNumW<'_, PeriClkCtrl11Spec> {
        I2c1ClkDivNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_div_numerator(&mut self) -> I2c1ClkDivNumeratorW<'_, PeriClkCtrl11Spec> {
        I2c1ClkDivNumeratorW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_div_denominator(&mut self) -> I2c1ClkDivDenominatorW<'_, PeriClkCtrl11Spec> {
        I2c1ClkDivDenominatorW::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_clk_en(&mut self) -> I2s0RxClkEnW<'_, PeriClkCtrl11Spec> {
        I2s0RxClkEnW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_clk_src_sel(&mut self) -> I2s0RxClkSrcSelW<'_, PeriClkCtrl11Spec> {
        I2s0RxClkSrcSelW::new(self, 25)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl11Spec;
impl crate::RegisterSpec for PeriClkCtrl11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl11::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl11Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl11::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL11 to value 0"]
impl crate::Resettable for PeriClkCtrl11Spec {}
