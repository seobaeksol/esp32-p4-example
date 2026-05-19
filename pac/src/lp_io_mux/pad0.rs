#[doc = "Register `PAD0` reader"]
pub type R = crate::R<Pad0Spec>;
#[doc = "Register `PAD0` writer"]
pub type W = crate::W<Pad0Spec>;
#[doc = "Field `REG_PAD0_DRV` reader - Reserved"]
pub type RegPad0DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD0_DRV` writer - Reserved"]
pub type RegPad0DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD0_RDE` reader - Reserved"]
pub type RegPad0RdeR = crate::BitReader;
#[doc = "Field `REG_PAD0_RDE` writer - Reserved"]
pub type RegPad0RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD0_RUE` reader - Reserved"]
pub type RegPad0RueR = crate::BitReader;
#[doc = "Field `REG_PAD0_RUE` writer - Reserved"]
pub type RegPad0RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD0_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad0MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD0_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad0MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD0_FUN_SEL` reader - function sel"]
pub type RegPad0FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD0_FUN_SEL` writer - function sel"]
pub type RegPad0FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD0_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad0SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD0_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad0SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD0_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad0SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD0_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad0SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD0_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad0SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD0_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad0SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD0_FUN_IE` reader - input enable in work mode"]
pub type RegPad0FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD0_FUN_IE` writer - input enable in work mode"]
pub type RegPad0FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD0_FILTER_EN` reader - need des"]
pub type RegPad0FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD0_FILTER_EN` writer - need des"]
pub type RegPad0FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad0_drv(&self) -> RegPad0DrvR {
        RegPad0DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad0_rde(&self) -> RegPad0RdeR {
        RegPad0RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad0_rue(&self) -> RegPad0RueR {
        RegPad0RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad0_mux_sel(&self) -> RegPad0MuxSelR {
        RegPad0MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad0_fun_sel(&self) -> RegPad0FunSelR {
        RegPad0FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad0_slp_sel(&self) -> RegPad0SlpSelR {
        RegPad0SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad0_slp_ie(&self) -> RegPad0SlpIeR {
        RegPad0SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad0_slp_oe(&self) -> RegPad0SlpOeR {
        RegPad0SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad0_fun_ie(&self) -> RegPad0FunIeR {
        RegPad0FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad0_filter_en(&self) -> RegPad0FilterEnR {
        RegPad0FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad0_drv(&mut self) -> RegPad0DrvW<'_, Pad0Spec> {
        RegPad0DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad0_rde(&mut self) -> RegPad0RdeW<'_, Pad0Spec> {
        RegPad0RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad0_rue(&mut self) -> RegPad0RueW<'_, Pad0Spec> {
        RegPad0RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad0_mux_sel(&mut self) -> RegPad0MuxSelW<'_, Pad0Spec> {
        RegPad0MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad0_fun_sel(&mut self) -> RegPad0FunSelW<'_, Pad0Spec> {
        RegPad0FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad0_slp_sel(&mut self) -> RegPad0SlpSelW<'_, Pad0Spec> {
        RegPad0SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad0_slp_ie(&mut self) -> RegPad0SlpIeW<'_, Pad0Spec> {
        RegPad0SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad0_slp_oe(&mut self) -> RegPad0SlpOeW<'_, Pad0Spec> {
        RegPad0SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad0_fun_ie(&mut self) -> RegPad0FunIeW<'_, Pad0Spec> {
        RegPad0FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad0_filter_en(&mut self) -> RegPad0FilterEnW<'_, Pad0Spec> {
        RegPad0FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad0Spec;
impl crate::RegisterSpec for Pad0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad0::R`](R) reader structure"]
impl crate::Readable for Pad0Spec {}
#[doc = "`write(|w| ..)` method takes [`pad0::W`](W) writer structure"]
impl crate::Writable for Pad0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD0 to value 0x02"]
impl crate::Resettable for Pad0Spec {
    const RESET_VALUE: u32 = 0x02;
}
