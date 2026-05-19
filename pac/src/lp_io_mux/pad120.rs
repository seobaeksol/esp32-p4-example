#[doc = "Register `PAD120` reader"]
pub type R = crate::R<Pad120Spec>;
#[doc = "Register `PAD120` writer"]
pub type W = crate::W<Pad120Spec>;
#[doc = "Field `REG_PAD12_DRV` reader - Reserved"]
pub type RegPad12DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD12_DRV` writer - Reserved"]
pub type RegPad12DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD12_RDE` reader - Reserved"]
pub type RegPad12RdeR = crate::BitReader;
#[doc = "Field `REG_PAD12_RDE` writer - Reserved"]
pub type RegPad12RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD12_RUE` reader - Reserved"]
pub type RegPad12RueR = crate::BitReader;
#[doc = "Field `REG_PAD12_RUE` writer - Reserved"]
pub type RegPad12RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD12_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad12MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD12_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad12MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD12_FUN_SEL` reader - function sel"]
pub type RegPad12FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD12_FUN_SEL` writer - function sel"]
pub type RegPad12FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD12_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad12SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD12_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad12SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD12_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad12SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD12_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad12SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD12_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad12SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD12_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad12SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD12_FUN_IE` reader - input enable in work mode"]
pub type RegPad12FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD12_FUN_IE` writer - input enable in work mode"]
pub type RegPad12FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD12_FILTER_EN` reader - need des"]
pub type RegPad12FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD12_FILTER_EN` writer - need des"]
pub type RegPad12FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad12_drv(&self) -> RegPad12DrvR {
        RegPad12DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad12_rde(&self) -> RegPad12RdeR {
        RegPad12RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad12_rue(&self) -> RegPad12RueR {
        RegPad12RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad12_mux_sel(&self) -> RegPad12MuxSelR {
        RegPad12MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad12_fun_sel(&self) -> RegPad12FunSelR {
        RegPad12FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad12_slp_sel(&self) -> RegPad12SlpSelR {
        RegPad12SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad12_slp_ie(&self) -> RegPad12SlpIeR {
        RegPad12SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad12_slp_oe(&self) -> RegPad12SlpOeR {
        RegPad12SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad12_fun_ie(&self) -> RegPad12FunIeR {
        RegPad12FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad12_filter_en(&self) -> RegPad12FilterEnR {
        RegPad12FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad12_drv(&mut self) -> RegPad12DrvW<'_, Pad120Spec> {
        RegPad12DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad12_rde(&mut self) -> RegPad12RdeW<'_, Pad120Spec> {
        RegPad12RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad12_rue(&mut self) -> RegPad12RueW<'_, Pad120Spec> {
        RegPad12RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad12_mux_sel(&mut self) -> RegPad12MuxSelW<'_, Pad120Spec> {
        RegPad12MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad12_fun_sel(&mut self) -> RegPad12FunSelW<'_, Pad120Spec> {
        RegPad12FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad12_slp_sel(&mut self) -> RegPad12SlpSelW<'_, Pad120Spec> {
        RegPad12SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad12_slp_ie(&mut self) -> RegPad12SlpIeW<'_, Pad120Spec> {
        RegPad12SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad12_slp_oe(&mut self) -> RegPad12SlpOeW<'_, Pad120Spec> {
        RegPad12SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad12_fun_ie(&mut self) -> RegPad12FunIeW<'_, Pad120Spec> {
        RegPad12FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad12_filter_en(&mut self) -> RegPad12FilterEnW<'_, Pad120Spec> {
        RegPad12FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad120Spec;
impl crate::RegisterSpec for Pad120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad120::R`](R) reader structure"]
impl crate::Readable for Pad120Spec {}
#[doc = "`write(|w| ..)` method takes [`pad120::W`](W) writer structure"]
impl crate::Writable for Pad120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD120 to value 0x02"]
impl crate::Resettable for Pad120Spec {
    const RESET_VALUE: u32 = 0x02;
}
