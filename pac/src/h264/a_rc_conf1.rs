#[doc = "Register `A_RC_CONF1` reader"]
pub type R = crate::R<ARcConf1Spec>;
#[doc = "Register `A_RC_CONF1` writer"]
pub type W = crate::W<ARcConf1Spec>;
#[doc = "Field `A_CHROMA_DC_QP_DELTA` reader - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
pub type AChromaDcQpDeltaR = crate::FieldReader;
#[doc = "Field `A_CHROMA_DC_QP_DELTA` writer - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
pub type AChromaDcQpDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `A_CHROMA_QP_DELTA` reader - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
pub type AChromaQpDeltaR = crate::FieldReader;
#[doc = "Field `A_CHROMA_QP_DELTA` writer - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
pub type AChromaQpDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A_QP_MIN` reader - Configures video A allowed luma QP min value."]
pub type AQpMinR = crate::FieldReader;
#[doc = "Field `A_QP_MIN` writer - Configures video A allowed luma QP min value."]
pub type AQpMinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_QP_MAX` reader - Configures video A allowed luma QP max value."]
pub type AQpMaxR = crate::FieldReader;
#[doc = "Field `A_QP_MAX` writer - Configures video A allowed luma QP max value."]
pub type AQpMaxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `A_MAD_FRAME_PRED` reader - Configures vdieo A frame level predicted MB MAD value."]
pub type AMadFramePredR = crate::FieldReader<u16>;
#[doc = "Field `A_MAD_FRAME_PRED` writer - Configures vdieo A frame level predicted MB MAD value."]
pub type AMadFramePredW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2 - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
    #[inline(always)]
    pub fn a_chroma_dc_qp_delta(&self) -> AChromaDcQpDeltaR {
        AChromaDcQpDeltaR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
    #[inline(always)]
    pub fn a_chroma_qp_delta(&self) -> AChromaQpDeltaR {
        AChromaQpDeltaR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:12 - Configures video A allowed luma QP min value."]
    #[inline(always)]
    pub fn a_qp_min(&self) -> AQpMinR {
        AQpMinR::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 13:18 - Configures video A allowed luma QP max value."]
    #[inline(always)]
    pub fn a_qp_max(&self) -> AQpMaxR {
        AQpMaxR::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:30 - Configures vdieo A frame level predicted MB MAD value."]
    #[inline(always)]
    pub fn a_mad_frame_pred(&self) -> AMadFramePredR {
        AMadFramePredR::new(((self.bits >> 19) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures video A chroma DC QP offset based on Chroma QP. Chroma DC QP = Chroma QP(after map) + reg_chroma_dc_qp_delta."]
    #[inline(always)]
    pub fn a_chroma_dc_qp_delta(&mut self) -> AChromaDcQpDeltaW<'_, ARcConf1Spec> {
        AChromaDcQpDeltaW::new(self, 0)
    }
    #[doc = "Bits 3:6 - Configures video A chroma QP offset based on luma QP. Chroma QP(before map) = Luma QP + reg_chroma_qp_delta."]
    #[inline(always)]
    pub fn a_chroma_qp_delta(&mut self) -> AChromaQpDeltaW<'_, ARcConf1Spec> {
        AChromaQpDeltaW::new(self, 3)
    }
    #[doc = "Bits 7:12 - Configures video A allowed luma QP min value."]
    #[inline(always)]
    pub fn a_qp_min(&mut self) -> AQpMinW<'_, ARcConf1Spec> {
        AQpMinW::new(self, 7)
    }
    #[doc = "Bits 13:18 - Configures video A allowed luma QP max value."]
    #[inline(always)]
    pub fn a_qp_max(&mut self) -> AQpMaxW<'_, ARcConf1Spec> {
        AQpMaxW::new(self, 13)
    }
    #[doc = "Bits 19:30 - Configures vdieo A frame level predicted MB MAD value."]
    #[inline(always)]
    pub fn a_mad_frame_pred(&mut self) -> AMadFramePredW<'_, ARcConf1Spec> {
        AMadFramePredW::new(self, 19)
    }
}
#[doc = "Video A rate control configuration register1.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_rc_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_rc_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARcConf1Spec;
impl crate::RegisterSpec for ARcConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_rc_conf1::R`](R) reader structure"]
impl crate::Readable for ARcConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`a_rc_conf1::W`](W) writer structure"]
impl crate::Writable for ARcConf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A_RC_CONF1 to value 0"]
impl crate::Resettable for ARcConf1Spec {}
