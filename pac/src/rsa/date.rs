#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `DATE` reader - Version control register."]
pub type DateR = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - Version control register."]
pub type DateW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Version control register."]
    #[inline(always)]
    pub fn date(&self) -> DateR {
        DateR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Version control register."]
    #[inline(always)]
    pub fn date(&mut self) -> DateW<'_, DateSpec> {
        DateW::new(self, 0)
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0x2020_0618"]
impl crate::Resettable for DateSpec {
    const RESET_VALUE: u32 = 0x2020_0618;
}
