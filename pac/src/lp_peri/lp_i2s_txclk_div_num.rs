#[doc = "Register `LP_I2S_TXCLK_DIV_NUM` reader"]
pub type R = crate::R<LpI2sTxclkDivNumSpec>;
#[doc = "Register `LP_I2S_TXCLK_DIV_NUM` writer"]
pub type W = crate::W<LpI2sTxclkDivNumSpec>;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_NUM` reader - need_des"]
pub type LpI2sTxClkmDivNumR = crate::FieldReader;
#[doc = "Field `LP_I2S_TX_CLKM_DIV_NUM` writer - need_des"]
pub type LpI2sTxClkmDivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_num(&self) -> LpI2sTxClkmDivNumR {
        LpI2sTxClkmDivNumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn lp_i2s_tx_clkm_div_num(&mut self) -> LpI2sTxClkmDivNumW<'_, LpI2sTxclkDivNumSpec> {
        LpI2sTxClkmDivNumW::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_i2s_txclk_div_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_i2s_txclk_div_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpI2sTxclkDivNumSpec;
impl crate::RegisterSpec for LpI2sTxclkDivNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_i2s_txclk_div_num::R`](R) reader structure"]
impl crate::Readable for LpI2sTxclkDivNumSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_i2s_txclk_div_num::W`](W) writer structure"]
impl crate::Writable for LpI2sTxclkDivNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_I2S_TXCLK_DIV_NUM to value 0x0200_0000"]
impl crate::Resettable for LpI2sTxclkDivNumSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
