#[doc = "Register `TX_INST_CFG1` reader"]
pub type R = crate::R<TxInstCfg1Spec>;
#[doc = "Register `TX_INST_CFG1` writer"]
pub type W = crate::W<TxInstCfg1Spec>;
#[doc = "Field `TX_INST` reader - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
pub type TxInstR = crate::FieldReader<u32>;
#[doc = "Field `TX_INST` writer - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
pub type TxInstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
    #[inline(always)]
    pub fn tx_inst(&self) -> TxInstR {
        TxInstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - write this bits to update instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_TX_INST_CFG0_REG"]
    #[inline(always)]
    pub fn tx_inst(&mut self) -> TxInstW<'_, TxInstCfg1Spec> {
        TxInstW::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_inst_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_inst_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxInstCfg1Spec;
impl crate::RegisterSpec for TxInstCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_inst_cfg1::R`](R) reader structure"]
impl crate::Readable for TxInstCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_inst_cfg1::W`](W) writer structure"]
impl crate::Writable for TxInstCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_INST_CFG1 to value 0x04"]
impl crate::Resettable for TxInstCfg1Spec {
    const RESET_VALUE: u32 = 0x04;
}
