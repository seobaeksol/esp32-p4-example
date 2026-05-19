#[doc = "Register `UHS` reader"]
pub type R = crate::R<UhsSpec>;
#[doc = "Register `UHS` writer"]
pub type W = crate::W<UhsSpec>;
#[doc = "Field `DDR` reader - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DdrR = crate::FieldReader;
#[doc = "Field `DDR` writer - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DdrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    pub fn ddr(&self) -> DdrR {
        DdrR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    pub fn ddr(&mut self) -> DdrW<'_, UhsSpec> {
        DdrW::new(self, 16)
    }
}
#[doc = "UHS-1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsSpec;
impl crate::RegisterSpec for UhsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs::R`](R) reader structure"]
impl crate::Readable for UhsSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs::W`](W) writer structure"]
impl crate::Writable for UhsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS to value 0"]
impl crate::Resettable for UhsSpec {}
