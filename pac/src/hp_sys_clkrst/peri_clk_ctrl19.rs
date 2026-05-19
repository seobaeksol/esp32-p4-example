#[doc = "Register `PERI_CLK_CTRL19` reader"]
pub type R = crate::R<PeriClkCtrl19Spec>;
#[doc = "Register `PERI_CLK_CTRL19` writer"]
pub type W = crate::W<PeriClkCtrl19Spec>;
#[doc = "Field `I2S2_TX_DIV_X` reader - Reserved"]
pub type I2s2TxDivXR = crate::FieldReader<u16>;
#[doc = "Field `I2S2_TX_DIV_X` writer - Reserved"]
pub type I2s2TxDivXW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S2_TX_DIV_Y` reader - Reserved"]
pub type I2s2TxDivYR = crate::FieldReader<u16>;
#[doc = "Field `I2S2_TX_DIV_Y` writer - Reserved"]
pub type I2s2TxDivYW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S2_TX_DIV_Z` reader - Reserved"]
pub type I2s2TxDivZR = crate::FieldReader<u16>;
#[doc = "Field `I2S2_TX_DIV_Z` writer - Reserved"]
pub type I2s2TxDivZW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S2_TX_DIV_YN1` reader - Reserved"]
pub type I2s2TxDivYn1R = crate::BitReader;
#[doc = "Field `I2S2_TX_DIV_YN1` writer - Reserved"]
pub type I2s2TxDivYn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_MST_CLK_SEL` reader - Reserved"]
pub type I2s2MstClkSelR = crate::BitReader;
#[doc = "Field `I2S2_MST_CLK_SEL` writer - Reserved"]
pub type I2s2MstClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CLK_SRC_SEL` reader - Reserved"]
pub type LcdClkSrcSelR = crate::FieldReader;
#[doc = "Field `LCD_CLK_SRC_SEL` writer - Reserved"]
pub type LcdClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CLK_EN` reader - Reserved"]
pub type LcdClkEnR = crate::BitReader;
#[doc = "Field `LCD_CLK_EN` writer - Reserved"]
pub type LcdClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_x(&self) -> I2s2TxDivXR {
        I2s2TxDivXR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_y(&self) -> I2s2TxDivYR {
        I2s2TxDivYR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_z(&self) -> I2s2TxDivZR {
        I2s2TxDivZR::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_yn1(&self) -> I2s2TxDivYn1R {
        I2s2TxDivYn1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn i2s2_mst_clk_sel(&self) -> I2s2MstClkSelR {
        I2s2MstClkSelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_src_sel(&self) -> LcdClkSrcSelR {
        LcdClkSrcSelR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_en(&self) -> LcdClkEnR {
        LcdClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_x(&mut self) -> I2s2TxDivXW<'_, PeriClkCtrl19Spec> {
        I2s2TxDivXW::new(self, 0)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_y(&mut self) -> I2s2TxDivYW<'_, PeriClkCtrl19Spec> {
        I2s2TxDivYW::new(self, 9)
    }
    #[doc = "Bits 18:26 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_z(&mut self) -> I2s2TxDivZW<'_, PeriClkCtrl19Spec> {
        I2s2TxDivZW::new(self, 18)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2s2_tx_div_yn1(&mut self) -> I2s2TxDivYn1W<'_, PeriClkCtrl19Spec> {
        I2s2TxDivYn1W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn i2s2_mst_clk_sel(&mut self) -> I2s2MstClkSelW<'_, PeriClkCtrl19Spec> {
        I2s2MstClkSelW::new(self, 28)
    }
    #[doc = "Bits 29:30 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_src_sel(&mut self) -> LcdClkSrcSelW<'_, PeriClkCtrl19Spec> {
        LcdClkSrcSelW::new(self, 29)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn lcd_clk_en(&mut self) -> LcdClkEnW<'_, PeriClkCtrl19Spec> {
        LcdClkEnW::new(self, 31)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl19Spec;
impl crate::RegisterSpec for PeriClkCtrl19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl19::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl19Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl19::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL19 to value 0"]
impl crate::Resettable for PeriClkCtrl19Spec {}
