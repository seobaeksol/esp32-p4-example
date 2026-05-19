#[doc = "Register `REGISTERRND_ECO_LOW` reader"]
pub type R = crate::R<RegisterrndEcoLowSpec>;
#[doc = "Register `REGISTERRND_ECO_LOW` writer"]
pub type W = crate::W<RegisterrndEcoLowSpec>;
#[doc = "Field `REGISTERRND_ECO_LOW` reader - ECO low register"]
pub type RegisterrndEcoLowR = crate::FieldReader<u32>;
#[doc = "Field `REGISTERRND_ECO_LOW` writer - ECO low register"]
pub type RegisterrndEcoLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECO low register"]
    #[inline(always)]
    pub fn registerrnd_eco_low(&self) -> RegisterrndEcoLowR {
        RegisterrndEcoLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECO low register"]
    #[inline(always)]
    pub fn registerrnd_eco_low(&mut self) -> RegisterrndEcoLowW<'_, RegisterrndEcoLowSpec> {
        RegisterrndEcoLowW::new(self, 0)
    }
}
#[doc = "MSPI ECO low register\n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegisterrndEcoLowSpec;
impl crate::RegisterSpec for RegisterrndEcoLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`registerrnd_eco_low::R`](R) reader structure"]
impl crate::Readable for RegisterrndEcoLowSpec {}
#[doc = "`write(|w| ..)` method takes [`registerrnd_eco_low::W`](W) writer structure"]
impl crate::Writable for RegisterrndEcoLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTERRND_ECO_LOW to value 0x037c"]
impl crate::Resettable for RegisterrndEcoLowSpec {
    const RESET_VALUE: u32 = 0x037c;
}
