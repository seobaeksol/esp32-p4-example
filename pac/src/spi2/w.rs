#[doc = "Register `W%s` reader"]
pub type R = crate::R<WSpec>;
#[doc = "Register `W%s` writer"]
pub type W = crate::W<WSpec>;
#[doc = "Field `BUF` reader - data buffer"]
pub type BufR = crate::FieldReader<u32>;
#[doc = "Field `BUF` writer - data buffer"]
pub type BufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf(&self) -> BufR {
        BufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf(&mut self) -> BufW<'_, WSpec> {
        BufW::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer%s\n\nYou can [`read`](crate::Reg::read) this register and get [`w::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WSpec;
impl crate::RegisterSpec for WSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w::R`](R) reader structure"]
impl crate::Readable for WSpec {}
#[doc = "`write(|w| ..)` method takes [`w::W`](W) writer structure"]
impl crate::Writable for WSpec {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets W%s to value 0"]
impl crate::Resettable for WSpec {}
