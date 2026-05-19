#[doc = "Register `PAD13` reader"]
pub type R = crate::R<Pad13Spec>;
#[doc = "Register `PAD13` writer"]
pub type W = crate::W<Pad13Spec>;
#[doc = "Field `REG_PAD13_DRV` reader - Reserved"]
pub type RegPad13DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD13_DRV` writer - Reserved"]
pub type RegPad13DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD13_RDE` reader - Reserved"]
pub type RegPad13RdeR = crate::BitReader;
#[doc = "Field `REG_PAD13_RDE` writer - Reserved"]
pub type RegPad13RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD13_RUE` reader - Reserved"]
pub type RegPad13RueR = crate::BitReader;
#[doc = "Field `REG_PAD13_RUE` writer - Reserved"]
pub type RegPad13RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD13_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad13MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD13_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad13MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD13_FUN_SEL` reader - function sel"]
pub type RegPad13FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD13_FUN_SEL` writer - function sel"]
pub type RegPad13FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD13_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad13SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD13_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad13SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD13_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad13SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD13_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad13SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD13_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad13SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD13_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad13SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD13_FUN_IE` reader - input enable in work mode"]
pub type RegPad13FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD13_FUN_IE` writer - input enable in work mode"]
pub type RegPad13FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD13_FILTER_EN` reader - need des"]
pub type RegPad13FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD13_FILTER_EN` writer - need des"]
pub type RegPad13FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad13_drv(&self) -> RegPad13DrvR {
        RegPad13DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad13_rde(&self) -> RegPad13RdeR {
        RegPad13RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad13_rue(&self) -> RegPad13RueR {
        RegPad13RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad13_mux_sel(&self) -> RegPad13MuxSelR {
        RegPad13MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad13_fun_sel(&self) -> RegPad13FunSelR {
        RegPad13FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad13_slp_sel(&self) -> RegPad13SlpSelR {
        RegPad13SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad13_slp_ie(&self) -> RegPad13SlpIeR {
        RegPad13SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad13_slp_oe(&self) -> RegPad13SlpOeR {
        RegPad13SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad13_fun_ie(&self) -> RegPad13FunIeR {
        RegPad13FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad13_filter_en(&self) -> RegPad13FilterEnR {
        RegPad13FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad13_drv(&mut self) -> RegPad13DrvW<'_, Pad13Spec> {
        RegPad13DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad13_rde(&mut self) -> RegPad13RdeW<'_, Pad13Spec> {
        RegPad13RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad13_rue(&mut self) -> RegPad13RueW<'_, Pad13Spec> {
        RegPad13RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad13_mux_sel(&mut self) -> RegPad13MuxSelW<'_, Pad13Spec> {
        RegPad13MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad13_fun_sel(&mut self) -> RegPad13FunSelW<'_, Pad13Spec> {
        RegPad13FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad13_slp_sel(&mut self) -> RegPad13SlpSelW<'_, Pad13Spec> {
        RegPad13SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad13_slp_ie(&mut self) -> RegPad13SlpIeW<'_, Pad13Spec> {
        RegPad13SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad13_slp_oe(&mut self) -> RegPad13SlpOeW<'_, Pad13Spec> {
        RegPad13SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad13_fun_ie(&mut self) -> RegPad13FunIeW<'_, Pad13Spec> {
        RegPad13FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad13_filter_en(&mut self) -> RegPad13FilterEnW<'_, Pad13Spec> {
        RegPad13FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad13Spec;
impl crate::RegisterSpec for Pad13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad13::R`](R) reader structure"]
impl crate::Readable for Pad13Spec {}
#[doc = "`write(|w| ..)` method takes [`pad13::W`](W) writer structure"]
impl crate::Writable for Pad13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD13 to value 0x02"]
impl crate::Resettable for Pad13Spec {
    const RESET_VALUE: u32 = 0x02;
}
