#[doc = "Register `LOADHI` reader"]
pub type R = crate::R<LoadhiSpec>;
#[doc = "Register `LOADHI` writer"]
pub type W = crate::W<LoadhiSpec>;
#[doc = "Field `LOAD_HI` reader - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
pub type LoadHiR = crate::FieldReader<u32>;
#[doc = "Field `LOAD_HI` writer - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
pub type LoadHiW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
    #[inline(always)]
    pub fn load_hi(&self) -> LoadHiR {
        LoadHiR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
    #[inline(always)]
    pub fn load_hi(&mut self) -> LoadHiW<'_, LoadhiSpec> {
        LoadHiW::new(self, 0)
    }
}
#[doc = "Timer 0 reload value, high 22 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`loadhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loadhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadhiSpec;
impl crate::RegisterSpec for LoadhiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loadhi::R`](R) reader structure"]
impl crate::Readable for LoadhiSpec {}
#[doc = "`write(|w| ..)` method takes [`loadhi::W`](W) writer structure"]
impl crate::Writable for LoadhiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOADHI to value 0"]
impl crate::Resettable for LoadhiSpec {}
