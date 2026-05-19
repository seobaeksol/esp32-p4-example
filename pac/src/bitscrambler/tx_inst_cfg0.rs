#[doc = "Register `TX_INST_CFG0` reader"]
pub type R = crate::R<TxInstCfg0Spec>;
#[doc = "Register `TX_INST_CFG0` writer"]
pub type W = crate::W<TxInstCfg0Spec>;
#[doc = "Field `TX_INST_IDX` reader - write this bits to specify the one of 8 instruction"]
pub type TxInstIdxR = crate::FieldReader;
#[doc = "Field `TX_INST_IDX` writer - write this bits to specify the one of 8 instruction"]
pub type TxInstIdxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_INST_POS` reader - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type TxInstPosR = crate::FieldReader;
#[doc = "Field `TX_INST_POS` writer - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type TxInstPosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    pub fn tx_inst_idx(&self) -> TxInstIdxR {
        TxInstIdxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
    #[inline(always)]
    pub fn tx_inst_pos(&self) -> TxInstPosR {
        TxInstPosR::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    pub fn tx_inst_idx(&mut self) -> TxInstIdxW<'_, TxInstCfg0Spec> {
        TxInstIdxW::new(self, 0)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
    #[inline(always)]
    pub fn tx_inst_pos(&mut self) -> TxInstPosW<'_, TxInstCfg0Spec> {
        TxInstPosW::new(self, 3)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_inst_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_inst_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxInstCfg0Spec;
impl crate::RegisterSpec for TxInstCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_inst_cfg0::R`](R) reader structure"]
impl crate::Readable for TxInstCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_inst_cfg0::W`](W) writer structure"]
impl crate::Writable for TxInstCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_INST_CFG0 to value 0"]
impl crate::Resettable for TxInstCfg0Spec {}
