#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `CLKDIV` reader - The integral part of the frequency divider factor."]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - The integral part of the frequency divider factor."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CLKDIV_FRAG` reader - The decimal part of the frequency divider factor."]
pub type ClkdivFragR = crate::FieldReader;
#[doc = "Field `CLKDIV_FRAG` writer - The decimal part of the frequency divider factor."]
pub type ClkdivFragW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - The integral part of the frequency divider factor."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:23 - The decimal part of the frequency divider factor."]
    #[inline(always)]
    pub fn clkdiv_frag(&self) -> ClkdivFragR {
        ClkdivFragR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - The integral part of the frequency divider factor."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, ClkdivSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bits 20:23 - The decimal part of the frequency divider factor."]
    #[inline(always)]
    pub fn clkdiv_frag(&mut self) -> ClkdivFragW<'_, ClkdivSpec> {
        ClkdivFragW::new(self, 20)
    }
}
#[doc = "Clock divider configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV to value 0x02b6"]
impl crate::Resettable for ClkdivSpec {
    const RESET_VALUE: u32 = 0x02b6;
}
