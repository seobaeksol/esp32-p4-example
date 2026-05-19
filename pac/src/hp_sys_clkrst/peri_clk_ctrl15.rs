#[doc = "Register `PERI_CLK_CTRL15` reader"]
pub type R = crate::R<PeriClkCtrl15Spec>;
#[doc = "Register `PERI_CLK_CTRL15` writer"]
pub type W = crate::W<PeriClkCtrl15Spec>;
#[doc = "Field `I2S1_RX_DIV_X` reader - Reserved"]
pub type I2s1RxDivXR = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_X` writer - Reserved"]
pub type I2s1RxDivXW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_Y` reader - Reserved"]
pub type I2s1RxDivYR = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_Y` writer - Reserved"]
pub type I2s1RxDivYW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_Z` reader - Reserved"]
pub type I2s1RxDivZR = crate::FieldReader<u16>;
#[doc = "Field `I2S1_RX_DIV_Z` writer - Reserved"]
pub type I2s1RxDivZW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_RX_DIV_YN1` reader - Reserved"]
pub type I2s1RxDivYn1R = crate::BitReader;
#[doc = "Field `I2S1_RX_DIV_YN1` writer - Reserved"]
pub type I2s1RxDivYn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TX_CLK_EN` reader - Reserved"]
pub type I2s1TxClkEnR = crate::BitReader;
#[doc = "Field `I2S1_TX_CLK_EN` writer - Reserved"]
pub type I2s1TxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_TX_CLK_SRC_SEL` reader - Reserved"]
pub type I2s1TxClkSrcSelR = crate::FieldReader;
#[doc = "Field `I2S1_TX_CLK_SRC_SEL` writer - Reserved"]
pub type I2s1TxClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_x(&self) -> I2s1RxDivXR {
        I2s1RxDivXR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_y(&self) -> I2s1RxDivYR {
        I2s1RxDivYR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_z(&self) -> I2s1RxDivZR {
        I2s1RxDivZR::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_yn1(&self) -> I2s1RxDivYn1R {
        I2s1RxDivYn1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_en(&self) -> I2s1TxClkEnR {
        I2s1TxClkEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_src_sel(&self) -> I2s1TxClkSrcSelR {
        I2s1TxClkSrcSelR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_x(&mut self) -> I2s1RxDivXW<'_, PeriClkCtrl15Spec> {
        I2s1RxDivXW::new(self, 0)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_y(&mut self) -> I2s1RxDivYW<'_, PeriClkCtrl15Spec> {
        I2s1RxDivYW::new(self, 9)
    }
    #[doc = "Bits 18:26 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_z(&mut self) -> I2s1RxDivZW<'_, PeriClkCtrl15Spec> {
        I2s1RxDivZW::new(self, 18)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_yn1(&mut self) -> I2s1RxDivYn1W<'_, PeriClkCtrl15Spec> {
        I2s1RxDivYn1W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_en(&mut self) -> I2s1TxClkEnW<'_, PeriClkCtrl15Spec> {
        I2s1TxClkEnW::new(self, 28)
    }
    #[doc = "Bits 29:30 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_clk_src_sel(&mut self) -> I2s1TxClkSrcSelW<'_, PeriClkCtrl15Spec> {
        I2s1TxClkSrcSelW::new(self, 29)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl15Spec;
impl crate::RegisterSpec for PeriClkCtrl15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl15::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl15Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl15::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL15 to value 0"]
impl crate::Resettable for PeriClkCtrl15Spec {}
