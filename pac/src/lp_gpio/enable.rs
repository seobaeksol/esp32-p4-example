#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `REG_GPIO_ENABLE_DATA` reader - Reserved"]
pub type RegGpioEnableDataR = crate::FieldReader<u16>;
#[doc = "Field `REG_GPIO_ENABLE_DATA` writer - Reserved"]
pub type RegGpioEnableDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_enable_data(&self) -> RegGpioEnableDataR {
        RegGpioEnableDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_enable_data(&mut self) -> RegGpioEnableDataW<'_, EnableSpec> {
        RegGpioEnableDataW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {}
