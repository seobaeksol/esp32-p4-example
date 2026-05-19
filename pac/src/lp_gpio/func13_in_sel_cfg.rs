#[doc = "Register `FUNC13_IN_SEL_CFG` reader"]
pub type R = crate::R<Func13InSelCfgSpec>;
#[doc = "Register `FUNC13_IN_SEL_CFG` writer"]
pub type W = crate::W<Func13InSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC13_IN_INV_SEL` reader - Reserved"]
pub type RegGpioFunc13InInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC13_IN_INV_SEL` writer - Reserved"]
pub type RegGpioFunc13InInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_SIG13_IN_SEL` reader - Reserved"]
pub type RegGpioSig13InSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_SIG13_IN_SEL` writer - Reserved"]
pub type RegGpioSig13InSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC13_IN_SEL` reader - Reserved"]
pub type RegGpioFunc13InSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC13_IN_SEL` writer - Reserved"]
pub type RegGpioFunc13InSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func13_in_inv_sel(&self) -> RegGpioFunc13InInvSelR {
        RegGpioFunc13InInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig13_in_sel(&self) -> RegGpioSig13InSelR {
        RegGpioSig13InSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func13_in_sel(&self) -> RegGpioFunc13InSelR {
        RegGpioFunc13InSelR::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func13_in_inv_sel(&mut self) -> RegGpioFunc13InInvSelW<'_, Func13InSelCfgSpec> {
        RegGpioFunc13InInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig13_in_sel(&mut self) -> RegGpioSig13InSelW<'_, Func13InSelCfgSpec> {
        RegGpioSig13InSelW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func13_in_sel(&mut self) -> RegGpioFunc13InSelW<'_, Func13InSelCfgSpec> {
        RegGpioFunc13InSelW::new(self, 2)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func13_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func13_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func13InSelCfgSpec;
impl crate::RegisterSpec for Func13InSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func13_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func13InSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func13_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func13InSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC13_IN_SEL_CFG to value 0x80"]
impl crate::Resettable for Func13InSelCfgSpec {
    const RESET_VALUE: u32 = 0x80;
}
