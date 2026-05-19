#[doc = "Register `MISO_DLEN` reader"]
pub type R = crate::R<MisoDlenSpec>;
#[doc = "Register `MISO_DLEN` writer"]
pub type W = crate::W<MisoDlenSpec>;
#[doc = "Field `USR_MISO_DBITLEN` reader - The length in bits of read-data. The register value shall be (bit_num-1)."]
pub type UsrMisoDbitlenR = crate::FieldReader<u16>;
#[doc = "Field `USR_MISO_DBITLEN` writer - The length in bits of read-data. The register value shall be (bit_num-1)."]
pub type UsrMisoDbitlenW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - The length in bits of read-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_miso_dbitlen(&self) -> UsrMisoDbitlenR {
        UsrMisoDbitlenR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The length in bits of read-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn usr_miso_dbitlen(&mut self) -> UsrMisoDbitlenW<'_, MisoDlenSpec> {
        UsrMisoDbitlenW::new(self, 0)
    }
}
#[doc = "SPI1 receive data bit length control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`miso_dlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miso_dlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisoDlenSpec;
impl crate::RegisterSpec for MisoDlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miso_dlen::R`](R) reader structure"]
impl crate::Readable for MisoDlenSpec {}
#[doc = "`write(|w| ..)` method takes [`miso_dlen::W`](W) writer structure"]
impl crate::Writable for MisoDlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISO_DLEN to value 0"]
impl crate::Resettable for MisoDlenSpec {}
