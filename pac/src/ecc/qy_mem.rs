#[doc = "Register `QY_MEM%s` reader"]
pub type R = crate::R<QyMemSpec>;
#[doc = "Register `QY_MEM%s` writer"]
pub type W = crate::W<QyMemSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores Qy.\n\nYou can [`read`](crate::Reg::read) this register and get [`qy_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qy_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QyMemSpec;
impl crate::RegisterSpec for QyMemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qy_mem::R`](R) reader structure"]
impl crate::Readable for QyMemSpec {}
#[doc = "`write(|w| ..)` method takes [`qy_mem::W`](W) writer structure"]
impl crate::Writable for QyMemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QY_MEM%s to value 0"]
impl crate::Resettable for QyMemSpec {}
