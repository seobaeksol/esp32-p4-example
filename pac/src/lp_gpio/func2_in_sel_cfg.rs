#[doc = "Register `FUNC2_IN_SEL_CFG` reader"]
pub type R = crate::R<Func2InSelCfgSpec>;
#[doc = "Register `FUNC2_IN_SEL_CFG` writer"]
pub type W = crate::W<Func2InSelCfgSpec>;
#[doc = "Field `REG_GPIO_FUNC2_IN_INV_SEL` reader - Reserved"]
pub type RegGpioFunc2InInvSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_FUNC2_IN_INV_SEL` writer - Reserved"]
pub type RegGpioFunc2InInvSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_SIG2_IN_SEL` reader - Reserved"]
pub type RegGpioSig2InSelR = crate::BitReader;
#[doc = "Field `REG_GPIO_SIG2_IN_SEL` writer - Reserved"]
pub type RegGpioSig2InSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_GPIO_FUNC2_IN_SEL` reader - Reserved"]
pub type RegGpioFunc2InSelR = crate::FieldReader;
#[doc = "Field `REG_GPIO_FUNC2_IN_SEL` writer - Reserved"]
pub type RegGpioFunc2InSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func2_in_inv_sel(&self) -> RegGpioFunc2InInvSelR {
        RegGpioFunc2InInvSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig2_in_sel(&self) -> RegGpioSig2InSelR {
        RegGpioSig2InSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func2_in_sel(&self) -> RegGpioFunc2InSelR {
        RegGpioFunc2InSelR::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func2_in_inv_sel(&mut self) -> RegGpioFunc2InInvSelW<'_, Func2InSelCfgSpec> {
        RegGpioFunc2InInvSelW::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_sig2_in_sel(&mut self) -> RegGpioSig2InSelW<'_, Func2InSelCfgSpec> {
        RegGpioSig2InSelW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_func2_in_sel(&mut self) -> RegGpioFunc2InSelW<'_, Func2InSelCfgSpec> {
        RegGpioFunc2InSelW::new(self, 2)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`func2_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func2_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Func2InSelCfgSpec;
impl crate::RegisterSpec for Func2InSelCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func2_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for Func2InSelCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`func2_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for Func2InSelCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC2_IN_SEL_CFG to value 0x80"]
impl crate::Resettable for Func2InSelCfgSpec {
    const RESET_VALUE: u32 = 0x80;
}
