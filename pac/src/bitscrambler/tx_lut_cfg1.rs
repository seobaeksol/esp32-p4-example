#[doc = "Register `TX_LUT_CFG1` reader"]
pub type R = crate::R<TxLutCfg1Spec>;
#[doc = "Register `TX_LUT_CFG1` writer"]
pub type W = crate::W<TxLutCfg1Spec>;
#[doc = "Field `TX_LUT` reader - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG"]
pub type TxLutR = crate::FieldReader<u32>;
#[doc = "Field `TX_LUT` writer - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG"]
pub type TxLutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG"]
    #[inline(always)]
    pub fn tx_lut(&self) -> TxLutR {
        TxLutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG"]
    #[inline(always)]
    pub fn tx_lut(&mut self) -> TxLutW<'_, TxLutCfg1Spec> {
        TxLutW::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_lut_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_lut_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxLutCfg1Spec;
impl crate::RegisterSpec for TxLutCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_lut_cfg1::R`](R) reader structure"]
impl crate::Readable for TxLutCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_lut_cfg1::W`](W) writer structure"]
impl crate::Writable for TxLutCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_LUT_CFG1 to value 0x14"]
impl crate::Resettable for TxLutCfg1Spec {
    const RESET_VALUE: u32 = 0x14;
}
