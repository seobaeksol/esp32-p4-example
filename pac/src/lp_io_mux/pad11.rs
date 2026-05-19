#[doc = "Register `PAD11` reader"]
pub type R = crate::R<Pad11Spec>;
#[doc = "Register `PAD11` writer"]
pub type W = crate::W<Pad11Spec>;
#[doc = "Field `REG_PAD11_DRV` reader - Reserved"]
pub type RegPad11DrvR = crate::FieldReader;
#[doc = "Field `REG_PAD11_DRV` writer - Reserved"]
pub type RegPad11DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD11_RDE` reader - Reserved"]
pub type RegPad11RdeR = crate::BitReader;
#[doc = "Field `REG_PAD11_RDE` writer - Reserved"]
pub type RegPad11RdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD11_RUE` reader - Reserved"]
pub type RegPad11RueR = crate::BitReader;
#[doc = "Field `REG_PAD11_RUE` writer - Reserved"]
pub type RegPad11RueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD11_MUX_SEL` reader - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad11MuxSelR = crate::BitReader;
#[doc = "Field `REG_PAD11_MUX_SEL` writer - 1:use LP GPIO,0: use digital GPIO"]
pub type RegPad11MuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD11_FUN_SEL` reader - function sel"]
pub type RegPad11FunSelR = crate::FieldReader;
#[doc = "Field `REG_PAD11_FUN_SEL` writer - function sel"]
pub type RegPad11FunSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PAD11_SLP_SEL` reader - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad11SlpSelR = crate::BitReader;
#[doc = "Field `REG_PAD11_SLP_SEL` writer - 1: enable sleep mode during sleep,0: no sleep mode"]
pub type RegPad11SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD11_SLP_IE` reader - input enable in sleep mode"]
pub type RegPad11SlpIeR = crate::BitReader;
#[doc = "Field `REG_PAD11_SLP_IE` writer - input enable in sleep mode"]
pub type RegPad11SlpIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD11_SLP_OE` reader - output enable in sleep mode"]
pub type RegPad11SlpOeR = crate::BitReader;
#[doc = "Field `REG_PAD11_SLP_OE` writer - output enable in sleep mode"]
pub type RegPad11SlpOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD11_FUN_IE` reader - input enable in work mode"]
pub type RegPad11FunIeR = crate::BitReader;
#[doc = "Field `REG_PAD11_FUN_IE` writer - input enable in work mode"]
pub type RegPad11FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PAD11_FILTER_EN` reader - need des"]
pub type RegPad11FilterEnR = crate::BitReader;
#[doc = "Field `REG_PAD11_FILTER_EN` writer - need des"]
pub type RegPad11FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad11_drv(&self) -> RegPad11DrvR {
        RegPad11DrvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad11_rde(&self) -> RegPad11RdeR {
        RegPad11RdeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad11_rue(&self) -> RegPad11RueR {
        RegPad11RueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad11_mux_sel(&self) -> RegPad11MuxSelR {
        RegPad11MuxSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad11_fun_sel(&self) -> RegPad11FunSelR {
        RegPad11FunSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad11_slp_sel(&self) -> RegPad11SlpSelR {
        RegPad11SlpSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad11_slp_ie(&self) -> RegPad11SlpIeR {
        RegPad11SlpIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad11_slp_oe(&self) -> RegPad11SlpOeR {
        RegPad11SlpOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad11_fun_ie(&self) -> RegPad11FunIeR {
        RegPad11FunIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad11_filter_en(&self) -> RegPad11FilterEnR {
        RegPad11FilterEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_pad11_drv(&mut self) -> RegPad11DrvW<'_, Pad11Spec> {
        RegPad11DrvW::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_pad11_rde(&mut self) -> RegPad11RdeW<'_, Pad11Spec> {
        RegPad11RdeW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_pad11_rue(&mut self) -> RegPad11RueW<'_, Pad11Spec> {
        RegPad11RueW::new(self, 3)
    }
    #[doc = "Bit 4 - 1:use LP GPIO,0: use digital GPIO"]
    #[inline(always)]
    pub fn reg_pad11_mux_sel(&mut self) -> RegPad11MuxSelW<'_, Pad11Spec> {
        RegPad11MuxSelW::new(self, 4)
    }
    #[doc = "Bits 5:6 - function sel"]
    #[inline(always)]
    pub fn reg_pad11_fun_sel(&mut self) -> RegPad11FunSelW<'_, Pad11Spec> {
        RegPad11FunSelW::new(self, 5)
    }
    #[doc = "Bit 7 - 1: enable sleep mode during sleep,0: no sleep mode"]
    #[inline(always)]
    pub fn reg_pad11_slp_sel(&mut self) -> RegPad11SlpSelW<'_, Pad11Spec> {
        RegPad11SlpSelW::new(self, 7)
    }
    #[doc = "Bit 8 - input enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad11_slp_ie(&mut self) -> RegPad11SlpIeW<'_, Pad11Spec> {
        RegPad11SlpIeW::new(self, 8)
    }
    #[doc = "Bit 9 - output enable in sleep mode"]
    #[inline(always)]
    pub fn reg_pad11_slp_oe(&mut self) -> RegPad11SlpOeW<'_, Pad11Spec> {
        RegPad11SlpOeW::new(self, 9)
    }
    #[doc = "Bit 10 - input enable in work mode"]
    #[inline(always)]
    pub fn reg_pad11_fun_ie(&mut self) -> RegPad11FunIeW<'_, Pad11Spec> {
        RegPad11FunIeW::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn reg_pad11_filter_en(&mut self) -> RegPad11FilterEnW<'_, Pad11Spec> {
        RegPad11FilterEnW::new(self, 11)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`pad11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad11Spec;
impl crate::RegisterSpec for Pad11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad11::R`](R) reader structure"]
impl crate::Readable for Pad11Spec {}
#[doc = "`write(|w| ..)` method takes [`pad11::W`](W) writer structure"]
impl crate::Writable for Pad11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD11 to value 0x02"]
impl crate::Resettable for Pad11Spec {
    const RESET_VALUE: u32 = 0x02;
}
