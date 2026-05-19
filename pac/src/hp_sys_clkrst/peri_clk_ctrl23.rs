#[doc = "Register `PERI_CLK_CTRL23` reader"]
pub type R = crate::R<PeriClkCtrl23Spec>;
#[doc = "Register `PERI_CLK_CTRL23` writer"]
pub type W = crate::W<PeriClkCtrl23Spec>;
#[doc = "Field `ADC_CLK_EN` reader - Reserved"]
pub type AdcClkEnR = crate::BitReader;
#[doc = "Field `ADC_CLK_EN` writer - Reserved"]
pub type AdcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_CLK_DIV_NUM` reader - Reserved"]
pub type AdcClkDivNumR = crate::FieldReader;
#[doc = "Field `ADC_CLK_DIV_NUM` writer - Reserved"]
pub type AdcClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADC_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type AdcClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `ADC_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type AdcClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADC_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type AdcClkDivDenominatorR = crate::FieldReader;
#[doc = "Field `ADC_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type AdcClkDivDenominatorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> AdcClkEnR {
        AdcClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_div_num(&self) -> AdcClkDivNumR {
        AdcClkDivNumR::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_div_numerator(&self) -> AdcClkDivNumeratorR {
        AdcClkDivNumeratorR::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:24 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_div_denominator(&self) -> AdcClkDivDenominatorR {
        AdcClkDivDenominatorR::new(((self.bits >> 17) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_en(&mut self) -> AdcClkEnW<'_, PeriClkCtrl23Spec> {
        AdcClkEnW::new(self, 0)
    }
    #[doc = "Bits 1:8 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_div_num(&mut self) -> AdcClkDivNumW<'_, PeriClkCtrl23Spec> {
        AdcClkDivNumW::new(self, 1)
    }
    #[doc = "Bits 9:16 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_div_numerator(&mut self) -> AdcClkDivNumeratorW<'_, PeriClkCtrl23Spec> {
        AdcClkDivNumeratorW::new(self, 9)
    }
    #[doc = "Bits 17:24 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_div_denominator(&mut self) -> AdcClkDivDenominatorW<'_, PeriClkCtrl23Spec> {
        AdcClkDivDenominatorW::new(self, 17)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl23Spec;
impl crate::RegisterSpec for PeriClkCtrl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl23::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl23Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl23::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL23 to value 0x08"]
impl crate::Resettable for PeriClkCtrl23Spec {
    const RESET_VALUE: u32 = 0x08;
}
