#[doc = "Register `FUNC14_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func14OutSelCfgSpec>;
#[doc = "Register `FUNC14_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func14OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC14_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc14OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC14_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc14OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC14_OE_SEL` reader - Reserved"]
pub type RegGpioFunc14OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC14_OE_SEL` writer - Reserved"]
pub type RegGpioFunc14OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC14_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc14OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC14_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc14OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC14_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc14OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC14_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc14OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_oe_inv_sel(&self) -> RegGpioFunc14OeInvSelR {
        RegGpioFunc14OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_oe_sel(&self) -> RegGpioFunc14OeSelR {
        RegGpioFunc14OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_out_inv_sel(&self) -> RegGpioFunc14OutInvSelR {
        RegGpioFunc14OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_out_sel(&self) -> RegGpioFunc14OutSelR {
        RegGpioFunc14OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_oe_inv_sel(
        &mut self,
    ) -> RegGpioFunc14OeInvSelW<'_, Func14OutSelCfgSpec> {
        RegGpioFunc14OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_oe_sel(&mut self) -> RegGpioFunc14OeSelW<'_, Func14OutSelCfgSpec> {
        RegGpioFunc14OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_out_inv_sel(
        &mut self,
    ) -> RegGpioFunc14OutInvSelW<'_, Func14OutSelCfgSpec> {
        RegGpioFunc14OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func14_out_sel(&mut self) -> RegGpioFunc14OutSelW<'_, Func14OutSelCfgSpec> {
        RegGpioFunc14OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func14_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func14_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func14OutSelCfgSpec;
impl crate::RegisterSpec for Func14OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func14_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func14OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func14_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func14OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC14_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func14OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
