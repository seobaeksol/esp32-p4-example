#[doc = "Register `VID_SHADOW_CTRL` reader"]
pub type R = crate::R<VidShadowCtrlSpec>;
#[doc = "Register `VID_SHADOW_CTRL` writer"]
pub type W = crate::W<VidShadowCtrlSpec>;
#[doc = "Field `VID_SHADOW_EN` reader - NA"]
pub type VidShadowEnR = crate::BitReader;
#[doc = "Field `VID_SHADOW_EN` writer - NA"]
pub type VidShadowEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_SHADOW_REQ` reader - NA"]
pub type VidShadowReqR = crate::BitReader;
#[doc = "Field `VID_SHADOW_REQ` writer - NA"]
pub type VidShadowReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_SHADOW_PIN_REQ` reader - NA"]
pub type VidShadowPinReqR = crate::BitReader;
#[doc = "Field `VID_SHADOW_PIN_REQ` writer - NA"]
pub type VidShadowPinReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn vid_shadow_en(&self) -> VidShadowEnR {
        VidShadowEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn vid_shadow_req(&self) -> VidShadowReqR {
        VidShadowReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn vid_shadow_pin_req(&self) -> VidShadowPinReqR {
        VidShadowPinReqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn vid_shadow_en(&mut self) -> VidShadowEnW<'_, VidShadowCtrlSpec> {
        VidShadowEnW::new(self, 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn vid_shadow_req(&mut self) -> VidShadowReqW<'_, VidShadowCtrlSpec> {
        VidShadowReqW::new(self, 8)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn vid_shadow_pin_req(&mut self) -> VidShadowPinReqW<'_, VidShadowCtrlSpec> {
        VidShadowPinReqW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`vid_shadow_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vid_shadow_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VidShadowCtrlSpec;
impl crate::RegisterSpec for VidShadowCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid_shadow_ctrl::R`](R) reader structure"]
impl crate::Readable for VidShadowCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vid_shadow_ctrl::W`](W) writer structure"]
impl crate::Writable for VidShadowCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VID_SHADOW_CTRL to value 0"]
impl crate::Resettable for VidShadowCtrlSpec {}
