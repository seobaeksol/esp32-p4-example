#[doc = "Register `DBIAS_CMD0` reader"]
pub type R = crate::R<DbiasCmd0Spec>;
#[doc = "Register `DBIAS_CMD0` writer"]
pub type W = crate::W<DbiasCmd0Spec>;
#[doc = "Field `DBIAS_CMD0` reader - needs field desc"]
pub type DbiasCmd0R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CMD0` writer - needs field desc"]
pub type DbiasCmd0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd0(&self) -> DbiasCmd0R {
        DbiasCmd0R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd0(&mut self) -> DbiasCmd0W<'_, DbiasCmd0Spec> {
        DbiasCmd0W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_cmd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_cmd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasCmd0Spec;
impl crate::RegisterSpec for DbiasCmd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_cmd0::R`](R) reader structure"]
impl crate::Readable for DbiasCmd0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbias_cmd0::W`](W) writer structure"]
impl crate::Writable for DbiasCmd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CMD0 to value 0"]
impl crate::Resettable for DbiasCmd0Spec {}
