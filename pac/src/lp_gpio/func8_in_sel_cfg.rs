#[doc = "Register `FUNC8_IN_SEL_CFG` reader"]
pub type R = crate::R<Func8InSelCfgSpec>;
#[doc = "Register `FUNC8_IN_SEL_CFG` writer"]
pub type W = crate::W<Func8InSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC8_IN_INV_SEL` reader - Reserved"]
pub type RegGpioFunc8InInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC8_IN_INV_SEL` writer - Reserved"]
pub type RegGpioFunc8InInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_SIG8_IN_SEL` reader - Reserved"]
pub type RegGpioSig8InSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_SIG8_IN_SEL` writer - Reserved"]
pub type RegGpioSig8InSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC8_IN_SEL` reader - Reserved"]
pub type RegGpioFunc8InSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC8_IN_SEL` writer - Reserved"]
pub type RegGpioFunc8InSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_in_inv_sel(&self) -> RegGpioFunc8InInvSelR {
        RegGpioFunc8InInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig8_in_sel(&self) -> RegGpioSig8InSelR {
        RegGpioSig8InSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_in_sel(&self) -> RegGpioFunc8InSelR {
        RegGpioFunc8InSelR::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_in_inv_sel(&mut self) -> RegGpioFunc8InInvSelW<'_, Func8InSelCfgSpec> {
        RegGpioFunc8InInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig8_in_sel(&mut self) -> RegGpioSig8InSelW<'_, Func8InSelCfgSpec> {
        RegGpioSig8InSelW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func8_in_sel(&mut self) -> RegGpioFunc8InSelW<'_, Func8InSelCfgSpec> {
        RegGpioFunc8InSelW::new(self, 2)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func8_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func8_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func8InSelCfgSpec;
impl crate::RegisterSpec for Func8InSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func8_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func8InSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func8_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func8InSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC8_IN_SEL_CFG to value 0x80"]
impl crate::Resettable for Func8InSelCfgSpec {
    const RESET_VALUE: u32 = 0x80;
}
