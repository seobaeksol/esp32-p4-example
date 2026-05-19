#[doc = "Register `VER_DATE` reader"]
pub type R = crate::R<VerDateSpec>;
#[doc = "Register `VER_DATE` writer"]
pub type W = crate::W<VerDateSpec>;
#[doc = "Field `REG_VER_DATE` reader - Reserved"]
pub type RegVerDateR = crate::FieldReader<u32>;
#[doc = "Field `REG_VER_DATE` writer - Reserved"]
pub type RegVerDateW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn reg_ver_date(&self) -> RegVerDateR {
        RegVerDateR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Reserved"]
    #[inline(always)]
    pub fn reg_ver_date(&mut self) -> RegVerDateW<'_, VerDateSpec> {
        RegVerDateW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ver_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VerDateSpec;
impl crate::RegisterSpec for VerDateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver_date::R`](R) reader structure"]
impl crate::Readable for VerDateSpec {}
#[doc = "`write(|w| ..)` method takes [`ver_date::W`](W) writer structure"]
impl crate::Writable for VerDateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VER_DATE to value 0x0023_0323"]
impl crate::Resettable for VerDateSpec {
    const RESET_VALUE: u32 = 0x0023_0323;
}
