#[doc = "Register `PAD8` reader"]
pub type R = crate::R<Pad8Spec>;
#[doc = "Register `PAD8` writer"]
pub type W = crate::W<Pad8Spec>;
#[doc = "Field `REG_PAD8_DRV` reader - Reserved"]
pub type RegPad8DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD8_DRV` writer - Reserved"]
pub type RegPad8DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD8_RDE` reader - Reserved"]
pub type RegPad8RdeR = crate::BitReader;
#[doc = "Field `REG_PAD8_RDE` writer - Reserved"]
pub type RegPad8RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD8_RUE` reader - Reserved"]
pub type RegPad8RueR = crate::BitReader;
#[doc = "Field `REG_PAD8_RUE` writer - Reserved"]
pub type RegPad8RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD8_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad8MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD8_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad8MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD8_FUN_SEL` reader - function sel"]
pub type RegPad8FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD8_FUN_SEL` writer - function sel"]
pub type RegPad8FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD8_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad8SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD8_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad8SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD8_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad8SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD8_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad8SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD8_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad8SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD8_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad8SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD8_FUN_IE` reader - input enable in work mode"]
pub type RegPad8FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD8_FUN_IE` writer - input enable in work mode"]
pub type RegPad8FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD8_FILTER_EN` reader - need des"]
pub type RegPad8FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD8_FILTER_EN` writer - need des"]
pub type RegPad8FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad8_drv(&self) -> RegPad8DrvR {
        RegPad8DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad8_rde(&self) -> RegPad8RdeR {
        RegPad8RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad8_rue(&self) -> RegPad8RueR {
        RegPad8RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad8_mux_sel(&self) -> RegPad8MuxSelR {
        RegPad8MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad8_fun_sel(&self) -> RegPad8FunSelR {
        RegPad8FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad8_slp_sel(&self) -> RegPad8SlpSelR {
        RegPad8SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad8_slp_ie(&self) -> RegPad8SlpIeR {
        RegPad8SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad8_slp_oe(&self) -> RegPad8SlpOeR {
        RegPad8SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad8_fun_ie(&self) -> RegPad8FunIeR {
        RegPad8FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad8_filter_en(&self) -> RegPad8FilterEnR {
        RegPad8FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad8_drv(&mut self) -> RegPad8DrvW<'_, Pad8Spec> {
        RegPad8DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad8_rde(&mut self) -> RegPad8RdeW<'_, Pad8Spec> {
        RegPad8RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad8_rue(&mut self) -> RegPad8RueW<'_, Pad8Spec> {
        RegPad8RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad8_mux_sel(&mut self) -> RegPad8MuxSelW<'_, Pad8Spec> {
        RegPad8MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad8_fun_sel(&mut self) -> RegPad8FunSelW<'_, Pad8Spec> {
        RegPad8FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad8_slp_sel(&mut self) -> RegPad8SlpSelW<'_, Pad8Spec> {
        RegPad8SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad8_slp_ie(&mut self) -> RegPad8SlpIeW<'_, Pad8Spec> {
        RegPad8SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad8_slp_oe(&mut self) -> RegPad8SlpOeW<'_, Pad8Spec> {
        RegPad8SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad8_fun_ie(&mut self) -> RegPad8FunIeW<'_, Pad8Spec> {
        RegPad8FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad8_filter_en(&mut self) -> RegPad8FilterEnW<'_, Pad8Spec> {
        RegPad8FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad8Spec;
impl crate::RegisterSpec for Pad8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad8::R`](R) reader structure"]
impl crate::Readable for Pad8Spec {}
#[doc = "`write(|w| ..)` method takes [`pad8::W`](W) writer structure"]
impl crate::Writable for Pad8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD8 to value 0x02"]
impl crate::Resettable for Pad8Spec {
    const RESET_VALUE: u32 = 0x02;
}
