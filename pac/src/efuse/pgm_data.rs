#[doc = "Register `PGM_DATA%s` reader"]
pub type R = crate::R<PgmDataSpec>;
#[doc = "Register `PGM_DATA%s` writer"]
pub type W = crate::W<PgmDataSpec>;
#[doc = "Field `PGM_DATA` reader - Configures the %sth 32-bit data to be programmed."]
pub type PgmDataR = crate::FieldReader<u32>;
#[doc = "Field `PGM_DATA` writer - Configures the %sth 32-bit data to be programmed."]
pub type PgmDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the %sth 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data(&self) -> PgmDataR {
        PgmDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the %sth 32-bit data to be programmed."]
    #[inline(always)]
    pub fn pgm_data(&mut self) -> PgmDataW<'_, PgmDataSpec> {
        PgmDataW::new(self, 0)
    }
}
#[doc = "Represents pgm_data%s\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgmDataSpec;
impl crate::RegisterSpec for PgmDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_data::R`](R) reader structure"]
impl crate::Readable for PgmDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pgm_data::W`](W) writer structure"]
impl crate::Writable for PgmDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGM_DATA%s to value 0"]
impl crate::Resettable for PgmDataSpec {}
