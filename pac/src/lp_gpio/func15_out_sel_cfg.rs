#[doc = "Register `FUNC15_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func15OutSelCfgSpec>;
#[doc = "Register `FUNC15_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func15OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC15_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc15OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC15_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc15OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC15_OE_SEL` reader - Reserved"]
pub type RegGpioFunc15OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC15_OE_SEL` writer - Reserved"]
pub type RegGpioFunc15OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC15_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc15OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC15_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc15OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC15_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc15OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC15_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc15OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_oe_inv_sel(&self) -> RegGpioFunc15OeInvSelR {
        RegGpioFunc15OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_oe_sel(&self) -> RegGpioFunc15OeSelR {
        RegGpioFunc15OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_out_inv_sel(&self) -> RegGpioFunc15OutInvSelR {
        RegGpioFunc15OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_out_sel(&self) -> RegGpioFunc15OutSelR {
        RegGpioFunc15OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_oe_inv_sel(
        &mut self,
    ) -> RegGpioFunc15OeInvSelW<'_, Func15OutSelCfgSpec> {
        RegGpioFunc15OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_oe_sel(&mut self) -> RegGpioFunc15OeSelW<'_, Func15OutSelCfgSpec> {
        RegGpioFunc15OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_out_inv_sel(
        &mut self,
    ) -> RegGpioFunc15OutInvSelW<'_, Func15OutSelCfgSpec> {
        RegGpioFunc15OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func15_out_sel(&mut self) -> RegGpioFunc15OutSelW<'_, Func15OutSelCfgSpec> {
        RegGpioFunc15OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func15_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func15_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func15OutSelCfgSpec;
impl crate::RegisterSpec for Func15OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func15_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func15OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func15_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func15OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC15_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func15OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
