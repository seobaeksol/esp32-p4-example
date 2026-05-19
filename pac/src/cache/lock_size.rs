#[doc = "Register `LOCK_SIZE` reader"]
pub type R = crate::R<LockSizeSpec>;
#[doc = "Register `LOCK_SIZE` writer"]
pub type W = crate::W<LockSizeSpec>;
#[doc = "Field `LOCK_SIZE` reader - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
pub type LockSizeR = crate::FieldReader<u16>;
#[doc = "Field `LOCK_SIZE` writer - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
pub type LockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
    #[inline(always)]
    pub fn lock_size(&self) -> LockSizeR {
        LockSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the lock/unlock operation, which should be used together with CACHE_LOCK_ADDR_REG"]
    #[inline(always)]
    pub fn lock_size(&mut self) -> LockSizeW<'_, LockSizeSpec> {
        LockSizeW::new(self, 0)
    }
}
#[doc = "Lock (manual lock) size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSizeSpec;
impl crate::RegisterSpec for LockSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock_size::R`](R) reader structure"]
impl crate::Readable for LockSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`lock_size::W`](W) writer structure"]
impl crate::Writable for LockSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK_SIZE to value 0"]
impl crate::Resettable for LockSizeSpec {}
