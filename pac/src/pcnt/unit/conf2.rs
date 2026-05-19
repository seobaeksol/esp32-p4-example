#[doc = "Register `CONF2` reader"]
pub type R = crate::R<Conf2Spec>;
#[doc = "Register `CONF2` writer"]
pub type W = crate::W<Conf2Spec>;
#[doc = "Field `CNT_H_LIM` reader - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CntHLimR = crate::FieldReader<u16>;
#[doc = "Field `CNT_H_LIM` writer - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CntHLimW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_L_LIM` reader - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CntLLimR = crate::FieldReader<u16>;
#[doc = "Field `CNT_L_LIM` writer - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CntLLimW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_h_lim(&self) -> CntHLimR {
        CntHLimR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_l_lim(&self) -> CntLLimR {
        CntLLimR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_h_lim(&mut self) -> CntHLimW<'_, Conf2Spec> {
        CntHLimW::new(self, 0)
    }
    #[doc = "Bits 16:31 - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_l_lim(&mut self) -> CntLLimW<'_, Conf2Spec> {
        CntLLimW::new(self, 16)
    }
}
#[doc = "Configuration register 2 for unit 0\n\nYou can [`read`](crate::Reg::read) this register and get [`conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Conf2Spec;
impl crate::RegisterSpec for Conf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf2::R`](R) reader structure"]
impl crate::Readable for Conf2Spec {}
#[doc = "`write(|w| ..)` method takes [`conf2::W`](W) writer structure"]
impl crate::Writable for Conf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF2 to value 0"]
impl crate::Resettable for Conf2Spec {}
