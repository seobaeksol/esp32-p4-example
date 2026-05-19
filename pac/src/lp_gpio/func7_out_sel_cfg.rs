#[doc = "Register `FUNC7_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func7OutSelCfgSpec>;
#[doc = "Register `FUNC7_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func7OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC7_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc7OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC7_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc7OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC7_OE_SEL` reader - Reserved"]
pub type RegGpioFunc7OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC7_OE_SEL` writer - Reserved"]
pub type RegGpioFunc7OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC7_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc7OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC7_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc7OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC7_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc7OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC7_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc7OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_oe_inv_sel(&self) -> RegGpioFunc7OeInvSelR {
        RegGpioFunc7OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_oe_sel(&self) -> RegGpioFunc7OeSelR {
        RegGpioFunc7OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_out_inv_sel(&self) -> RegGpioFunc7OutInvSelR {
        RegGpioFunc7OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_out_sel(&self) -> RegGpioFunc7OutSelR {
        RegGpioFunc7OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_oe_inv_sel(&mut self) -> RegGpioFunc7OeInvSelW<'_, Func7OutSelCfgSpec> {
        RegGpioFunc7OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_oe_sel(&mut self) -> RegGpioFunc7OeSelW<'_, Func7OutSelCfgSpec> {
        RegGpioFunc7OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_out_inv_sel(&mut self) -> RegGpioFunc7OutInvSelW<'_, Func7OutSelCfgSpec> {
        RegGpioFunc7OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func7_out_sel(&mut self) -> RegGpioFunc7OutSelW<'_, Func7OutSelCfgSpec> {
        RegGpioFunc7OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func7_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func7_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func7OutSelCfgSpec;
impl crate::RegisterSpec for Func7OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func7_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func7OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func7_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func7OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC7_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func7OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
