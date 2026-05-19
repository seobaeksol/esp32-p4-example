#[doc = "Register `PAD1` reader"]
pub type R = crate::R<Pad1Spec>;
#[doc = "Register `PAD1` writer"]
pub type W = crate::W<Pad1Spec>;
#[doc = "Field `REG_PAD1_DRV` reader - Reserved"]
pub type RegPad1DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD1_DRV` writer - Reserved"]
pub type RegPad1DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD1_RDE` reader - Reserved"]
pub type RegPad1RdeR = crate::BitReader;
#[doc = "Field `REG_PAD1_RDE` writer - Reserved"]
pub type RegPad1RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD1_RUE` reader - Reserved"]
pub type RegPad1RueR = crate::BitReader;
#[doc = "Field `REG_PAD1_RUE` writer - Reserved"]
pub type RegPad1RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD1_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad1MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD1_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad1MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD1_FUN_SEL` reader - function sel"]
pub type RegPad1FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD1_FUN_SEL` writer - function sel"]
pub type RegPad1FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD1_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad1SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD1_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad1SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD1_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad1SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD1_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad1SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD1_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad1SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD1_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad1SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD1_FUN_IE` reader - input enable in work mode"]
pub type RegPad1FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD1_FUN_IE` writer - input enable in work mode"]
pub type RegPad1FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD1_FILTER_EN` reader - need des"]
pub type RegPad1FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD1_FILTER_EN` writer - need des"]
pub type RegPad1FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad1_drv(&self) -> RegPad1DrvR {
        RegPad1DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad1_rde(&self) -> RegPad1RdeR {
        RegPad1RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad1_rue(&self) -> RegPad1RueR {
        RegPad1RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad1_mux_sel(&self) -> RegPad1MuxSelR {
        RegPad1MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad1_fun_sel(&self) -> RegPad1FunSelR {
        RegPad1FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad1_slp_sel(&self) -> RegPad1SlpSelR {
        RegPad1SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad1_slp_ie(&self) -> RegPad1SlpIeR {
        RegPad1SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad1_slp_oe(&self) -> RegPad1SlpOeR {
        RegPad1SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad1_fun_ie(&self) -> RegPad1FunIeR {
        RegPad1FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad1_filter_en(&self) -> RegPad1FilterEnR {
        RegPad1FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad1_drv(&mut self) -> RegPad1DrvW<'_, Pad1Spec> {
        RegPad1DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad1_rde(&mut self) -> RegPad1RdeW<'_, Pad1Spec> {
        RegPad1RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad1_rue(&mut self) -> RegPad1RueW<'_, Pad1Spec> {
        RegPad1RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad1_mux_sel(&mut self) -> RegPad1MuxSelW<'_, Pad1Spec> {
        RegPad1MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad1_fun_sel(&mut self) -> RegPad1FunSelW<'_, Pad1Spec> {
        RegPad1FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad1_slp_sel(&mut self) -> RegPad1SlpSelW<'_, Pad1Spec> {
        RegPad1SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad1_slp_ie(&mut self) -> RegPad1SlpIeW<'_, Pad1Spec> {
        RegPad1SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad1_slp_oe(&mut self) -> RegPad1SlpOeW<'_, Pad1Spec> {
        RegPad1SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad1_fun_ie(&mut self) -> RegPad1FunIeW<'_, Pad1Spec> {
        RegPad1FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad1_filter_en(&mut self) -> RegPad1FilterEnW<'_, Pad1Spec> {
        RegPad1FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad1Spec;
impl crate::RegisterSpec for Pad1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad1::R`](R) reader structure"]
impl crate::Readable for Pad1Spec {}
#[doc = "`write(|w| ..)` method takes [`pad1::W`](W) writer structure"]
impl crate::Writable for Pad1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD1 to value 0x02"]
impl crate::Resettable for Pad1Spec {
    const RESET_VALUE: u32 = 0x02;
}
