#[doc = "Register `MULT_DATE` reader"]
pub type R = crate::R<MultDateSpec>;
#[doc = "Register `MULT_DATE` writer"]
pub type W = crate::W<MultDateSpec>;
#[doc = "Field `MULT_DATE` reader - ECC mult version control register"]
pub type MultDateR = crate::FieldReader<u32>;
#[doc = "Field `MULT_DATE` writer - ECC mult version control register"]
pub type MultDateW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - ECC mult version control register"]
    #[inline(always)]
    pub fn mult_date(&self) -> MultDateR {
        MultDateR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - ECC mult version control register"]
    #[inline(always)]
    pub fn mult_date(&mut self) -> MultDateW<'_, MultDateSpec> {
        MultDateW::new(self, 0)
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MultDateSpec;
impl crate::RegisterSpec for MultDateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_date::R`](R) reader structure"]
impl crate::Readable for MultDateSpec {}
#[doc = "`write(|w| ..)` method takes [`mult_date::W`](W) writer structure"]
impl crate::Writable for MultDateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_DATE to value 0x0240_8120"]
impl crate::Resettable for MultDateSpec {
    const RESET_VALUE: u32 = 0x0240_8120;
}
