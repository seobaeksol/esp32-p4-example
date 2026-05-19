#[doc = "Register `EMACADDR5LOW` reader"]
pub type R = crate::R<Emacaddr5lowSpec>;
#[doc = "Register `EMACADDR5LOW` writer"]
pub type W = crate::W<Emacaddr5lowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "This field contains the lower 32 bits of the sixth 6-byte MAC address. The content of this field is undefined so the register needs to be configured after the initialization process.\n\nYou can [`read`](crate::Reg::read) this register and get [`emacaddr5low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacaddr5low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Emacaddr5lowSpec;
impl crate::RegisterSpec for Emacaddr5lowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr5low::R`](R) reader structure"]
impl crate::Readable for Emacaddr5lowSpec {}
#[doc = "`write(|w| ..)` method takes [`emacaddr5low::W`](W) writer structure"]
impl crate::Writable for Emacaddr5lowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACADDR5LOW to value 0"]
impl crate::Resettable for Emacaddr5lowSpec {}
