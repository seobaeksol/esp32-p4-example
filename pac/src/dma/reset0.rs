#[doc = "Register `RESET0` reader"]
pub type R = crate::R<Reset0Spec>;
#[doc = "Register `RESET0` writer"]
pub type W = crate::W<Reset0Spec>;
#[doc = "Field `DMAC_RST` reader - NA"]
pub type DmacRstR = crate::BitReader;
#[doc = "Field `DMAC_RST` writer - NA"]
pub type DmacRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dmac_rst(&self) -> DmacRstR {
        DmacRstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dmac_rst(&mut self) -> DmacRstW<'_, Reset0Spec> {
        DmacRstW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`reset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reset0Spec;
impl crate::RegisterSpec for Reset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset0::R`](R) reader structure"]
impl crate::Readable for Reset0Spec {}
#[doc = "`write(|w| ..)` method takes [`reset0::W`](W) writer structure"]
impl crate::Writable for Reset0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET0 to value 0"]
impl crate::Resettable for Reset0Spec {}
