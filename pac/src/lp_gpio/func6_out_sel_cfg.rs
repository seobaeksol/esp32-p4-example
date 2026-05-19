#[doc = "Register `FUNC6_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func6OutSelCfgSpec>;
#[doc = "Register `FUNC6_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func6OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC6_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc6OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC6_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc6OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC6_OE_SEL` reader - Reserved"]
pub type RegGpioFunc6OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC6_OE_SEL` writer - Reserved"]
pub type RegGpioFunc6OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC6_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc6OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC6_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc6OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC6_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc6OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC6_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc6OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_oe_inv_sel(&self) -> RegGpioFunc6OeInvSelR {
        RegGpioFunc6OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_oe_sel(&self) -> RegGpioFunc6OeSelR {
        RegGpioFunc6OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_out_inv_sel(&self) -> RegGpioFunc6OutInvSelR {
        RegGpioFunc6OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_out_sel(&self) -> RegGpioFunc6OutSelR {
        RegGpioFunc6OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_oe_inv_sel(&mut self) -> RegGpioFunc6OeInvSelW<'_, Func6OutSelCfgSpec> {
        RegGpioFunc6OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_oe_sel(&mut self) -> RegGpioFunc6OeSelW<'_, Func6OutSelCfgSpec> {
        RegGpioFunc6OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_out_inv_sel(&mut self) -> RegGpioFunc6OutInvSelW<'_, Func6OutSelCfgSpec> {
        RegGpioFunc6OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func6_out_sel(&mut self) -> RegGpioFunc6OutSelW<'_, Func6OutSelCfgSpec> {
        RegGpioFunc6OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func6_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func6_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func6OutSelCfgSpec;
impl crate::RegisterSpec for Func6OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func6_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func6OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func6_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func6OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC6_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func6OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
