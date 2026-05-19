#[doc = "Register `PERI_CLK_CTRL24` reader"]
pub type R = crate::R<PeriClkCtrl24Spec>;
#[doc = "Register `PERI_CLK_CTRL24` writer"]
pub type W = crate::W<PeriClkCtrl24Spec>;
#[doc = "Field `ADC_SAR1_CLK_DIV_NUM` reader - Reserved"]
pub type AdcSar1ClkDivNumR = crate::FieldReader;
#[doc = "Field `ADC_SAR1_CLK_DIV_NUM` writer - Reserved"]
pub type AdcSar1ClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADC_SAR2_CLK_DIV_NUM` reader - Reserved"]
pub type AdcSar2ClkDivNumR = crate::FieldReader;
#[doc = "Field `ADC_SAR2_CLK_DIV_NUM` writer - Reserved"]
pub type AdcSar2ClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PVT_CLK_DIV_NUM` reader - Reserved"]
pub type PvtClkDivNumR = crate::FieldReader;
#[doc = "Field `PVT_CLK_DIV_NUM` writer - Reserved"]
pub type PvtClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PVT_CLK_EN` reader - Reserved"]
pub type PvtClkEnR = crate::BitReader;
#[doc = "Field `PVT_CLK_EN` writer - Reserved"]
pub type PvtClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn adc_sar1_clk_div_num(&self) -> AdcSar1ClkDivNumR {
        AdcSar1ClkDivNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn adc_sar2_clk_div_num(&self) -> AdcSar2ClkDivNumR {
        AdcSar2ClkDivNumR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_div_num(&self) -> PvtClkDivNumR {
        PvtClkDivNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_en(&self) -> PvtClkEnR {
        PvtClkEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn adc_sar1_clk_div_num(&mut self) -> AdcSar1ClkDivNumW<'_, PeriClkCtrl24Spec> {
        AdcSar1ClkDivNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn adc_sar2_clk_div_num(&mut self) -> AdcSar2ClkDivNumW<'_, PeriClkCtrl24Spec> {
        AdcSar2ClkDivNumW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_div_num(&mut self) -> PvtClkDivNumW<'_, PeriClkCtrl24Spec> {
        PvtClkDivNumW::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pvt_clk_en(&mut self) -> PvtClkEnW<'_, PeriClkCtrl24Spec> {
        PvtClkEnW::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl24Spec;
impl crate::RegisterSpec for PeriClkCtrl24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl24::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl24Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl24::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL24 to value 0x0404"]
impl crate::Resettable for PeriClkCtrl24Spec {
    const RESET_VALUE: u32 = 0x0404;
}
