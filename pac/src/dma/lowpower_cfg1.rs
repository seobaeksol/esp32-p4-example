#[doc = "Register `LOWPOWER_CFG1` reader"]
pub type R = crate::R<LowpowerCfg1Spec>;
#[doc = "Register `LOWPOWER_CFG1` writer"]
pub type W = crate::W<LowpowerCfg1Spec>;
#[doc = "Field `GLCH_LPDLY` reader - NA"]
pub type GlchLpdlyR = crate::FieldReader;
#[doc = "Field `GLCH_LPDLY` writer - NA"]
pub type GlchLpdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SBIU_LPDLY` reader - NA"]
pub type SbiuLpdlyR = crate::FieldReader;
#[doc = "Field `SBIU_LPDLY` writer - NA"]
pub type SbiuLpdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MXIF_LPDLY` reader - NA"]
pub type MxifLpdlyR = crate::FieldReader;
#[doc = "Field `MXIF_LPDLY` writer - NA"]
pub type MxifLpdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn glch_lpdly(&self) -> GlchLpdlyR {
        GlchLpdlyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn sbiu_lpdly(&self) -> SbiuLpdlyR {
        SbiuLpdlyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn mxif_lpdly(&self) -> MxifLpdlyR {
        MxifLpdlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn glch_lpdly(&mut self) -> GlchLpdlyW<'_, LowpowerCfg1Spec> {
        GlchLpdlyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn sbiu_lpdly(&mut self) -> SbiuLpdlyW<'_, LowpowerCfg1Spec> {
        SbiuLpdlyW::new(self, 8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn mxif_lpdly(&mut self) -> MxifLpdlyW<'_, LowpowerCfg1Spec> {
        MxifLpdlyW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowpowerCfg1Spec;
impl crate::RegisterSpec for LowpowerCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowpower_cfg1::R`](R) reader structure"]
impl crate::Readable for LowpowerCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`lowpower_cfg1::W`](W) writer structure"]
impl crate::Writable for LowpowerCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOWPOWER_CFG1 to value 0x0040_4040"]
impl crate::Resettable for LowpowerCfg1Spec {
    const RESET_VALUE: u32 = 0x0040_4040;
}
