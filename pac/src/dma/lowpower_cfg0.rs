#[doc = "Register `LOWPOWER_CFG0` reader"]
pub type R = crate::R<LowpowerCfg0Spec>;
#[doc = "Register `LOWPOWER_CFG0` writer"]
pub type W = crate::W<LowpowerCfg0Spec>;
#[doc = "Field `GBL_CSLP_EN` reader - NA"]
pub type GblCslpEnR = crate::BitReader;
#[doc = "Field `GBL_CSLP_EN` writer - NA"]
pub type GblCslpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL_CSLP_EN` reader - NA"]
pub type ChnlCslpEnR = crate::BitReader;
#[doc = "Field `CHNL_CSLP_EN` writer - NA"]
pub type ChnlCslpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIU_CSLP_EN` reader - NA"]
pub type SbiuCslpEnR = crate::BitReader;
#[doc = "Field `SBIU_CSLP_EN` writer - NA"]
pub type SbiuCslpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MXIF_CSLP_EN` reader - NA"]
pub type MxifCslpEnR = crate::BitReader;
#[doc = "Field `MXIF_CSLP_EN` writer - NA"]
pub type MxifCslpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn gbl_cslp_en(&self) -> GblCslpEnR {
        GblCslpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn chnl_cslp_en(&self) -> ChnlCslpEnR {
        ChnlCslpEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn sbiu_cslp_en(&self) -> SbiuCslpEnR {
        SbiuCslpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mxif_cslp_en(&self) -> MxifCslpEnR {
        MxifCslpEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn gbl_cslp_en(&mut self) -> GblCslpEnW<'_, LowpowerCfg0Spec> {
        GblCslpEnW::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn chnl_cslp_en(&mut self) -> ChnlCslpEnW<'_, LowpowerCfg0Spec> {
        ChnlCslpEnW::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn sbiu_cslp_en(&mut self) -> SbiuCslpEnW<'_, LowpowerCfg0Spec> {
        SbiuCslpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn mxif_cslp_en(&mut self) -> MxifCslpEnW<'_, LowpowerCfg0Spec> {
        MxifCslpEnW::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowpowerCfg0Spec;
impl crate::RegisterSpec for LowpowerCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowpower_cfg0::R`](R) reader structure"]
impl crate::Readable for LowpowerCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`lowpower_cfg0::W`](W) writer structure"]
impl crate::Writable for LowpowerCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOWPOWER_CFG0 to value 0x0f"]
impl crate::Resettable for LowpowerCfg0Spec {
    const RESET_VALUE: u32 = 0x0f;
}
