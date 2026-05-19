#[doc = "Register `DBIAS_CMD4` reader"]
pub type R = crate::R<DbiasCmd4Spec>;
#[doc = "Register `DBIAS_CMD4` writer"]
pub type W = crate::W<DbiasCmd4Spec>;
#[doc = "Field `DBIAS_CMD4` reader - needs field desc"]
pub type DbiasCmd4R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CMD4` writer - needs field desc"]
pub type DbiasCmd4W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd4(&self) -> DbiasCmd4R {
        DbiasCmd4R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd4(&mut self) -> DbiasCmd4W<'_, DbiasCmd4Spec> {
        DbiasCmd4W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_cmd4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_cmd4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasCmd4Spec;
impl crate::RegisterSpec for DbiasCmd4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_cmd4::R`](R) reader structure"]
impl crate::Readable for DbiasCmd4Spec {}
#[doc = "`write(|w| ..)` method takes [`dbias_cmd4::W`](W) writer structure"]
impl crate::Writable for DbiasCmd4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CMD4 to value 0"]
impl crate::Resettable for DbiasCmd4Spec {}
