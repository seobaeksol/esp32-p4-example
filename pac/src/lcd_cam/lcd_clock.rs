#[doc = "Register `LCD_CLOCK` reader"]
pub type R = crate::R<LcdClockSpec>;
#[doc = "Register `LCD_CLOCK` writer"]
pub type W = crate::W<LcdClockSpec>;
#[doc = "Field `LCD_CLKCNT_N` reader - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub type LcdClkcntNR = crate::FieldReader;
#[doc = "Field `LCD_CLKCNT_N` writer - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
pub type LcdClkcntNW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LCD_CLK_EQU_SYSCLK` reader - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub type LcdClkEquSysclkR = crate::BitReader;
#[doc = "Field `LCD_CLK_EQU_SYSCLK` writer - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
pub type LcdClkEquSysclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CK_IDLE_EDGE` reader - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub type LcdCkIdleEdgeR = crate::BitReader;
#[doc = "Field `LCD_CK_IDLE_EDGE` writer - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
pub type LcdCkIdleEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CK_OUT_EDGE` reader - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
pub type LcdCkOutEdgeR = crate::BitReader;
#[doc = "Field `LCD_CK_OUT_EDGE` writer - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
pub type LcdCkOutEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CLKM_DIV_NUM` reader - Integral LCD clock divider value"]
pub type LcdClkmDivNumR = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_NUM` writer - Integral LCD clock divider value"]
pub type LcdClkmDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LCD_CLKM_DIV_B` reader - Fractional clock divider numerator value"]
pub type LcdClkmDivBR = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_B` writer - Fractional clock divider numerator value"]
pub type LcdClkmDivBW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LCD_CLKM_DIV_A` reader - Fractional clock divider denominator value"]
pub type LcdClkmDivAR = crate::FieldReader;
#[doc = "Field `LCD_CLKM_DIV_A` writer - Fractional clock divider denominator value"]
pub type LcdClkmDivAW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LCD_CLK_SEL` reader - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type LcdClkSelR = crate::FieldReader;
#[doc = "Field `LCD_CLK_SEL` writer - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
pub type LcdClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLK_EN` reader - Set this bit to enable clk gate"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set this bit to enable clk gate"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
    #[inline(always)]
    pub fn lcd_clkcnt_n(&self) -> LcdClkcntNR {
        LcdClkcntNR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
    #[inline(always)]
    pub fn lcd_clk_equ_sysclk(&self) -> LcdClkEquSysclkR {
        LcdClkEquSysclkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
    #[inline(always)]
    pub fn lcd_ck_idle_edge(&self) -> LcdCkIdleEdgeR {
        LcdCkIdleEdgeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
    #[inline(always)]
    pub fn lcd_ck_out_edge(&self) -> LcdCkOutEdgeR {
        LcdCkOutEdgeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - Integral LCD clock divider value"]
    #[inline(always)]
    pub fn lcd_clkm_div_num(&self) -> LcdClkmDivNumR {
        LcdClkmDivNumR::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_b(&self) -> LcdClkmDivBR {
        LcdClkmDivBR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_a(&self) -> LcdClkmDivAR {
        LcdClkmDivAR::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bits 29:30 - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn lcd_clk_sel(&self) -> LcdClkSelR {
        LcdClkSelR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1) when reg_clk_equ_sysclk is 0."]
    #[inline(always)]
    pub fn lcd_clkcnt_n(&mut self) -> LcdClkcntNW<'_, LcdClockSpec> {
        LcdClkcntNW::new(self, 0)
    }
    #[doc = "Bit 6 - 1: f_LCD_PCLK = f_LCD_CLK. 0: f_LCD_PCLK = f_LCD_CLK / (reg_clkcnt_N + 1)."]
    #[inline(always)]
    pub fn lcd_clk_equ_sysclk(&mut self) -> LcdClkEquSysclkW<'_, LcdClockSpec> {
        LcdClkEquSysclkW::new(self, 6)
    }
    #[doc = "Bit 7 - 1: LCD_PCLK line is high when idle 0: LCD_PCLK line is low when idle."]
    #[inline(always)]
    pub fn lcd_ck_idle_edge(&mut self) -> LcdCkIdleEdgeW<'_, LcdClockSpec> {
        LcdCkIdleEdgeW::new(self, 7)
    }
    #[doc = "Bit 8 - 1: LCD_PCLK line is high in the first half data cycle. 0: LCD_PCLK line is low in the second half data cycle."]
    #[inline(always)]
    pub fn lcd_ck_out_edge(&mut self) -> LcdCkOutEdgeW<'_, LcdClockSpec> {
        LcdCkOutEdgeW::new(self, 8)
    }
    #[doc = "Bits 9:16 - Integral LCD clock divider value"]
    #[inline(always)]
    pub fn lcd_clkm_div_num(&mut self) -> LcdClkmDivNumW<'_, LcdClockSpec> {
        LcdClkmDivNumW::new(self, 9)
    }
    #[doc = "Bits 17:22 - Fractional clock divider numerator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_b(&mut self) -> LcdClkmDivBW<'_, LcdClockSpec> {
        LcdClkmDivBW::new(self, 17)
    }
    #[doc = "Bits 23:28 - Fractional clock divider denominator value"]
    #[inline(always)]
    pub fn lcd_clkm_div_a(&mut self) -> LcdClkmDivAW<'_, LcdClockSpec> {
        LcdClkmDivAW::new(self, 23)
    }
    #[doc = "Bits 29:30 - Select LCD module source clock. 0: no clock. 1: APLL. 2: CLK160. 3: no clock."]
    #[inline(always)]
    pub fn lcd_clk_sel(&mut self) -> LcdClkSelW<'_, LcdClockSpec> {
        LcdClkSelW::new(self, 29)
    }
    #[doc = "Bit 31 - Set this bit to enable clk gate"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, LcdClockSpec> {
        ClkEnW::new(self, 31)
    }
}
#[doc = "LCD clock config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdClockSpec;
impl crate::RegisterSpec for LcdClockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_clock::R`](R) reader structure"]
impl crate::Readable for LcdClockSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_clock::W`](W) writer structure"]
impl crate::Writable for LcdClockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_CLOCK to value 0x0843"]
impl crate::Resettable for LcdClockSpec {
    const RESET_VALUE: u32 = 0x0843;
}
