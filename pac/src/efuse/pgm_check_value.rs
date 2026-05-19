#[doc = "Register `PGM_CHECK_VALUE%s` reader"]
pub type R = crate::R<PgmCheckValueSpec>;
#[doc = "Register `PGM_CHECK_VALUE%s` writer"]
pub type W = crate::W<PgmCheckValueSpec>;
#[doc = "Field `PGM_RS_DATA` reader - Configures the %sth RS code to be programmed."]
pub type PgmRsDataR = crate::FieldReader<u32>;
#[doc = "Field `PGM_RS_DATA` writer - Configures the %sth RS code to be programmed."]
pub type PgmRsDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the %sth RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data(&self) -> PgmRsDataR {
        PgmRsDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the %sth RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_rs_data(&mut self) -> PgmRsDataW<'_, PgmCheckValueSpec> {
        PgmRsDataW::new(self, 0)
    }
}
#[doc = "Represents pgm_check_value%s\n\nYou can [`read`](crate::Reg::read) this register and get [`pgm_check_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pgm_check_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgmCheckValueSpec;
impl crate::RegisterSpec for PgmCheckValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pgm_check_value::R`](R) reader structure"]
impl crate::Readable for PgmCheckValueSpec {}
#[doc = "`write(|w| ..)` method takes [`pgm_check_value::W`](W) writer structure"]
impl crate::Writable for PgmCheckValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PGM_CHECK_VALUE%s to value 0"]
impl crate::Resettable for PgmCheckValueSpec {}
