#[doc = "Register `PERI_CLK_CTRL22` reader"]
pub type R = crate::R<PeriClkCtrl22Spec>;
#[doc = "Register `PERI_CLK_CTRL22` writer"]
pub type W = crate::W<PeriClkCtrl22Spec>;
#[doc = "Field `LEDC_CLK_SRC_SEL` reader - Reserved"]
pub type LedcClkSrcSelR = crate::FieldReader;
#[doc = "Field `LEDC_CLK_SRC_SEL` writer - Reserved"]
pub type LedcClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEDC_CLK_EN` reader - Reserved"]
pub type LedcClkEnR = crate::BitReader;
#[doc = "Field `LEDC_CLK_EN` writer - Reserved"]
pub type LedcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_CLK_SRC_SEL` reader - Reserved"]
pub type RmtClkSrcSelR = crate::FieldReader;
#[doc = "Field `RMT_CLK_SRC_SEL` writer - Reserved"]
pub type RmtClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RMT_CLK_EN` reader - Reserved"]
pub type RmtClkEnR = crate::BitReader;
#[doc = "Field `RMT_CLK_EN` writer - Reserved"]
pub type RmtClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_CLK_DIV_NUM` reader - Reserved"]
pub type RmtClkDivNumR = crate::FieldReader;
#[doc = "Field `RMT_CLK_DIV_NUM` writer - Reserved"]
pub type RmtClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RMT_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type RmtClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `RMT_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type RmtClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RMT_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type RmtClkDivDenominatorR = crate::FieldReader;
#[doc = "Field `RMT_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type RmtClkDivDenominatorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADC_CLK_SRC_SEL` reader - Reserved"]
pub type AdcClkSrcSelR = crate::FieldReader;
#[doc = "Field `ADC_CLK_SRC_SEL` writer - Reserved"]
pub type AdcClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn ledc_clk_src_sel(&self) -> LedcClkSrcSelR {
        LedcClkSrcSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LedcClkEnR {
        LedcClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_src_sel(&self) -> RmtClkSrcSelR {
        RmtClkSrcSelR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RmtClkEnR {
        RmtClkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_num(&self) -> RmtClkDivNumR {
        RmtClkDivNumR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_numerator(&self) -> RmtClkDivNumeratorR {
        RmtClkDivNumeratorR::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:29 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_denominator(&self) -> RmtClkDivDenominatorR {
        RmtClkDivDenominatorR::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_src_sel(&self) -> AdcClkSrcSelR {
        AdcClkSrcSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn ledc_clk_src_sel(&mut self) -> LedcClkSrcSelW<'_, PeriClkCtrl22Spec> {
        LedcClkSrcSelW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LedcClkEnW<'_, PeriClkCtrl22Spec> {
        LedcClkEnW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_src_sel(&mut self) -> RmtClkSrcSelW<'_, PeriClkCtrl22Spec> {
        RmtClkSrcSelW::new(self, 3)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RmtClkEnW<'_, PeriClkCtrl22Spec> {
        RmtClkEnW::new(self, 5)
    }
    #[doc = "Bits 6:13 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_num(&mut self) -> RmtClkDivNumW<'_, PeriClkCtrl22Spec> {
        RmtClkDivNumW::new(self, 6)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_numerator(&mut self) -> RmtClkDivNumeratorW<'_, PeriClkCtrl22Spec> {
        RmtClkDivNumeratorW::new(self, 14)
    }
    #[doc = "Bits 22:29 - Reserved"]
    #[inline(always)]
    pub fn rmt_clk_div_denominator(&mut self) -> RmtClkDivDenominatorW<'_, PeriClkCtrl22Spec> {
        RmtClkDivDenominatorW::new(self, 22)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn adc_clk_src_sel(&mut self) -> AdcClkSrcSelW<'_, PeriClkCtrl22Spec> {
        AdcClkSrcSelW::new(self, 30)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl22Spec;
impl crate::RegisterSpec for PeriClkCtrl22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl22::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl22Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl22::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL22 to value 0"]
impl crate::Resettable for PeriClkCtrl22Spec {}
