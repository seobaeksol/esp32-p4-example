#[doc = "Register `PY_MEM%s` reader"]
pub type R = crate::R<PyMemSpec>;
#[doc = "Register `PY_MEM%s` writer"]
pub type W = crate::W<PyMemSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores Py.\n\nYou can [`read`](crate::Reg::read) this register and get [`py_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`py_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PyMemSpec;
impl crate::RegisterSpec for PyMemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`py_mem::R`](R) reader structure"]
impl crate::Readable for PyMemSpec {}
#[doc = "`write(|w| ..)` method takes [`py_mem::W`](W) writer structure"]
impl crate::Writable for PyMemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PY_MEM%s to value 0"]
impl crate::Resettable for PyMemSpec {}
