#[doc = "Register `EMACMIIDATA` reader"]
pub type R = crate::R<EmacmiidataSpec>;
#[doc = "Register `EMACMIIDATA` writer"]
pub type W = crate::W<EmacmiidataSpec>;
#[doc = "Field `MII_DATA` reader - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MiiDataR = crate::FieldReader<u16>;
#[doc = "Field `MII_DATA` writer - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
pub type MiiDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    pub fn mii_data(&self) -> MiiDataR {
        MiiDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field contains the 16-bit data value read from the PHY after a Management Read operation or the 16-bit data value to be written to the PHY before a Management Write operation."]
    #[inline(always)]
    pub fn mii_data(&mut self) -> MiiDataW<'_, EmacmiidataSpec> {
        MiiDataW::new(self, 0)
    }
}
#[doc = "PHY data read write\n\nYou can [`read`](crate::Reg::read) this register and get [`emacmiidata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emacmiidata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmacmiidataSpec;
impl crate::RegisterSpec for EmacmiidataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacmiidata::R`](R) reader structure"]
impl crate::Readable for EmacmiidataSpec {}
#[doc = "`write(|w| ..)` method takes [`emacmiidata::W`](W) writer structure"]
impl crate::Writable for EmacmiidataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACMIIDATA to value 0"]
impl crate::Resettable for EmacmiidataSpec {}
