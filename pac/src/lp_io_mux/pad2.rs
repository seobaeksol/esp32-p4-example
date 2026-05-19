#[doc = "Register `PAD2` reader"]
pub type R = crate::R<Pad2Spec>;
#[doc = "Register `PAD2` writer"]
pub type W = crate::W<Pad2Spec>;
#[doc = "Field `REG_PAD2_DRV` reader - Reserved"]
pub type RegPad2DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD2_DRV` writer - Reserved"]
pub type RegPad2DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD2_RDE` reader - Reserved"]
pub type RegPad2RdeR = crate::BitReader;
#[doc = "Field `REG_PAD2_RDE` writer - Reserved"]
pub type RegPad2RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD2_RUE` reader - Reserved"]
pub type RegPad2RueR = crate::BitReader;
#[doc = "Field `REG_PAD2_RUE` writer - Reserved"]
pub type RegPad2RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD2_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad2MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD2_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad2MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD2_FUN_SEL` reader - function sel"]
pub type RegPad2FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD2_FUN_SEL` writer - function sel"]
pub type RegPad2FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD2_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad2SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD2_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad2SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD2_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad2SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD2_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad2SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD2_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad2SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD2_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad2SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD2_FUN_IE` reader - input enable in work mode"]
pub type RegPad2FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD2_FUN_IE` writer - input enable in work mode"]
pub type RegPad2FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD2_FILTER_EN` reader - need des"]
pub type RegPad2FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD2_FILTER_EN` writer - need des"]
pub type RegPad2FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad2_drv(&self) -> RegPad2DrvR {
        RegPad2DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad2_rde(&self) -> RegPad2RdeR {
        RegPad2RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad2_rue(&self) -> RegPad2RueR {
        RegPad2RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad2_mux_sel(&self) -> RegPad2MuxSelR {
        RegPad2MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad2_fun_sel(&self) -> RegPad2FunSelR {
        RegPad2FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad2_slp_sel(&self) -> RegPad2SlpSelR {
        RegPad2SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad2_slp_ie(&self) -> RegPad2SlpIeR {
        RegPad2SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad2_slp_oe(&self) -> RegPad2SlpOeR {
        RegPad2SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad2_fun_ie(&self) -> RegPad2FunIeR {
        RegPad2FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad2_filter_en(&self) -> RegPad2FilterEnR {
        RegPad2FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad2_drv(&mut self) -> RegPad2DrvW<'_, Pad2Spec> {
        RegPad2DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad2_rde(&mut self) -> RegPad2RdeW<'_, Pad2Spec> {
        RegPad2RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad2_rue(&mut self) -> RegPad2RueW<'_, Pad2Spec> {
        RegPad2RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad2_mux_sel(&mut self) -> RegPad2MuxSelW<'_, Pad2Spec> {
        RegPad2MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad2_fun_sel(&mut self) -> RegPad2FunSelW<'_, Pad2Spec> {
        RegPad2FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad2_slp_sel(&mut self) -> RegPad2SlpSelW<'_, Pad2Spec> {
        RegPad2SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad2_slp_ie(&mut self) -> RegPad2SlpIeW<'_, Pad2Spec> {
        RegPad2SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad2_slp_oe(&mut self) -> RegPad2SlpOeW<'_, Pad2Spec> {
        RegPad2SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad2_fun_ie(&mut self) -> RegPad2FunIeW<'_, Pad2Spec> {
        RegPad2FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad2_filter_en(&mut self) -> RegPad2FilterEnW<'_, Pad2Spec> {
        RegPad2FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad2Spec;
impl crate::RegisterSpec for Pad2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad2::R`](R) reader structure"]
impl crate::Readable for Pad2Spec {}
#[doc = "`write(|w| ..)` method takes [`pad2::W`](W) writer structure"]
impl crate::Writable for Pad2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD2 to value 0x02"]
impl crate::Resettable for Pad2Spec {
    const RESET_VALUE: u32 = 0x02;
}
