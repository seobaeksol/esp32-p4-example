#[doc = "Register `PERI_CLK_CTRL17` reader"]
pub type R = crate::R<PeriClkCtrl17Spec>;
#[doc = "Register `PERI_CLK_CTRL17` writer"]
pub type W = crate::W<PeriClkCtrl17Spec>;
#[doc = "Field `I2S1_TX_DIV_Z` reader - Reserved"]
pub type I2s1TxDivZR = crate::FieldReader<u16>;
#[doc = "Field `I2S1_TX_DIV_Z` writer - Reserved"]
pub type I2s1TxDivZW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `I2S1_TX_DIV_YN1` reader - Reserved"]
pub type I2s1TxDivYn1R = crate::BitReader;
#[doc = "Field `I2S1_TX_DIV_YN1` writer - Reserved"]
pub type I2s1TxDivYn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_MST_CLK_SEL` reader - Reserved"]
pub type I2s1MstClkSelR = crate::BitReader;
#[doc = "Field `I2S1_MST_CLK_SEL` writer - Reserved"]
pub type I2s1MstClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_RX_CLK_EN` reader - Reserved"]
pub type I2s2RxClkEnR = crate::BitReader;
#[doc = "Field `I2S2_RX_CLK_EN` writer - Reserved"]
pub type I2s2RxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_RX_CLK_SRC_SEL` reader - Reserved"]
pub type I2s2RxClkSrcSelR = crate::FieldReader;
#[doc = "Field `I2S2_RX_CLK_SRC_SEL` writer - Reserved"]
pub type I2s2RxClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S2_RX_DIV_N` reader - Reserved"]
pub type I2s2RxDivNR = crate::FieldReader;
#[doc = "Field `I2S2_RX_DIV_N` writer - Reserved"]
pub type I2s2RxDivNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2S2_RX_DIV_X` reader - Reserved"]
pub type I2s2RxDivXR = crate::FieldReader<u16>;
#[doc = "Field `I2S2_RX_DIV_X` writer - Reserved"]
pub type I2s2RxDivXW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_div_z(&self) -> I2s1TxDivZR {
        I2s1TxDivZR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_div_yn1(&self) -> I2s1TxDivYn1R {
        I2s1TxDivYn1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn i2s1_mst_clk_sel(&self) -> I2s1MstClkSelR {
        I2s1MstClkSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_clk_en(&self) -> I2s2RxClkEnR {
        I2s2RxClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_clk_src_sel(&self) -> I2s2RxClkSrcSelR {
        I2s2RxClkSrcSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_div_n(&self) -> I2s2RxDivNR {
        I2s2RxDivNR::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:30 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_div_x(&self) -> I2s2RxDivXR {
        I2s2RxDivXR::new(((self.bits >> 22) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_div_z(&mut self) -> I2s1TxDivZW<'_, PeriClkCtrl17Spec> {
        I2s1TxDivZW::new(self, 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn i2s1_tx_div_yn1(&mut self) -> I2s1TxDivYn1W<'_, PeriClkCtrl17Spec> {
        I2s1TxDivYn1W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn i2s1_mst_clk_sel(&mut self) -> I2s1MstClkSelW<'_, PeriClkCtrl17Spec> {
        I2s1MstClkSelW::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_clk_en(&mut self) -> I2s2RxClkEnW<'_, PeriClkCtrl17Spec> {
        I2s2RxClkEnW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_clk_src_sel(&mut self) -> I2s2RxClkSrcSelW<'_, PeriClkCtrl17Spec> {
        I2s2RxClkSrcSelW::new(self, 12)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_div_n(&mut self) -> I2s2RxDivNW<'_, PeriClkCtrl17Spec> {
        I2s2RxDivNW::new(self, 14)
    }
    #[doc = "Bits 22:30 - Reserved"]
    #[inline(always)]
    pub fn i2s2_rx_div_x(&mut self) -> I2s2RxDivXW<'_, PeriClkCtrl17Spec> {
        I2s2RxDivXW::new(self, 22)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl17Spec;
impl crate::RegisterSpec for PeriClkCtrl17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl17::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl17Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl17::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL17 to value 0"]
impl crate::Resettable for PeriClkCtrl17Spec {}
