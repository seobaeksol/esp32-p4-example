#[doc = "Register `PERI_CLK_CTRL14` reader"]
pub type R = crate::R<PeriClkCtrl14Spec>;
#[doc = "Register `PERI_CLK_CTRL14` writer"]
pub type W = crate::W<PeriClkCtrl14Spec>;
#[doc = "Field `I2S0_TX_DIV_Y` reader - Reserved"]
pub type I2s0TxDivYR = crate::FieldReader<u16>;
#[doc = "Field `I2S0_TX_DIV_Y` writer - Reserved"]
pub type I2s0TxDivYW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S0_TX_DIV_Z` reader - Reserved"]
pub type I2s0TxDivZR = crate::FieldReader<u16>;
#[doc = "Field `I2S0_TX_DIV_Z` writer - Reserved"]
pub type I2s0TxDivZW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S0_TX_DIV_YN1` reader - Reserved"]
pub type I2s0TxDivYn1R = crate::BitReader;
#[doc = "Field `I2S0_TX_DIV_YN1` writer - Reserved"]
pub type I2s0TxDivYn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_MST_CLK_SEL` reader - Reserved"]
pub type I2s0MstClkSelR = crate::BitReader;
#[doc = "Field `I2S0_MST_CLK_SEL` writer - Reserved"]
pub type I2s0MstClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_RX_CLK_EN` reader - Reserved"]
pub type I2s1RxClkEnR = crate::BitReader;
#[doc = "Field `I2S1_RX_CLK_EN` writer - Reserved"]
pub type I2s1RxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_RX_CLK_SRC_SEL` reader - Reserved"]
pub type I2s1RxClkSrcSelR = crate::FieldReader;
#[doc = "Field `I2S1_RX_CLK_SRC_SEL` writer - Reserved"]
pub type I2s1RxClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S1_RX_DIV_N` reader - Reserved"]
pub type I2s1RxDivNR = crate::FieldReader;
#[doc = "Field `I2S1_RX_DIV_N` writer - Reserved"]
pub type I2s1RxDivNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_y(&self) -> I2s0TxDivYR {
        I2s0TxDivYR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_z(&self) -> I2s0TxDivZR {
        I2s0TxDivZR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_yn1(&self) -> I2s0TxDivYn1R {
        I2s0TxDivYn1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn i2s0_mst_clk_sel(&self) -> I2s0MstClkSelR {
        I2s0MstClkSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_clk_en(&self) -> I2s1RxClkEnR {
        I2s1RxClkEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_clk_src_sel(&self) -> I2s1RxClkSrcSelR {
        I2s1RxClkSrcSelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:30 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_n(&self) -> I2s1RxDivNR {
        I2s1RxDivNR::new(((self.bits >> 23) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_y(&mut self) -> I2s0TxDivYW<'_, PeriClkCtrl14Spec> {
        I2s0TxDivYW::new(self, 0)
    }
    #[doc = "Bits 9:17 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_z(&mut self) -> I2s0TxDivZW<'_, PeriClkCtrl14Spec> {
        I2s0TxDivZW::new(self, 9)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn i2s0_tx_div_yn1(&mut self) -> I2s0TxDivYn1W<'_, PeriClkCtrl14Spec> {
        I2s0TxDivYn1W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn i2s0_mst_clk_sel(&mut self) -> I2s0MstClkSelW<'_, PeriClkCtrl14Spec> {
        I2s0MstClkSelW::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_clk_en(&mut self) -> I2s1RxClkEnW<'_, PeriClkCtrl14Spec> {
        I2s1RxClkEnW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_clk_src_sel(&mut self) -> I2s1RxClkSrcSelW<'_, PeriClkCtrl14Spec> {
        I2s1RxClkSrcSelW::new(self, 21)
    }
    #[doc = "Bits 23:30 - Reserved"]
    #[inline(always)]
    pub fn i2s1_rx_div_n(&mut self) -> I2s1RxDivNW<'_, PeriClkCtrl14Spec> {
        I2s1RxDivNW::new(self, 23)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl14Spec;
impl crate::RegisterSpec for PeriClkCtrl14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl14::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl14Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl14::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL14 to value 0"]
impl crate::Resettable for PeriClkCtrl14Spec {}
