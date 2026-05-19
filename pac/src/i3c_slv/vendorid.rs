#[doc = "Register `VENDORID` reader"]
pub type R = crate::R<VendoridSpec>;
#[doc = "Register `VENDORID` writer"]
pub type W = crate::W<VendoridSpec>;
#[doc = "Field `VID` reader - NA"]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `VID` writer - NA"]
pub type VidW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - NA"]
    #[inline(always)]
    pub fn vid(&mut self) -> VidW<'_, VendoridSpec> {
        VidW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vendorid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vendorid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VendoridSpec;
impl crate::RegisterSpec for VendoridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vendorid::R`](R) reader structure"]
impl crate::Readable for VendoridSpec {}
#[doc = "`write(|w| ..)` method takes [`vendorid::W`](W) writer structure"]
impl crate::Writable for VendoridSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VENDORID to value 0x5550"]
impl crate::Resettable for VendoridSpec {
    const RESET_VALUE: u32 = 0x5550;
}
