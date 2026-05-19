#[doc = "Register `DBIAS_CHANNEL1_SEL` reader"]
pub type R = crate::R<DbiasChannel1SelSpec>;
#[doc = "Register `DBIAS_CHANNEL1_SEL` writer"]
pub type W = crate::W<DbiasChannel1SelSpec>;
#[doc = "Field `DBIAS_CHANNEL1_CFG` reader - needs field desc"]
pub type DbiasChannel1CfgR = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CHANNEL1_CFG` writer - needs field desc"]
pub type DbiasChannel1CfgW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel1_cfg(&self) -> DbiasChannel1CfgR {
        DbiasChannel1CfgR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel1_cfg(&mut self) -> DbiasChannel1CfgW<'_, DbiasChannel1SelSpec> {
        DbiasChannel1CfgW::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_channel1_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_channel1_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiasChannel1SelSpec;
impl crate::RegisterSpec for DbiasChannel1SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_channel1_sel::R`](R) reader structure"]
impl crate::Readable for DbiasChannel1SelSpec {}
#[doc = "`write(|w| ..)` method takes [`dbias_channel1_sel::W`](W) writer structure"]
impl crate::Writable for DbiasChannel1SelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CHANNEL1_SEL to value 0"]
impl crate::Resettable for DbiasChannel1SelSpec {}
