#[doc = "Register `FUNC5_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func5OutSelCfgSpec>;
#[doc = "Register `FUNC5_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func5OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC5_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc5OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC5_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc5OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC5_OE_SEL` reader - Reserved"]
pub type RegGpioFunc5OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC5_OE_SEL` writer - Reserved"]
pub type RegGpioFunc5OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC5_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc5OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC5_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc5OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC5_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc5OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC5_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc5OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_oe_inv_sel(&self) -> RegGpioFunc5OeInvSelR {
        RegGpioFunc5OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_oe_sel(&self) -> RegGpioFunc5OeSelR {
        RegGpioFunc5OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_out_inv_sel(&self) -> RegGpioFunc5OutInvSelR {
        RegGpioFunc5OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_out_sel(&self) -> RegGpioFunc5OutSelR {
        RegGpioFunc5OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_oe_inv_sel(&mut self) -> RegGpioFunc5OeInvSelW<'_, Func5OutSelCfgSpec> {
        RegGpioFunc5OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_oe_sel(&mut self) -> RegGpioFunc5OeSelW<'_, Func5OutSelCfgSpec> {
        RegGpioFunc5OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_out_inv_sel(&mut self) -> RegGpioFunc5OutInvSelW<'_, Func5OutSelCfgSpec> {
        RegGpioFunc5OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func5_out_sel(&mut self) -> RegGpioFunc5OutSelW<'_, Func5OutSelCfgSpec> {
        RegGpioFunc5OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func5_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func5_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func5OutSelCfgSpec;
impl crate::RegisterSpec for Func5OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func5_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func5OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func5_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func5OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC5_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func5OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
