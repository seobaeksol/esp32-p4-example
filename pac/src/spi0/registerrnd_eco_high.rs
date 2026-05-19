#[doc = "Register `REGISTERRND_ECO_HIGH` reader"]
pub type R = crate::R<RegisterrndEcoHighSpec>;
#[doc = "Register `REGISTERRND_ECO_HIGH` writer"]
pub type W = crate::W<RegisterrndEcoHighSpec>;
#[doc = "Field `REGISTERRND_ECO_HIGH` reader - ECO high register"]
pub type RegisterrndEcoHighR = crate::FieldReader<u32>;
#[doc = "Field `REGISTERRND_ECO_HIGH` writer - ECO high register"]
pub type RegisterrndEcoHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECO high register"]
    #[inline(always)]
    pub fn registerrnd_eco_high(&self) -> RegisterrndEcoHighR {
        RegisterrndEcoHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECO high register"]
    #[inline(always)]
    pub fn registerrnd_eco_high(&mut self) -> RegisterrndEcoHighW<'_, RegisterrndEcoHighSpec> {
        RegisterrndEcoHighW::new(self, 0)
    }
}
#[doc = "MSPI ECO high register\n\nYou can [`read`](crate::Reg::read) this register and get [`registerrnd_eco_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`registerrnd_eco_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegisterrndEcoHighSpec;
impl crate::RegisterSpec for RegisterrndEcoHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`registerrnd_eco_high::R`](R) reader structure"]
impl crate::Readable for RegisterrndEcoHighSpec {}
#[doc = "`write(|w| ..)` method takes [`registerrnd_eco_high::W`](W) writer structure"]
impl crate::Writable for RegisterrndEcoHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTERRND_ECO_HIGH to value 0x037c"]
impl crate::Resettable for RegisterrndEcoHighSpec {
    const RESET_VALUE: u32 = 0x037c;
}
