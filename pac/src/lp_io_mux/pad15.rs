#[doc = "Register `PAD15` reader"]
pub type R = crate::R<Pad15Spec>;
#[doc = "Register `PAD15` writer"]
pub type W = crate::W<Pad15Spec>;
#[doc = "Field `REG_PAD15_DRV` reader - Reserved"]
pub type RegPad15DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD15_DRV` writer - Reserved"]
pub type RegPad15DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD15_RDE` reader - Reserved"]
pub type RegPad15RdeR = crate::BitReader;
#[doc = "Field `REG_PAD15_RDE` writer - Reserved"]
pub type RegPad15RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_RUE` reader - Reserved"]
pub type RegPad15RueR = crate::BitReader;
#[doc = "Field `REG_PAD15_RUE` writer - Reserved"]
pub type RegPad15RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad15MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD15_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad15MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_FUN_SEL` reader - function sel"]
pub type RegPad15FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD15_FUN_SEL` writer - function sel"]
pub type RegPad15FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD15_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad15SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD15_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad15SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad15SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD15_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad15SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad15SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD15_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad15SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_FUN_IE` reader - input enable in work mode"]
pub type RegPad15FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD15_FUN_IE` writer - input enable in work mode"]
pub type RegPad15FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD15_FILTER_EN` reader - need des"]
pub type RegPad15FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD15_FILTER_EN` writer - need des"]
pub type RegPad15FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_drv(&self) -> RegPad15DrvR {
        RegPad15DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_rde(&self) -> RegPad15RdeR {
        RegPad15RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_rue(&self) -> RegPad15RueR {
        RegPad15RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad15_mux_sel(&self) -> RegPad15MuxSelR {
        RegPad15MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad15_fun_sel(&self) -> RegPad15FunSelR {
        RegPad15FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_sel(&self) -> RegPad15SlpSelR {
        RegPad15SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_ie(&self) -> RegPad15SlpIeR {
        RegPad15SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_oe(&self) -> RegPad15SlpOeR {
        RegPad15SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad15_fun_ie(&self) -> RegPad15FunIeR {
        RegPad15FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad15_filter_en(&self) -> RegPad15FilterEnR {
        RegPad15FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_drv(&mut self) -> RegPad15DrvW<'_, Pad15Spec> {
        RegPad15DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_rde(&mut self) -> RegPad15RdeW<'_, Pad15Spec> {
        RegPad15RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad15_rue(&mut self) -> RegPad15RueW<'_, Pad15Spec> {
        RegPad15RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad15_mux_sel(&mut self) -> RegPad15MuxSelW<'_, Pad15Spec> {
        RegPad15MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad15_fun_sel(&mut self) -> RegPad15FunSelW<'_, Pad15Spec> {
        RegPad15FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_sel(&mut self) -> RegPad15SlpSelW<'_, Pad15Spec> {
        RegPad15SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_ie(&mut self) -> RegPad15SlpIeW<'_, Pad15Spec> {
        RegPad15SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad15_slp_oe(&mut self) -> RegPad15SlpOeW<'_, Pad15Spec> {
        RegPad15SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad15_fun_ie(&mut self) -> RegPad15FunIeW<'_, Pad15Spec> {
        RegPad15FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad15_filter_en(&mut self) -> RegPad15FilterEnW<'_, Pad15Spec> {
        RegPad15FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad15Spec;
impl crate::RegisterSpec for Pad15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad15::R`](R) reader structure"]
impl crate::Readable for Pad15Spec {}
#[doc = "`write(|w| ..)` method takes [`pad15::W`](W) writer structure"]
impl crate::Writable for Pad15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD15 to value 0x02"]
impl crate::Resettable for Pad15Spec {
    const RESET_VALUE: u32 = 0x02;
}
