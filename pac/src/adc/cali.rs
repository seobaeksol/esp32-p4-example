#[doc = "Register `CALI` reader"]
pub type R = crate::R<CaliSpec>;
#[doc = "Register `CALI` writer"]
pub type W = crate::W<CaliSpec>;
#[doc = "Field `CFG` reader - need_des"]
pub type CfgR = crate::FieldReader<u32>;
#[doc = "Field `CFG` writer - need_des"]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CfgW<'_, CaliSpec> {
        CfgW::new(self, 0)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaliSpec;
impl crate::RegisterSpec for CaliSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali::R`](R) reader structure"]
impl crate::Readable for CaliSpec {}
#[doc = "`write(|w| ..)` method takes [`cali::W`](W) writer structure"]
impl crate::Writable for CaliSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALI to value 0x8000"]
impl crate::Resettable for CaliSpec {
    const RESET_VALUE: u32 = 0x8000;
}
