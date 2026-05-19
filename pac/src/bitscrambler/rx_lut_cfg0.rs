#[doc = "Register `RX_LUT_CFG0` reader"]
pub type R = crate::R<RxLutCfg0Spec>;
#[doc = "Register `RX_LUT_CFG0` writer"]
pub type W = crate::W<RxLutCfg0Spec>;
#[doc = "Field `RX_LUT_IDX` reader - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
pub type RxLutIdxR = crate::FieldReader<u16>;
#[doc = "Field `RX_LUT_IDX` writer - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
pub type RxLutIdxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RX_LUT_MODE` reader - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
pub type RxLutModeR = crate::FieldReader;
#[doc = "Field `RX_LUT_MODE` writer - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
pub type RxLutModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:10 - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
    #[inline(always)]
    pub fn rx_lut_idx(&self) -> RxLutIdxR {
        RxLutIdxR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
    #[inline(always)]
    pub fn rx_lut_mode(&self) -> RxLutModeR {
        RxLutModeR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
    #[inline(always)]
    pub fn rx_lut_idx(&mut self) -> RxLutIdxW<'_, RxLutCfg0Spec> {
        RxLutIdxW::new(self, 0)
    }
    #[doc = "Bits 11:12 - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
    #[inline(always)]
    pub fn rx_lut_mode(&mut self) -> RxLutModeW<'_, RxLutCfg0Spec> {
        RxLutModeW::new(self, 11)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_lut_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_lut_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxLutCfg0Spec;
impl crate::RegisterSpec for RxLutCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_lut_cfg0::R`](R) reader structure"]
impl crate::Readable for RxLutCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_lut_cfg0::W`](W) writer structure"]
impl crate::Writable for RxLutCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_LUT_CFG0 to value 0"]
impl crate::Resettable for RxLutCfg0Spec {}
