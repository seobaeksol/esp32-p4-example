#[doc = "Register `PERI_CLK_CTRL118` reader"]
pub type R = crate::R<PeriClkCtrl118Spec>;
#[doc = "Register `PERI_CLK_CTRL118` writer"]
pub type W = crate::W<PeriClkCtrl118Spec>;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUMERATOR` reader - Reserved"]
pub type ParlioRxClkDivNumeratorR = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_DIV_NUMERATOR` writer - Reserved"]
pub type ParlioRxClkDivNumeratorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARLIO_RX_CLK_DIV_DENOMINATOR` reader - Reserved"]
pub type ParlioRxClkDivDenominatorR = crate::FieldReader;
#[doc = "Field `PARLIO_RX_CLK_DIV_DENOMINATOR` writer - Reserved"]
pub type ParlioRxClkDivDenominatorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PARLIO_TX_CLK_SRC_SEL` reader - Reserved"]
pub type ParlioTxClkSrcSelR = crate::FieldReader;
#[doc = "Field `PARLIO_TX_CLK_SRC_SEL` writer - Reserved"]
pub type ParlioTxClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PARLIO_TX_CLK_EN` reader - Reserved"]
pub type ParlioTxClkEnR = crate::BitReader;
#[doc = "Field `PARLIO_TX_CLK_EN` writer - Reserved"]
pub type ParlioTxClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARLIO_TX_CLK_DIV_NUM` reader - Reserved"]
pub type ParlioTxClkDivNumR = crate::FieldReader;
#[doc = "Field `PARLIO_TX_CLK_DIV_NUM` writer - Reserved"]
pub type ParlioTxClkDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_numerator(&self) -> ParlioRxClkDivNumeratorR {
        ParlioRxClkDivNumeratorR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_denominator(&self) -> ParlioRxClkDivDenominatorR {
        ParlioRxClkDivDenominatorR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Reserved"]
    #[inline(always)]
    pub fn parlio_tx_clk_src_sel(&self) -> ParlioTxClkSrcSelR {
        ParlioTxClkSrcSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn parlio_tx_clk_en(&self) -> ParlioTxClkEnR {
        ParlioTxClkEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - Reserved"]
    #[inline(always)]
    pub fn parlio_tx_clk_div_num(&self) -> ParlioTxClkDivNumR {
        ParlioTxClkDivNumR::new(((self.bits >> 19) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_numerator(
        &mut self,
    ) -> ParlioRxClkDivNumeratorW<'_, PeriClkCtrl118Spec> {
        ParlioRxClkDivNumeratorW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn parlio_rx_clk_div_denominator(
        &mut self,
    ) -> ParlioRxClkDivDenominatorW<'_, PeriClkCtrl118Spec> {
        ParlioRxClkDivDenominatorW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Reserved"]
    #[inline(always)]
    pub fn parlio_tx_clk_src_sel(&mut self) -> ParlioTxClkSrcSelW<'_, PeriClkCtrl118Spec> {
        ParlioTxClkSrcSelW::new(self, 16)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn parlio_tx_clk_en(&mut self) -> ParlioTxClkEnW<'_, PeriClkCtrl118Spec> {
        ParlioTxClkEnW::new(self, 18)
    }
    #[doc = "Bits 19:26 - Reserved"]
    #[inline(always)]
    pub fn parlio_tx_clk_div_num(&mut self) -> ParlioTxClkDivNumW<'_, PeriClkCtrl118Spec> {
        ParlioTxClkDivNumW::new(self, 19)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriClkCtrl118Spec;
impl crate::RegisterSpec for PeriClkCtrl118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl118::R`](R) reader structure"]
impl crate::Readable for PeriClkCtrl118Spec {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl118::W`](W) writer structure"]
impl crate::Writable for PeriClkCtrl118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL118 to value 0"]
impl crate::Resettable for PeriClkCtrl118Spec {}
