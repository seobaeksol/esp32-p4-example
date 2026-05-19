#[doc = "Register `RX_INST_CFG0` reader"]
pub type R = crate::R<RxInstCfg0Spec>;
#[doc = "Register `RX_INST_CFG0` writer"]
pub type W = crate::W<RxInstCfg0Spec>;
#[doc = "Field `RX_INST_IDX` reader - write this bits to specify the one of 8 instruction"]
pub type RxInstIdxR = crate::FieldReader;
#[doc = "Field `RX_INST_IDX` writer - write this bits to specify the one of 8 instruction"]
pub type RxInstIdxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_INST_POS` reader - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type RxInstPosR = crate::FieldReader;
#[doc = "Field `RX_INST_POS` writer - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type RxInstPosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    pub fn rx_inst_idx(&self) -> RxInstIdxR {
        RxInstIdxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
    #[inline(always)]
    pub fn rx_inst_pos(&self) -> RxInstPosR {
        RxInstPosR::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    pub fn rx_inst_idx(&mut self) -> RxInstIdxW<'_, RxInstCfg0Spec> {
        RxInstIdxW::new(self, 0)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
    #[inline(always)]
    pub fn rx_inst_pos(&mut self) -> RxInstPosW<'_, RxInstCfg0Spec> {
        RxInstPosW::new(self, 3)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_inst_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_inst_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxInstCfg0Spec;
impl crate::RegisterSpec for RxInstCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_inst_cfg0::R`](R) reader structure"]
impl crate::Readable for RxInstCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_inst_cfg0::W`](W) writer structure"]
impl crate::Writable for RxInstCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_INST_CFG0 to value 0"]
impl crate::Resettable for RxInstCfg0Spec {}
