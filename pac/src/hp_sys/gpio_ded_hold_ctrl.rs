#[doc = "Register `GPIO_DED_HOLD_CTRL` reader"]
pub type R = crate::R<GpioDedHoldCtrlSpec>;
#[doc = "Register `GPIO_DED_HOLD_CTRL` writer"]
pub type W = crate::W<GpioDedHoldCtrlSpec>;
#[doc = "Field `REG_GPIO_DED_HOLD` reader - hold control for gpio63~56"]
pub type RegGpioDedHoldR = crate::FieldReader<u32>;
#[doc = "Field `REG_GPIO_DED_HOLD` writer - hold control for gpio63~56"]
pub type RegGpioDedHoldW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - hold control for gpio63~56"]
    #[inline(always)]
    pub fn reg_gpio_ded_hold(&self) -> RegGpioDedHoldR {
        RegGpioDedHoldR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - hold control for gpio63~56"]
    #[inline(always)]
    pub fn reg_gpio_ded_hold(&mut self) -> RegGpioDedHoldW<'_, GpioDedHoldCtrlSpec> {
        RegGpioDedHoldW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ded_hold_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ded_hold_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDedHoldCtrlSpec;
impl crate::RegisterSpec for GpioDedHoldCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ded_hold_ctrl::R`](R) reader structure"]
impl crate::Readable for GpioDedHoldCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_ded_hold_ctrl::W`](W) writer structure"]
impl crate::Writable for GpioDedHoldCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_DED_HOLD_CTRL to value 0"]
impl crate::Resettable for GpioDedHoldCtrlSpec {}
