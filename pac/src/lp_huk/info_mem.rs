#[doc = "Register `INFO_MEM[%s]` reader"]
pub type R = crate::R<InfoMemSpec>;
#[doc = "Register `INFO_MEM[%s]` writer"]
pub type W = crate::W<InfoMemSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores HUK info.\n\nYou can [`read`](crate::Reg::read) this register and get [`info_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoMemSpec;
impl crate::RegisterSpec for InfoMemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info_mem::R`](R) reader structure"]
impl crate::Readable for InfoMemSpec {}
#[doc = "`write(|w| ..)` method takes [`info_mem::W`](W) writer structure"]
impl crate::Writable for InfoMemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INFO_MEM[%s] to value 0"]
impl crate::Resettable for InfoMemSpec {}
