#[doc = "Register `FUNC0_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func0OutSelCfgSpec>;
#[doc = "Register `FUNC0_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func0OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC0_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc0OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC0_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc0OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC0_OE_SEL` reader - Reserved"]
pub type RegGpioFunc0OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC0_OE_SEL` writer - Reserved"]
pub type RegGpioFunc0OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC0_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc0OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC0_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc0OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC0_OUT_SEL` reader - reg_gpio_func0_out_sel\\[5:1\\]==16 -> output gpio register value to pad"]
pub type RegGpioFunc0OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC0_OUT_SEL` writer - reg_gpio_func0_out_sel\\[5:1\\]==16 -> output gpio register value to pad"]
pub type RegGpioFunc0OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func0_oe_inv_sel(&self) -> RegGpioFunc0OeInvSelR {
        RegGpioFunc0OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func0_oe_sel(&self) -> RegGpioFunc0OeSelR {
        RegGpioFunc0OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func0_out_inv_sel(&self) -> RegGpioFunc0OutInvSelR {
        RegGpioFunc0OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - reg_gpio_func0_out_sel\\[5:1\\]==16 -> output gpio register value to pad"]
    #[inline(always)]
    pub fn reg_gpio_func0_out_sel(&self) -> RegGpioFunc0OutSelR {
        RegGpioFunc0OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func0_oe_inv_sel(&mut self) -> RegGpioFunc0OeInvSelW<'_, Func0OutSelCfgSpec> {
        RegGpioFunc0OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func0_oe_sel(&mut self) -> RegGpioFunc0OeSelW<'_, Func0OutSelCfgSpec> {
        RegGpioFunc0OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func0_out_inv_sel(&mut self) -> RegGpioFunc0OutInvSelW<'_, Func0OutSelCfgSpec> {
        RegGpioFunc0OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - reg_gpio_func0_out_sel\\[5:1\\]==16 -> output gpio register value to pad"]
    #[inline(always)]
    pub fn reg_gpio_func0_out_sel(&mut self) -> RegGpioFunc0OutSelW<'_, Func0OutSelCfgSpec> {
        RegGpioFunc0OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func0_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func0_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func0OutSelCfgSpec;
impl crate::RegisterSpec for Func0OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func0_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func0OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func0_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func0OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC0_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func0OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
