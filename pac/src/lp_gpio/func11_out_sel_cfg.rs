#[doc = "Register `FUNC11_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func11OutSelCfgSpec>;
#[doc = "Register `FUNC11_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func11OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC11_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc11OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC11_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc11OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC11_OE_SEL` reader - Reserved"]
pub type RegGpioFunc11OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC11_OE_SEL` writer - Reserved"]
pub type RegGpioFunc11OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC11_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc11OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC11_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc11OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC11_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc11OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC11_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc11OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_inv_sel(&self) -> RegGpioFunc11OeInvSelR {
        RegGpioFunc11OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_sel(&self) -> RegGpioFunc11OeSelR {
        RegGpioFunc11OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_inv_sel(&self) -> RegGpioFunc11OutInvSelR {
        RegGpioFunc11OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_sel(&self) -> RegGpioFunc11OutSelR {
        RegGpioFunc11OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_inv_sel(
        &mut self,
    ) -> RegGpioFunc11OeInvSelW<'_, Func11OutSelCfgSpec> {
        RegGpioFunc11OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_oe_sel(&mut self) -> RegGpioFunc11OeSelW<'_, Func11OutSelCfgSpec> {
        RegGpioFunc11OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_inv_sel(
        &mut self,
    ) -> RegGpioFunc11OutInvSelW<'_, Func11OutSelCfgSpec> {
        RegGpioFunc11OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func11_out_sel(&mut self) -> RegGpioFunc11OutSelW<'_, Func11OutSelCfgSpec> {
        RegGpioFunc11OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func11_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func11_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func11OutSelCfgSpec;
impl crate::RegisterSpec for Func11OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func11_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func11OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func11_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func11OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC11_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func11OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
