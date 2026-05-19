#[doc = "Register `USRID` reader"]
pub type R = crate::R<UsridSpec>;
#[doc = "Register `USRID` writer"]
pub type W = crate::W<UsridSpec>;
#[doc = "Field `USRID` reader - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub type UsridR = crate::FieldReader<u32>;
#[doc = "Field `USRID` writer - User identification register, value set by user. Can also be used as a scratchpad register by user."]
pub type UsridW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    pub fn usrid(&self) -> UsridR {
        UsridR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User identification register, value set by user. Can also be used as a scratchpad register by user."]
    #[inline(always)]
    pub fn usrid(&mut self) -> UsridW<'_, UsridSpec> {
        UsridW::new(self, 0)
    }
}
#[doc = "User ID (scratchpad) register\n\nYou can [`read`](crate::Reg::read) this register and get [`usrid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usrid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsridSpec;
impl crate::RegisterSpec for UsridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usrid::R`](R) reader structure"]
impl crate::Readable for UsridSpec {}
#[doc = "`write(|w| ..)` method takes [`usrid::W`](W) writer structure"]
impl crate::Writable for UsridSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USRID to value 0"]
impl crate::Resettable for UsridSpec {}
