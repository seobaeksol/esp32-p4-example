#[doc = "Register `DBIAS_CMD2` reader"]
pub type R = crate::R<DbiasCmd2Spec>;
#[doc = "Register `DBIAS_CMD2` writer"]
pub type W = crate::W<DbiasCmd2Spec>;
#[doc = "Field `DBIAS_CMD2` reader - needs field desc"]
pub type DbiasCmd2R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CMD2` writer - needs field desc"]
pub type DbiasCmd2W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd2(&self) -> DbiasCmd2R {
        DbiasCmd2R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd2(&mut self) -> DbiasCmd2W<'_, DbiasCmd2Spec> {
        DbiasCmd2W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_cmd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_cmd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasCmd2Spec;
impl crate::RegisterSpec for DbiasCmd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_cmd2::R`](R) reader structure"]
impl crate::Readable for DbiasCmd2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbias_cmd2::W`](W) writer structure"]
impl crate::Writable for DbiasCmd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CMD2 to value 0"]
impl crate::Resettable for DbiasCmd2Spec {}
