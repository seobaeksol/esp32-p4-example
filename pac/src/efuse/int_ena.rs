#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `READ_DONE` reader - The enable signal for read_done interrupt."]
pub type ReadDoneR = crate::BitReader;
#[doc = "Field `READ_DONE` writer - The enable signal for read_done interrupt."]
pub type ReadDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGM_DONE` reader - The enable signal for pgm_done interrupt."]
pub type PgmDoneR = crate::BitReader;
#[doc = "Field `PGM_DONE` writer - The enable signal for pgm_done interrupt."]
pub type PgmDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The enable signal for read_done interrupt."]
    #[inline(always)]
    pub fn read_done(&self) -> ReadDoneR {
        ReadDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable signal for pgm_done interrupt."]
    #[inline(always)]
    pub fn pgm_done(&self) -> PgmDoneR {
        PgmDoneR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable signal for read_done interrupt."]
    #[inline(always)]
    pub fn read_done(&mut self) -> ReadDoneW<'_, IntEnaSpec> {
        ReadDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - The enable signal for pgm_done interrupt."]
    #[inline(always)]
    pub fn pgm_done(&mut self) -> PgmDoneW<'_, IntEnaSpec> {
        PgmDoneW::new(self, 1)
    }
}
#[doc = "eFuse interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
