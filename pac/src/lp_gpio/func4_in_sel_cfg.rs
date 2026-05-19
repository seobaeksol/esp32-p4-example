#[doc = "Register `FUNC4_IN_SEL_CFG` reader"]
pub type R = crate::R<Func4InSelCfgSpec>;
#[doc = "Register `FUNC4_IN_SEL_CFG` writer"]
pub type W = crate::W<Func4InSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC4_IN_INV_SEL` reader - Reserved"]
pub type RegGpioFunc4InInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC4_IN_INV_SEL` writer - Reserved"]
pub type RegGpioFunc4InInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_SIG4_IN_SEL` reader - Reserved"]
pub type RegGpioSig4InSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_SIG4_IN_SEL` writer - Reserved"]
pub type RegGpioSig4InSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC4_IN_SEL` reader - Reserved"]
pub type RegGpioFunc4InSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC4_IN_SEL` writer - Reserved"]
pub type RegGpioFunc4InSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func4_in_inv_sel(&self) -> RegGpioFunc4InInvSelR {
        RegGpioFunc4InInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig4_in_sel(&self) -> RegGpioSig4InSelR {
        RegGpioSig4InSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func4_in_sel(&self) -> RegGpioFunc4InSelR {
        RegGpioFunc4InSelR::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func4_in_inv_sel(&mut self) -> RegGpioFunc4InInvSelW<'_, Func4InSelCfgSpec> {
        RegGpioFunc4InInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig4_in_sel(&mut self) -> RegGpioSig4InSelW<'_, Func4InSelCfgSpec> {
        RegGpioSig4InSelW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func4_in_sel(&mut self) -> RegGpioFunc4InSelW<'_, Func4InSelCfgSpec> {
        RegGpioFunc4InSelW::new(self, 2)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func4_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func4_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func4InSelCfgSpec;
impl crate::RegisterSpec for Func4InSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func4_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func4InSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func4_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func4InSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC4_IN_SEL_CFG to value 0xc0"]
impl crate::Resettable for Func4InSelCfgSpec {
    const RESET_VALUE: u32 = 0xc0;
}
