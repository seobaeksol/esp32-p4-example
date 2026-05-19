#[doc = "Register `PERI_CLK_CTRL13` reader"]
pub type R = crate::R<PeriClkCtrl13Spec>;
#[doc = "Register `PERI_CLK_CTRL13` writer"]
pub type W = crate::W<PeriClkCtrl13Spec>;
#[doc = "Field `I2S0_RX_DIV_Z` reader - Reserved"]
pub type I2s0RxDivZR = crate::FieldReader<u16>;
#[doc = "Field `I2S0_RX_DIV_Z` writer - Reserved"]
pub type I2s0RxDivZW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S0_RX_DIV_YN1` reader - Reserved"]
pub type I2s0RxDivYn1R = crate::BitReader;
#[doc = "Field `I2S0_RX_DIV_YN1` writer - Reserved"]
pub type I2s0RxDivYn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TX_CLK_EN` reader - Reserved"]
pub type I2s0TxClkEnR = crate::BitReader;
#[doc = "Field `I2S0_TX_CLK_EN` writer - Reserved"]
pub type I2s0TxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TX_CLK_SRC_SEL` reader - Reserved"]
pub type I2s0TxClkSrcSelR = crate::FieldReader;
#[doc = "Field `I2S0_TX_CLK_SRC_SEL` writer - Reserved"]
pub type I2s0TxClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S0_TX_DIV_N` reader - Reserved"]
pub type I2s0TxDivNR = crate::FieldReader;
#[doc = "Field `I2S0_TX_DIV_N` writer - Reserved"]
pub type I2s0TxDivNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2S0_TX_DIV_X` reader - Reserved"]
pub type I2s0TxDivXR = crate::FieldReader<u16>;
#[doc = "Field `I2S0_TX_DIV_X` writer - Reserved"]
pub type I2s0TxDivXW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_z(&self) -> I2s0RxDivZR {
        I2s0RxDivZR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_yn1(&self) -> I2s0RxDivYn1R {
        I2s0RxDivYn1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_clk_en(&self) -> I2s0TxClkEnR {
        I2s0TxClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_clk_src_sel(&self) -> I2s0TxClkSrcSelR {
        I2s0TxClkSrcSelR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:20 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_n(&self) -> I2s0TxDivNR {
        I2s0TxDivNR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:29 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_x(&self) -> I2s0TxDivXR {
        I2s0TxDivXR::new(((self.bits >> 21) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_z(&mut self) -> I2s0RxDivZW<'_, PeriClkCtrl13Spec> {
        I2s0RxDivZW::new(self, 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn i2s0_rx_div_yn1(&mut self) -> I2s0RxDivYn1W<'_, PeriClkCtrl13Spec> {
        I2s0RxDivYn1W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_clk_en(&mut self) -> I2s0TxClkEnW<'_, PeriClkCtrl13Spec> {
        I2s0TxClkEnW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_clk_src_sel(&mut self) -> I2s0TxClkSrcSelW<'_, PeriClkCtrl13Spec> {
        I2s0TxClkSrcSelW::new(self, 11)
    }
    #[doc = "Bits 13:20 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_n(&mut self) -> I2s0TxDivNW<'_, PeriClkCtrl13Spec> {
        I2s0TxDivNW::new(self, 13)
    }
    #[doc = "Bits 21:29 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_x(&mut self) -> I2s0TxDivXW<'_, PeriClkCtrl13Spec> {
        I2s0TxDivXW::new(self, 21)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl13Spec;
impl crate::RegisterSpec for PeriClkCtrl13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl13::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl13Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl13::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL13 to value 0"]
impl crate::Resettable for PeriClkCtrl13Spec {}
