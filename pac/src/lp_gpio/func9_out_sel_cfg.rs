#[doc = "Register `FUNC9_OUT_SEL_CFG` reader"]
pub type R = crate::R<Func9OutSelCfgSpec>;
#[doc = "Register `FUNC9_OUT_SEL_CFG` writer"]
pub type W = crate::W<Func9OutSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC9_OE_INV_SEL` reader - Reserved"]
pub type RegGpioFunc9OeInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC9_OE_INV_SEL` writer - Reserved"]
pub type RegGpioFunc9OeInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC9_OE_SEL` reader - Reserved"]
pub type RegGpioFunc9OeSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC9_OE_SEL` writer - Reserved"]
pub type RegGpioFunc9OeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC9_OUT_INV_SEL` reader - Reserved"]
pub type RegGpioFunc9OutInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC9_OUT_INV_SEL` writer - Reserved"]
pub type RegGpioFunc9OutInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC9_OUT_SEL` reader - Reserved"]
pub type RegGpioFunc9OutSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC9_OUT_SEL` writer - Reserved"]
pub type RegGpioFunc9OutSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_oe_inv_sel(&self) -> RegGpioFunc9OeInvSelR {
        RegGpioFunc9OeInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_oe_sel(&self) -> RegGpioFunc9OeSelR {
        RegGpioFunc9OeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_out_inv_sel(&self) -> RegGpioFunc9OutInvSelR {
        RegGpioFunc9OutInvSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_out_sel(&self) -> RegGpioFunc9OutSelR {
        RegGpioFunc9OutSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_oe_inv_sel(&mut self) -> RegGpioFunc9OeInvSelW<'_, Func9OutSelCfgSpec> {
        RegGpioFunc9OeInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_oe_sel(&mut self) -> RegGpioFunc9OeSelW<'_, Func9OutSelCfgSpec> {
        RegGpioFunc9OeSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_out_inv_sel(&mut self) -> RegGpioFunc9OutInvSelW<'_, Func9OutSelCfgSpec> {
        RegGpioFunc9OutInvSelW::new(self, 2)
    }
    #[doc = "Bits 3:8 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func9_out_sel(&mut self) -> RegGpioFunc9OutSelW<'_, Func9OutSelCfgSpec> {
        RegGpioFunc9OutSelW::new(self, 3)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func9_out_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func9_out_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func9OutSelCfgSpec;
impl crate::RegisterSpec for Func9OutSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func9_out_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func9OutSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func9_out_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func9OutSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC9_OUT_SEL_CFG to value 0x0100"]
impl crate::Resettable for Func9OutSelCfgSpec {
    const RESET_VALUE: u32 = 0x0100;
}
