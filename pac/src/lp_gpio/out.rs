#[doc = "Register `OUT` reader"]
pub type R = crate::R<OutSpec>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OutSpec>;
#[doc = "Field `REG_GPIO_OUT_DATA` reader - Reserved"]
pub type RegGpioOutDataR = crate::FieldReader<u16>;
#[doc = "Field `REG_GPIO_OUT_DATA` writer - Reserved"]
pub type RegGpioOutDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_out_data(&self) -> RegGpioOutDataR {
        RegGpioOutDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_out_data(&mut self) -> RegGpioOutDataW<'_, OutSpec> {
        RegGpioOutDataW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSpec;
impl crate::RegisterSpec for OutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OutSpec {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OutSpec {}
