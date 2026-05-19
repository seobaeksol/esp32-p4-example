#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<IntRawSpec>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<IntRawSpec>;
#[doc = "Field `OVERFLOW` reader - need_des"]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - need_des"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_WAKEUP` reader - need_des"]
pub type SocWakeupR = crate::BitReader;
#[doc = "Field `SOC_WAKEUP` writer - need_des"]
pub type SocWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SocWakeupR {
        SocWakeupR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, IntRawSpec> {
        OverflowW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup(&mut self) -> SocWakeupW<'_, IntRawSpec> {
        SocWakeupW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntRawSpec;
impl crate::RegisterSpec for IntRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for IntRawSpec {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for IntRawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for IntRawSpec {}
