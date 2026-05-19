#[doc = "Register `DBIAS_CMD1` reader"]
pub type R = crate::R<DbiasCmd1Spec>;
#[doc = "Register `DBIAS_CMD1` writer"]
pub type W = crate::W<DbiasCmd1Spec>;
#[doc = "Field `DBIAS_CMD1` reader - needs field desc"]
pub type DbiasCmd1R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CMD1` writer - needs field desc"]
pub type DbiasCmd1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd1(&self) -> DbiasCmd1R {
        DbiasCmd1R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd1(&mut self) -> DbiasCmd1W<'_, DbiasCmd1Spec> {
        DbiasCmd1W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_cmd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_cmd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasCmd1Spec;
impl crate::RegisterSpec for DbiasCmd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_cmd1::R`](R) reader structure"]
impl crate::Readable for DbiasCmd1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbias_cmd1::W`](W) writer structure"]
impl crate::Writable for DbiasCmd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CMD1 to value 0"]
impl crate::Resettable for DbiasCmd1Spec {}
