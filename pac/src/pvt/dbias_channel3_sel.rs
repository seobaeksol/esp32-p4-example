#[doc = "Register `DBIAS_CHANNEL3_SEL` reader"]
pub type R = crate::R<DbiasChannel3SelSpec>;
#[doc = "Register `DBIAS_CHANNEL3_SEL` writer"]
pub type W = crate::W<DbiasChannel3SelSpec>;
#[doc = "Field `DBIAS_CHANNEL3_CFG` reader - needs field desc"]
pub type DbiasChannel3CfgR = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CHANNEL3_CFG` writer - needs field desc"]
pub type DbiasChannel3CfgW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel3_cfg(&self) -> DbiasChannel3CfgR {
        DbiasChannel3CfgR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel3_cfg(&mut self) -> DbiasChannel3CfgW<'_, DbiasChannel3SelSpec> {
        DbiasChannel3CfgW::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_channel3_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_channel3_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasChannel3SelSpec;
impl crate::RegisterSpec for DbiasChannel3SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_channel3_sel::R`](R) reader structure"]
impl crate::Readable for DbiasChannel3SelSpec {}
#[doc = "`write(|w| ..)` method takes [`dbias_channel3_sel::W`](W) writer structure"]
impl crate::Writable for DbiasChannel3SelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CHANNEL3_SEL to value 0"]
impl crate::Resettable for DbiasChannel3SelSpec {}
