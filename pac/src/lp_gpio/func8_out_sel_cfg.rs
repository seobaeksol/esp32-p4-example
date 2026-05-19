#[doc = "Register `FUNC8_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func8OutSelCfgSpec>;
#[doc = "Register `FUNC8_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func8OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC8_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc8OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC8_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc8OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC8_OE_SEL` reader - Reserved"]
pub type RegGpioFunc8OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC8_OE_SEL` writer - Reserved"]
pub type RegGpioFunc8OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC8_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc8OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC8_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc8OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC8_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc8OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC8_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc8OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_oe_inv_sel(&self) -> RegGpioFunc8OeInvSelR {
        RegGpioFunc8OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_oe_sel(&self) -> RegGpioFunc8OeSelR {
        RegGpioFunc8OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_out_inv_sel(&self) -> RegGpioFunc8OutInvSelR {
        RegGpioFunc8OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_out_sel(&self) -> RegGpioFunc8OutSelR {
        RegGpioFunc8OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_oe_inv_sel(&mut self) -> RegGpioFunc8OeInvSelW<'_, Func8OutSelCfgSpec> {
        RegGpioFunc8OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_oe_sel(&mut self) -> RegGpioFunc8OeSelW<'_, Func8OutSelCfgSpec> {
        RegGpioFunc8OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_out_inv_sel(&mut self) -> RegGpioFunc8OutInvSelW<'_, Func8OutSelCfgSpec> {
        RegGpioFunc8OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_out_sel(&mut self) -> RegGpioFunc8OutSelW<'_, Func8OutSelCfgSpec> {
        RegGpioFunc8OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func8_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func8_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func8OutSelCfgSpec;
impl crate::RegisterSpec for Func8OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func8_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func8OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func8_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func8OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC8_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func8OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
