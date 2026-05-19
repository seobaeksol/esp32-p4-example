#[doc = "Register `PERI_CLK_CTRL110` reader"]
pub type R = crate::R<PeriClkCtrl110Spec>;
#[doc = "Register `PERI_CLK_CTRL110` writer"]
pub type W = crate::W<PeriClkCtrl110Spec>;
#[doc = "Field `LCD_CLK_DIV_NUM` reader - Reserved"]
pub type LcdClkDivNumR = crate::FieldReader;
#[doc = "Field `LCD_CLK_DIV_NUM` writer - Reserved"]
pub type LcdClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LCD_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type LcdClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `LCD_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type LcdClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LCD_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type LcdClkDivDenominatorR = crate::FieldReader;
#[doc = "Field `LCD_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type LcdClkDivDenominatorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART0_CLK_SRC_SEL` reader - Reserved"]
pub type Uart0ClkSrcSelR = crate::FieldReader;
#[doc = "Field `UART0_CLK_SRC_SEL` writer - Reserved"]
pub type Uart0ClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART0_CLK_EN` reader - Reserved"]
pub type Uart0ClkEnR = crate::BitReader;
#[doc = "Field `UART0_CLK_EN` writer - Reserved"]
pub type Uart0ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_div_num(&self) -> LcdClkDivNumR {
        LcdClkDivNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_div_numerator(&self) -> LcdClkDivNumeratorR {
        LcdClkDivNumeratorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_div_denominator(&self) -> LcdClkDivDenominatorR {
        LcdClkDivDenominatorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Reserved"]
    #[inline(always)]
    pub fn uart0_clk_src_sel(&self) -> Uart0ClkSrcSelR {
        Uart0ClkSrcSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn uart0_clk_en(&self) -> Uart0ClkEnR {
        Uart0ClkEnR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_div_num(&mut self) -> LcdClkDivNumW<'_, PeriClkCtrl110Spec> {
        LcdClkDivNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_div_numerator(&mut self) -> LcdClkDivNumeratorW<'_, PeriClkCtrl110Spec> {
        LcdClkDivNumeratorW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_div_denominator(&mut self) -> LcdClkDivDenominatorW<'_, PeriClkCtrl110Spec> {
        LcdClkDivDenominatorW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Reserved"]
    #[inline(always)]
    pub fn uart0_clk_src_sel(&mut self) -> Uart0ClkSrcSelW<'_, PeriClkCtrl110Spec> {
        Uart0ClkSrcSelW::new(self, 24)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn uart0_clk_en(&mut self) -> Uart0ClkEnW<'_, PeriClkCtrl110Spec> {
        Uart0ClkEnW::new(self, 26)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl110Spec;
impl crate::RegisterSpec for PeriClkCtrl110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl110::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl110Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl110::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL110 to value 0x0400_0000"]
impl crate::Resettable for PeriClkCtrl110Spec {
    const RESET_VALUE: u32 = 0x0400_0000;
}
