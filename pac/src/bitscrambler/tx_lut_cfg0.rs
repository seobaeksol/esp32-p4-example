#[doc = "Register `TX_LUT_CFG0` reader"]
pub type R = crate::R<TxLutCfg0Spec>;
#[doc = "Register `TX_LUT_CFG0` writer"]
pub type W = crate::W<TxLutCfg0Spec>;
#[doc = "Field `TX_LUT_IDX` reader - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode"]
pub type TxLutIdxR = crate::FieldReader<u16>;
#[doc = "Field `TX_LUT_IDX` writer - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode"]
pub type TxLutIdxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `TX_LUT_MODE` reader - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
pub type TxLutModeR = crate::FieldReader;
#[doc = "Field `TX_LUT_MODE` writer - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
pub type TxLutModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:10 - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode"]
    #[inline(always)]
    pub fn tx_lut_idx(&self) -> TxLutIdxR {
        TxLutIdxR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
    #[inline(always)]
    pub fn tx_lut_mode(&self) -> TxLutModeR {
        TxLutModeR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_tx_lut_mode"]
    #[inline(always)]
    pub fn tx_lut_idx(&mut self) -> TxLutIdxW<'_, TxLutCfg0Spec> {
        TxLutIdxW::new(self, 0)
    }
    #[doc = "Bits 11:12 - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
    #[inline(always)]
    pub fn tx_lut_mode(&mut self) -> TxLutModeW<'_, TxLutCfg0Spec> {
        TxLutModeW::new(self, 11)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_lut_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_lut_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxLutCfg0Spec;
impl crate::RegisterSpec for TxLutCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_lut_cfg0::R`](R) reader structure"]
impl crate::Readable for TxLutCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_lut_cfg0::W`](W) writer structure"]
impl crate::Writable for TxLutCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_LUT_CFG0 to value 0"]
impl crate::Resettable for TxLutCfg0Spec {}
