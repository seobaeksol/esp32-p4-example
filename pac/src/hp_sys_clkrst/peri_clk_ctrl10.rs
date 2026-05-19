#[doc = "Register `PERI_CLK_CTRL10` reader"]
pub type R = crate::R<PeriClkCtrl10Spec>;
#[doc = "Register `PERI_CLK_CTRL10` writer"]
pub type W = crate::W<PeriClkCtrl10Spec>;
#[doc = "Field `I2C0_CLK_SRC_SEL` reader - Reserved"]
pub type I2c0ClkSrcSelR = crate::BitReader;
#[doc = "Field `I2C0_CLK_SRC_SEL` writer - Reserved"]
pub type I2c0ClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_CLK_EN` reader - Reserved"]
pub type I2c0ClkEnR = crate::BitReader;
#[doc = "Field `I2C0_CLK_EN` writer - Reserved"]
pub type I2c0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0_CLK_DIV_NUM` reader - Reserved"]
pub type I2c0ClkDivNumR = crate::FieldReader;
#[doc = "Field `I2C0_CLK_DIV_NUM` writer - Reserved"]
pub type I2c0ClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C0_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type I2c0ClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `I2C0_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type I2c0ClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C0_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type I2c0ClkDivDenominatorR = crate::FieldReader;
#[doc = "Field `I2C0_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type I2c0ClkDivDenominatorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C1_CLK_SRC_SEL` reader - Reserved"]
pub type I2c1ClkSrcSelR = crate::BitReader;
#[doc = "Field `I2C1_CLK_SRC_SEL` writer - Reserved"]
pub type I2c1ClkSrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_CLK_EN` reader - Reserved"]
pub type I2c1ClkEnR = crate::BitReader;
#[doc = "Field `I2C1_CLK_EN` writer - Reserved"]
pub type I2c1ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_src_sel(&self) -> I2c0ClkSrcSelR {
        I2c0ClkSrcSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2c0ClkEnR {
        I2c0ClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_num(&self) -> I2c0ClkDivNumR {
        I2c0ClkDivNumR::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_numerator(&self) -> I2c0ClkDivNumeratorR {
        I2c0ClkDivNumeratorR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_denominator(&self) -> I2c0ClkDivDenominatorR {
        I2c0ClkDivDenominatorR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_src_sel(&self) -> I2c1ClkSrcSelR {
        I2c1ClkSrcSelR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_en(&self) -> I2c1ClkEnR {
        I2c1ClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_src_sel(&mut self) -> I2c0ClkSrcSelW<'_, PeriClkCtrl10Spec> {
        I2c0ClkSrcSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_en(&mut self) -> I2c0ClkEnW<'_, PeriClkCtrl10Spec> {
        I2c0ClkEnW::new(self, 1)
    }
    #[doc = "Bits 2:9 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_num(&mut self) -> I2c0ClkDivNumW<'_, PeriClkCtrl10Spec> {
        I2c0ClkDivNumW::new(self, 2)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_numerator(&mut self) -> I2c0ClkDivNumeratorW<'_, PeriClkCtrl10Spec> {
        I2c0ClkDivNumeratorW::new(self, 10)
    }
    #[doc = "Bits 18:25 - Reserved"]
    #[inline(always)]
    pub fn i2c0_clk_div_denominator(&mut self) -> I2c0ClkDivDenominatorW<'_, PeriClkCtrl10Spec> {
        I2c0ClkDivDenominatorW::new(self, 18)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_src_sel(&mut self) -> I2c1ClkSrcSelW<'_, PeriClkCtrl10Spec> {
        I2c1ClkSrcSelW::new(self, 26)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2c1_clk_en(&mut self) -> I2c1ClkEnW<'_, PeriClkCtrl10Spec> {
        I2c1ClkEnW::new(self, 27)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl10Spec;
impl crate::RegisterSpec for PeriClkCtrl10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl10::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl10Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl10::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL10 to value 0"]
impl crate::Resettable for PeriClkCtrl10Spec {}
